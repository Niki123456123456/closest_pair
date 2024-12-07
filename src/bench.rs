use crate::{
    twod, BruteForce, ClosestPairAlgorithm, DivideAndConquer, DivideAndConquerPresortedY,
    GridAlgorithm, GridAlgorithmConst, Point, SweepLine,
};
use egui::mutex::Mutex;
use rand::Rng;
use rand::{distributions::Standard, prelude::Distribution};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};
use std::fmt::Debug;
use std::sync::Arc;
#[derive(PartialEq, Clone)]
pub enum Number {
    F32,
    F64,
}

#[derive(Clone)]
pub struct Settings {
    pub algorithms: Vec<(&'static str, bool)>,
    pub generations: Vec<(PointDistribution, bool)>,
    pub max_size: usize,
    pub repeat: usize,
    pub number: Number,
}
pub struct Result {
    pub name: String,
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
                generations: PointDistribution::iter().map(|x| (x, x == PointDistribution::Evenly)).collect(),
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
            for (name, enabled) in &mut self.settings.generations {
                ui.checkbox(enabled, name.as_ref());
            }
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.settings.max_size, 2..=30).text("size"));
                ui.label(print_thousands(2_i64.pow(self.settings.max_size as u32)))
            });
            ui.add(egui::Slider::new(&mut self.settings.repeat, 1..=100).text("repeat"));

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.settings.number, Number::F32, "f32");
                ui.selectable_value(&mut self.settings.number, Number::F64, "f64");
            });

            ui.horizontal(|ui| {
                if ui.button("run").clicked() {
                    self.results = Arc::new(Mutex::new((vec![], false)));
                    let results = self.results.clone();
                    let settings = self.settings.clone();

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        execute(move|| {
                            match settings.number {
                                Number::F32 => {
                                    bench::<f32>(&settings, results);
                                }
                                Number::F64 => {
                                    bench::<f64>(&settings, results);
                                }
                            }
                        });
                    }
                    
                    #[cfg(target_arch = "wasm32")]
                    {
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
                    }
    
                    
                }
            
                {
                    let results = self.results.lock();
                    if results.1 == false {
                        ui.spinner();
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("copy results").clicked() {
                        let results = self.results.lock();
                        let mut lines = vec![];
                        let mut i = 0;
                        let mut titles : Vec<_> = results.0.iter().map(|x|x.name.to_string()).collect();
                        titles.insert(0, "number of points".to_string());
                        lines.push(titles.join("	"));
                        loop {
                            let mut cells = vec![];
                            let mut break_loop = true;
                            for (i2,result) in results.0.iter().enumerate(){
                                if let Some(x) = result.observations.get(i) {
                                    if i2 == 0 {
                                        cells.push(x.1.to_string());
                                    }
                                    cells.push(x.2.as_nanos().to_string());
                                    break_loop = false;
                                } else {
                                    cells.push("".to_string());
                                }
                            }
                            lines.push(cells.join("	"));
                            if break_loop {
                                break;
                            }
                            i += 1;
                        }
                        let mut ctx: clipboard::ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
                        clipboard::ClipboardProvider::set_contents(&mut ctx, lines.join("\n")).unwrap();
                    }
                }
                
            });

            let mut lines = vec![];
            {
                let results = self.results.lock();
                let line : Vec<_> =  results.0.iter().map(|x| {
                    let circle_points: egui_plot::PlotPoints = x.observations.iter().map(|x| [x.1 as f64, x.2.as_micros() as f64 / x.0 as f64]).collect();
                    return egui_plot::Line::new(circle_points)
                    .name(x.name.to_string());
                }).collect();
                lines = line;
            }

            egui_plot::Plot::new("bench").label_formatter(|name, value| {
                let power = (value.x.floor() as u32);
                let count = 2_usize.pow(power);
                let duration = value.y;
                return format!("{name}\nnumber of points: 2^{power} = {count} \nduration per point: {duration:.2}ms");
            }).legend(egui_plot::Legend::default()).show(ui, |plot_ui| {
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
    
    for size in 2..=settings.max_size {
        let len = 2_usize.pow(size as u32);
        for setting in settings.generations.iter().filter(|x| x.1) {
            let points = generate_points::<T>(len, setting.0, &mut rng);

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
                    let name = format!("{}_{:?}", algo.name() , setting.0);
                    let mut results = results.lock();
                    if results.0.iter().enumerate().filter(|x|x.1.name == name).next().is_none() {
                        results.0.push(Result {
                            name: name.to_string(),
                            observations: vec![],
                        });
                    }
                    if let Some(results) = results.0.iter_mut().filter(|x|x.name == name).next() {
                        results.observations.push((len,size, duration));
                    }
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
#[derive(PartialEq, Clone, Copy, EnumIter, EnumString, AsRefStr, Debug)]
pub enum PointDistribution {
    Horizontal,
    Vertical,
    Evenly,
    IncreasingDistance,
    DecreasingDistance,
}

fn generate_points<T: twod::Number>(len: usize, distribution : PointDistribution, rng: &mut rand::prelude::ThreadRng) -> Vec<Point<T>>
where
    Standard: Distribution<T>,
{
    let mut points: Vec<Point<T>> = vec![];
    for i in 0..len {
        let p = match distribution {
            PointDistribution::Horizontal => Point{ x: rng.gen(), y: T::CENTER },
            PointDistribution::Vertical => Point{ x: T::CENTER, y: rng.gen() },
            PointDistribution::Evenly => Point::ran(rng),
            PointDistribution::IncreasingDistance =>Point{ x: T::CENTER, y:  T::MAX - (T::MAX / T::from_usize(len - i))  }, 
            PointDistribution::DecreasingDistance => Point{ x: T::CENTER, y:  T::MAX / T::from_usize(i + 1)  },
        };
        points.push(p);
    }
    return points;
}



#[cfg(not(target_arch = "wasm32"))]
fn execute<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}

#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
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
