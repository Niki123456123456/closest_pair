use rand::Rng;
mod brute_force;
mod grid_const;

pub trait ClosestPairAlgorithm {
    fn execute<'a, const DIMENSION: usize>(&self, points: &'a[Point<DIMENSION>]) -> ClosestPair<'a, DIMENSION>;
}
#[derive(Debug, Clone)]
pub struct ClosestPair<'a, const DIMENSION: usize> {
    pub point_a: &'a Point<DIMENSION>,
    pub point_b: &'a Point<DIMENSION>,
    pub distance: f32,
}

impl<'a, const DIMENSION: usize> ClosestPair<'a,DIMENSION> {
    #[inline(always)]
    fn euclidean(point_a: &'a Point<DIMENSION>, point_b: &'a Point<DIMENSION>) -> Self {
        Self {
            point_a,
            point_b,
            distance: euclidean_distance(point_a, point_b),
        }
    }
}

impl<const DIMENSION: usize> Eq for ClosestPair<'_, DIMENSION> {}

impl<const DIMENSION: usize> PartialEq for ClosestPair<'_, DIMENSION> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<const DIMENSION: usize> PartialOrd for ClosestPair<'_, DIMENSION> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<const DIMENSION: usize> Ord for ClosestPair<'_, DIMENSION> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point<const DIMENSION : usize> {
    pub coordinates : [f32; DIMENSION]
}

impl<const DIMENSION : usize> Eq for Point<DIMENSION> {
    
}

impl<const DIMENSION : usize> Ord for Point<DIMENSION> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       self.coordinates[1].partial_cmp(&other.coordinates[1]).unwrap()
    }
}

impl<const DIMENSION : usize> Point<DIMENSION> {
    pub fn new(coordinates : [f32; DIMENSION]) -> Self {
        Self {
            coordinates
        }
    }

    pub fn ran(rng: &mut rand::prelude::ThreadRng) -> Self {
        let mut coordinates = [0.0; DIMENSION];
        for i in 0..DIMENSION {
            coordinates[i] = rng.gen()
        }
        Self {
            coordinates,
        }
    }
}

#[inline(always)]
pub fn euclidean_distance<const DIMENSION : usize>(point_a: &Point<DIMENSION>, point_b: &Point<DIMENSION>) -> f32 {
    point_a.coordinates.iter().zip(point_b.coordinates.iter()).map(|(a,b)| (a - b).powi(2)).sum::<f32>().sqrt()
}
