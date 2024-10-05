use std::{collections::HashMap, i32};

use super::*;

pub struct GridAlgorithmConst;

type Grid<'a, const DIMENSION: usize> = HashMap<CellKey<DIMENSION>, Cell<&'a Point<DIMENSION>>>;

#[derive(Eq, Hash, PartialEq, Clone)]
struct CellKey<const DIMENSION: usize> {
    pub coordinates : [i32; DIMENSION]
}

impl<const DIMENSION: usize> CellKey<DIMENSION> {
    fn new(point: &Point<DIMENSION>, radius: f32) -> Self {
        let mut coordinates  = [0; DIMENSION];
        for (i,coordinate) in point.coordinates.iter().enumerate() {
            coordinates[i] = (coordinate / radius).floor() as i32;
        }
        Self {
            coordinates
        }
    }

    fn nearbys(&self) -> Vec<Self> {
        let radius = 1;
        let mut nearbys = vec![];
        let mut i = 0;
        return nearbys;
    }
}

struct Cell<T> {
    points: [T; 4],
    size: u8,
}

impl<T> Cell<T> {
    fn push(& mut self, point: T) {
        self.points[self.size as usize] = point;
        self.size += 1;
    }
}

impl ClosestPairAlgorithm for GridAlgorithmConst {
    fn execute<'a, const DIMENSION: usize>(&self, points: &'a[Point<DIMENSION>]) -> ClosestPair<'a, DIMENSION> {
        let total_len = points.len();
        let mut closest_pair = ClosestPair::euclidean(&points[0], &points[1]);
        let mut grid = create_grid(&points[..=1], closest_pair.distance, total_len);

        for (i, point) in points.iter().enumerate().skip(2) {
            let key = CellKey::new(point, closest_pair.distance);
            let nearby_keys = key.nearbys();
            let current_pair = nearby_keys
                .iter()
                .filter_map(|key| grid.get(key)).map(|x|x.points.iter().take(x.size as usize))
                .flatten()
                .map(|p| ClosestPair::euclidean(&p, point))
                .min();
            if current_pair
                .as_ref()
                .is_some_and(|current_pair| current_pair < &closest_pair)
            {
                closest_pair = current_pair.unwrap();
                grid = create_grid(&points[..=i], closest_pair.distance, total_len);
            } else {
                grid.entry(key).or_insert(Cell { points: [const { &Point { coordinates: [0.0; DIMENSION] } }; 4], size: 0 }).push(point);
            }
        }

        return closest_pair;
    }
}

fn create_grid<const DIMENSION: usize>(points: &[Point<DIMENSION>], radius: f32, total_len : usize) -> Grid<'_, DIMENSION> {
    let mut grid = Grid::with_capacity(total_len);
    for point in points.iter() {
        let key = CellKey::new(point, radius);
        grid.entry(key).or_insert(Cell { points: [const { &Point { coordinates: [0.0; DIMENSION] } }; 4], size: 0 }).push(point);
    }
    return grid;
}
