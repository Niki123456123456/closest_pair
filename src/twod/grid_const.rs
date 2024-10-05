use std::{collections::HashMap, i32};

use super::*;

pub struct GridAlgorithmConst;

type Grid<'a, T> = HashMap<CellKey, Cell<&'a Point<T>>>;

#[derive(Eq, Hash, PartialEq, Default, Clone)]
struct CellKey {
    pub x: i64,
    pub y: i64,
}

impl CellKey {
    fn new<T : Number>(point: &Point<T>, radius: T) -> Self {
        Self {
            x: (point.x / radius).floor().as_i64(),
            y: (point.y / radius).floor().as_i64(),
        }
    }

    fn nearbys(&self) -> [Self; 9] {
        let radius = 1;
        let mut nearbys = [const { Self { x: 0, y: 0 } }; 9];
        let mut i = 0;
        for x in self.x.wrapping_sub(radius)..=self.x.wrapping_add(radius) {
            for y in self.y.wrapping_sub(radius)..=self.y.wrapping_add(radius) {
                nearbys[i] = CellKey { x, y };
                i += 1;
            }
        }
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

impl<T : Number + 'static> ClosestPairAlgorithm<T> for GridAlgorithmConst {
    fn name(&self) -> &'static str {
        "grid const"
    }
    fn limit(&self) -> usize {
        usize::MAX
    }
    fn execute<'a>(&self, points: &'a [Point<T>]) -> ClosestPair<'a, T> {
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
                grid.entry(key).or_insert(Cell { points: T::default_points(), size: 0 }).push(point);
            }
        }

        return closest_pair;
    }

    fn drawings<'a>(&self, _points: &'a [Point<T>]) -> Vec<Vec<Drawing<T>>> {
        vec![]
    }
}

fn create_grid<T : Number + 'static>(points: &[Point<T>], radius: T, total_len : usize) -> Grid<'_, T> {
    let mut grid = Grid::with_capacity(total_len);
    for point in points.iter() {
        let key = CellKey::new(point, radius);
        grid.entry(key).or_insert(Cell { points: T::default_points(), size: 0 }).push(point);
    }
    return grid;
}
