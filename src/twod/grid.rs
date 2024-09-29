use std::{collections::HashMap, i32};

use super::*;

pub struct GridAlgorithm;

#[derive(Eq, Hash, PartialEq, Default, Clone)]
struct CellKey {
    pub x: i32,
    pub y: i32,
}

impl CellKey {
    fn new(point: &Point, radius: f32) -> Self {
        Self {
            x: (point.x / radius).floor() as i32,
            y: (point.y / radius).floor() as i32,
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

type Grid<'a> = HashMap<CellKey, Vec<&'a Point>>;

impl ClosestPairAlgorithm for GridAlgorithm {
    fn name(&self) -> &'static str {
        "grid"
    }
    fn execute<'a>(&self, points: &'a [Point]) -> ClosestPair<'a> {
        let mut closest_pair = ClosestPair::euclidean(&points[0], &points[1]);
        let mut grid = create_grid(&points[..=1], closest_pair.distance);

        for (i, point) in points[2..].iter().enumerate() {
            let i = i + 2;
            let key = CellKey::new(point, closest_pair.distance);
            let nearby_keys = key.nearbys();
            let current_pair = nearby_keys
                .iter()
                .filter_map(|key| grid.get(key))
                .flatten()
                .map(|p| ClosestPair::euclidean(&p, point))
                .min();
            if current_pair
                .as_ref()
                .is_some_and(|current_pair| current_pair < &closest_pair)
            {
                closest_pair = current_pair.unwrap();
                grid = create_grid(&points[..=i], closest_pair.distance);
            } else {
                grid.entry(key).or_default().push(point);
            }
        }

        return closest_pair;
    }

    fn drawings<'a>(&self, points: &'a [Point]) -> Vec<Vec<Drawing>> {
        let mut drawings = vec![];

        let mut closest_pair = ClosestPair::euclidean(&points[0], &points[1]);
        let mut grid = create_grid(&points[..=1], closest_pair.distance);

        for (i, point) in points[2..].iter().enumerate() {
            let i = i + 2;
            let key = CellKey::new(point, closest_pair.distance);
            let nearby_keys = key.nearbys();
            let current_pair = nearby_keys
                .iter()
                .filter_map(|key| grid.get(key))
                .flatten()
                .map(|p| ClosestPair::euclidean(&p, point))
                .min();

            {
                let mut current_drawing = vec![];
                let mut x = closest_pair.distance;
                while x < 1.0 {
                    current_drawing.push(Drawing::Line(
                        Point::new(x, 0.0),
                        Point::new(x, 1.0),
                        Color32::WHITE,
                    ));
                    current_drawing.push(Drawing::Line(
                        Point::new(0.0, x),
                        Point::new(1.0, x),
                        Color32::WHITE,
                    ));
                    x += closest_pair.distance;
                }
                for (index, point) in points[0..=i].iter().enumerate() {
                    if index == i {
                        current_drawing.push(Drawing::Point(point.clone(), Color32::RED));
                    } else {
                        current_drawing.push(Drawing::Point(point.clone(), Color32::WHITE));
                    }
                }
                drawings.push(current_drawing);
            }

            if current_pair
                .as_ref()
                .is_some_and(|current_pair| current_pair < &closest_pair)
            {
                closest_pair = current_pair.unwrap();
                grid = create_grid(&points[..=i], closest_pair.distance);
            } else {
                grid.entry(key).or_default().push(point);
            }
        }

        return drawings;
    }
}

fn create_grid(points: &[Point], radius: f32) -> Grid<'_> {
    let mut grid = Grid::new();
    for point in points.iter() {
        let key = CellKey::new(point, radius);
        grid.entry(key).or_default().push(point);
    }
    return grid;
}
