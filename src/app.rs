use std::result;

use egui::{vec2, Color32};

use crate::twod::*;

pub struct AlgorithmResult {
    name: &'static str,
    point_a: Point,
    point_b: Point,
    distance: f32,
    duration: std::time::Duration,
    drawings : Vec<Vec<Drawing>>,
    drawing_step : usize,
}

pub struct App {
    rng: rand::prelude::ThreadRng,
    points: Vec<Point>,
    results: Vec<AlgorithmResult>,
    len : usize,
}

fn generate_points(
    rng: &mut rand::prelude::ThreadRng,
    len: usize,
) -> (Vec<Point>, Vec<AlgorithmResult>) {
    let mut points = vec![];
    for _ in 0..len {
        points.push(Point::ran(rng));
    }
    let algorithms: Vec<Box<dyn ClosestPairAlgorithm>> =
        vec![Box::new(BruteForce), Box::new(GridAlgorithm)];

    let mut results: Vec<AlgorithmResult> = vec![];

    for algorithm in algorithms.iter() {
        let start = std::time::Instant::now();

        let clostet_pair = algorithm.execute(&points);

        let duration: std::time::Duration = start.elapsed();

        let drawings = algorithm.drawings(&points);
        results.push(AlgorithmResult {
            name: algorithm.name(),
            point_a: clostet_pair.point_a.clone(),
            point_b: clostet_pair.point_b.clone(),
            distance: clostet_pair.distance,
            duration,
            drawings,
            drawing_step: 0,
        });
    }

    return (points, results);
}

impl Default for App {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let len = 10;
        let (points, results) = generate_points(&mut rng, len);

        Self {
            rng,
            points,
            results,
            len,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
                
                let start_pos = ui.next_widget_position();
                let height = ui.available_height();
                let width = ui.available_width();
                let size = if height < width { height } else { width } * 0.25;
                for point in self.points.iter() {
                    let pos = start_pos + size * vec2(point.x, point.y);
                    ui.painter().circle_filled(pos, 1.0, Color32::WHITE);
                }
                ui.allocate_at_least(vec2(size, size), egui::Sense::click());
    
                slider(ui, &mut self.len, 10, 5000, true, "number of points");
                if ui.button("regenerate").clicked() {
                    (self.points, self.results) = generate_points(&mut self.rng, self.len);
                }
    
                for result in self.results.iter() {
                    ui.label(format!("{}:", result.name));
                    ui.label(format!("distance: {}", result.distance));
                    ui.label(format!("took: {}ms", result.duration.as_micros()));
                    ui.label(format!(
                        "point_a: {} {}",
                        result.point_a.x, result.point_a.y
                    ));
                    ui.label(format!(
                        "point_b: {} {}",
                        result.point_b.x, result.point_b.y
                    ));
                }
                for result in self.results.iter_mut() {
                    if result.drawings.is_empty() {
                        continue;
                    }
                    slider(ui, &mut result.drawing_step, 0, result.drawings.len()-1, false, "step");
                    let drawings = &result.drawings[result.drawing_step];
                    let start_pos = ui.next_widget_position();
                    for drawing in drawings.iter() {
                        match drawing {
                            Drawing::Point(point, color32) => {
                                let pos = start_pos + size * vec2(point.x, point.y);
                                ui.painter().circle_filled(pos, 1.0, *color32);
                            },
                            Drawing::Line(point, point1, color32) => {
                                let pos = start_pos + size * vec2(point.x, point.y);
                                let pos1 = start_pos + size * vec2(point1.x, point1.y);
                                ui.painter().line_segment([pos, pos1], egui::epaint::PathStroke::new(0.5, *color32));
                            },
                        }
                    }
                    ui.allocate_at_least(vec2(size, size), egui::Sense::click());
                 }
            
            });
    }
}


fn slider(ui: &mut egui::Ui, value: &mut usize, min_value : usize, max_value : usize, logarithmic : bool, text : &str) {
    let mut value_f64 = *value as f64;
    ui.add(egui::Slider::new(&mut value_f64, min_value as f64..=max_value as f64).logarithmic(logarithmic).text(text));
    *value = value_f64 as usize;
}