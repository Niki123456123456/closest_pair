use crate::{
    twod, BruteForce, ClosestPairAlgorithm, DivideAndConquer, DivideAndConquerPresortedY,
    GridAlgorithm, GridAlgorithmConst, Point, SweepLine,
};
use egui::mutex::Mutex;
use rand::{distributions::Standard, prelude::Distribution};
use std::fmt::Debug;
#[derive(PartialEq, Clone)]
pub enum Number {
    F32,
    F64,
}

#[derive(Clone)]
pub struct Settings {
    pub algorithms: Vec<(&'static str, bool)>,
    pub max_size: usize,
    pub repeat: usize,
    pub number: Number,
}
pub struct Result {
    pub name: &'static str,
    pub observations: Vec<(usize,usize, std::time::Duration)>,
}
pub struct Bench {
    pub settings: Settings,
    pub results: Arc<Mutex<(Vec<Result>, bool)>>,
}

impl Bench {
    pub fn new() -> Self {
        let algorithms: Vec<Box<dyn ClosestPairAlgorithm<f32>>> = vec![
            Box::new(BruteForce),
            Box::new(DivideAndConquerPresortedY),
            Box::new(DivideAndConquer),
            Box::new(SweepLine),
            Box::new(GridAlgorithm),
            Box::new(GridAlgorithmConst),
        ];
        Self {
            settings: Settings {
                algorithms: algorithms.iter().map(|x| (x.name(), true)).collect(),
                max_size: 10,
                number: Number::F32,
                repeat: 1,
            },
            results: Arc::new(Mutex::new((vec![], true))),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("bench").show(ctx, |ui| {
            for (name, enabled) in &mut self.settings.algorithms {
                ui.checkbox(enabled, *name);
            }
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.settings.max_size, 2..=25).text("size"));
                ui.label(print_thousands(2_i64.pow(self.settings.max_size as u32)))
            });
            ui.add(egui::Slider::new(&mut self.settings.repeat, 1..=10).text("repeat"));

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.settings.number, Number::F32, "f32");
                ui.selectable_value(&mut self.settings.number, Number::F64, "f64");
            });

            ui.horizontal(|ui| {
                if ui.button("run").clicked() {
                    self.results = Arc::new(Mutex::new((vec![], false)));
                    let results = self.results.clone();
                    let settings = self.settings.clone();
                    execute(async move {
                        match settings.number {
                            Number::F32 => {
                                bench::<f32>(&settings, results);
                            }
                            Number::F64 => {
                                bench::<f64>(&settings, results);
                            }
                        }
                    });
    
                    {
                        let results = self.results.lock();
                        if results.1 == false {
                            ui.spinner();
                        }
                    }
                }
            });

            let mut lines = vec![];
            {
                let results = self.results.lock();
                let line : Vec<_> =  results.0.iter().map(|x| {
                    let circle_points: egui_plot::PlotPoints = x.observations.iter().map(|x| [x.1 as f64, x.2.as_micros() as f64 / x.0 as f64]).collect();
                    return egui_plot::Line::new(circle_points)
                    .name(x.name);
                }).collect();
                lines = line;
            }

            egui_plot::Plot::new("bench").legend(egui_plot::Legend::default()).show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line);
                }
            });
        });
    }
}

fn bench<T: twod::Number + Debug + 'static>(
    settings: &Settings,
    results: Arc<Mutex<(Vec<Result>, bool)>>,
) where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();

    let mut algorithms: Vec<Box<dyn ClosestPairAlgorithm<T>>> = vec![
        Box::new(BruteForce),
        Box::new(DivideAndConquerPresortedY),
        Box::new(DivideAndConquer),
        Box::new(SweepLine),
        Box::new(GridAlgorithm),
        Box::new(GridAlgorithmConst),
    ];
    algorithms = algorithms
        .into_iter()
        .filter(|x| settings.algorithms.iter().any(|a| a.0 == x.name() && a.1))
        .collect();
    {
        let mut results = results.lock();
        for algo in algorithms.iter() {
            results.0.push(Result {
                name: algo.name(),
                observations: vec![],
            });
        }
    }

    for size in 2..=settings.max_size {
        let len = 2_usize.pow(size as u32);
        let points = generate_points::<T>(len, &mut rng);

        for (i, algo) in algorithms.iter().enumerate() {
            let mut durations = vec![];
            let mut distance = T::MIN;
            for _ in 0..settings.repeat {
                let start = web_time::Instant::now();
                let result = algo.execute(&points);
                let duration: std::time::Duration = start.elapsed();
                distance = result.distance;
                durations.push(duration);
            }
            let duration: std::time::Duration =
                durations.iter().sum::<std::time::Duration>() / settings.repeat as u32;

            {
                let mut results = results.lock();
                if let Some(results) = results.0.get_mut(i) {
                    results.observations.push((len,size, duration));
                }
            }
        }
    }

    {
        let mut results = results.lock();
        results.1 = true;
        print!("ready");
    }
}

fn generate_points<T: twod::Number>(len: usize, rng: &mut rand::prelude::ThreadRng) -> Vec<Point<T>>
where
    Standard: Distribution<T>,
{
    let mut points: Vec<Point<T>> = vec![];
    for _ in 0..len {
        points.push(Point::ran(rng));
    }
    return points;
}

use std::{future::Future, sync::Arc};

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

fn print_thousands(i: impl ToString) -> String {
    i.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<core::result::Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}
