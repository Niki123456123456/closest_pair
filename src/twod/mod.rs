
use egui::Color32;
use rand::Rng;

mod brute_force;
mod divide_and_conquer;
mod grid;

pub use brute_force::BruteForce;
pub use grid::GridAlgorithm;

pub trait ClosestPairAlgorithm {
    fn name(&self,) -> &'static str;
    fn execute<'a>(&self, points: &'a[Point]) -> ClosestPair<'a>;
    fn drawings<'a>(&self, points: &'a[Point]) -> Vec<Vec<Drawing>>;
}
#[derive(Debug, Clone)]
pub struct ClosestPair<'a> {
    pub point_a: &'a Point,
    pub point_b: &'a Point,
    pub distance: f32,
}

impl<'a> ClosestPair<'a> {
    fn euclidean(point_a: &'a Point, point_b: &'a Point) -> Self {
        Self {
            point_a,
            point_b,
            distance: euclidean_distance(point_a, point_b),
        }
    }
}

impl Eq for ClosestPair<'_> {}

impl PartialEq for ClosestPair<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for ClosestPair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for ClosestPair<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn ran(rng: &mut rand::prelude::ThreadRng) -> Self {
        Self {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

pub fn euclidean_distance(point_a: &Point, point_b: &Point) -> f32 {
    ((point_a.x - point_b.x).powi(2) + (point_a.y - point_b.y).powi(2)).sqrt()
}


pub enum Drawing {
    Point(Point, Color32),
    Line(Point, Point, Color32)
}