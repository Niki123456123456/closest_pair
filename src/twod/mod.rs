use std::ops::{Add, Div, Sub};

use egui::Color32;
use rand::{distributions::Standard, prelude::Distribution, Rng};

mod brute_force;
mod divide_and_conquer;
mod grid;
mod grid_const;
mod sweep_line;

pub use brute_force::BruteForce;
pub use divide_and_conquer::DivideAndConquer;
pub use grid::GridAlgorithm;
pub use grid_const::GridAlgorithmConst;
pub use sweep_line::SweepLine;

pub trait ClosestPairAlgorithm<T: Number> {
    fn limit(&self) -> usize;
    fn name(&self) -> &'static str;
    fn execute<'a>(&self, points: &'a [Point<T>]) -> ClosestPair<'a, T>;
    fn drawings<'a>(&self, points: &'a [Point<T>]) -> Vec<Vec<Drawing<T>>>;
}
#[derive(Debug, Clone)]
pub struct ClosestPair<'a, T: Number> {
    pub point_a: &'a Point<T>,
    pub point_b: &'a Point<T>,
    pub distance: T,
}

impl<'a, T: Number> ClosestPair<'a, T> {
    #[inline(always)]
    fn euclidean(point_a: &'a Point<T>, point_b: &'a Point<T>) -> Self {
        Self {
            point_a,
            point_b,
            distance: euclidean_distance(point_a, point_b),
        }
    }
}

impl<T: Number> Eq for ClosestPair<'_, T> {}

impl<T: Number> PartialEq for ClosestPair<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T: Number> PartialOrd for ClosestPair<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<T: Number> Ord for ClosestPair<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: Number> Eq for Point<T> {}

impl<T: Number> Ord for Point<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.partial_cmp(&other.y).unwrap()
    }
}

impl<T: Number> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

}

impl<T: Number> Point<T> where Standard: Distribution<T> {

    pub fn ran(rng: &mut rand::prelude::ThreadRng) -> Self {
        Self {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

#[inline]
pub fn euclidean_distance<T: Number>(point_a: &Point<T>, point_b: &Point<T>) -> T {
    ((point_a.x - point_b.x).powi(2) + (point_a.y - point_b.y).powi(2)).sqrt()
}

pub trait Number: Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + PartialOrd + Sized + Copy + Default {
    const MIN: Self;
    const MAX: Self;
   
    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn floor(self) -> Self;
    fn as_i32(self) -> i32;
    fn default_points() -> [&'static Point<Self>; 4];
}

impl Number for f32 {
    const MIN: f32 = 0.0;
    const MAX: f32 = 1.0;
    #[inline]
    fn powi(self, n: i32) -> Self {
        f32::powi(self, n)
    }
    #[inline]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    #[inline]
    fn abs(self) -> Self {
        f32::abs(self)
    }
    #[inline]
    fn floor(self) -> Self {
        f32::floor(self)
    }
    #[inline]
    fn as_i32(self) -> i32 {
        self as i32
    }
    #[inline]
    fn default_points() -> [&'static Point<Self>; 4] {
        [const { &Point { x: 0.0, y: 0.0 } }; 4]
    }
    
    

   
}

pub enum Drawing<T: Number> {
    Point(Point<T>, Color32),
    Line(Point<T>, Point<T>, Color32),
}
