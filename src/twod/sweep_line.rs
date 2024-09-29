use core::f32;
use std::collections::BTreeSet;

use super::*;

pub struct SweepLine;

impl ClosestPairAlgorithm for SweepLine {
    fn limit(&self,) -> usize {
        10_000_000
     }
    fn name(&self) -> &'static str {
        "sweep line"
    }
    fn execute<'a>(&self, points: &'a [Point]) -> ClosestPair<'a> {
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let mut closest_pair = ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]);
        let mut set: BTreeSet<&Point> = BTreeSet::new();
        set.insert(&points_sorted_x[0]);
        set.insert(&points_sorted_x[1]);

        let mut j = 0;
        for (i,&point) in points_sorted_x.iter().enumerate().skip(2) {
            loop {
                if j >= i {
                    break;
                }
                if (point.x - points_sorted_x[j].x).abs() < closest_pair.distance {
                    break;
                }
                set.remove(&points_sorted_x[j]);
                j += 1;
            }

            let lower_bound = Point::new( f32::MIN, point.y - closest_pair.distance);
            let upper_bound = Point::new( f32::MAX, point.y + closest_pair.distance);
            //println!("len {}", set.len());
            for point_b in set.range(lower_bound..=upper_bound) {
                let current_pair = ClosestPair::euclidean(point, point_b);
                if current_pair < closest_pair {
                    closest_pair = current_pair;
                }
            }
            set.insert(point);
        }
        
        return closest_pair;
    }

    fn drawings<'a>(&self, points: &'a [Point]) -> Vec<Vec<Drawing>> {
        let mut drawings = vec![];
       

        return drawings;
    }
}
