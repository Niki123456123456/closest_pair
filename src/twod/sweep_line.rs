use std::collections::BTreeSet;

use super::*;

pub struct SweepLine;

impl<T: Number> ClosestPairAlgorithm<T> for SweepLine {
    fn limit(&self,) -> usize {
        usize::MAX
     }
    fn name(&self) -> &'static str {
        "sweep line"
    }
    fn execute<'a>(&self, points: &'a [Point<T>]) -> ClosestPair<'a, T> {
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let mut closest_pair = ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]);
        let mut set: BTreeSet<&Point<T>> = BTreeSet::new();
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

            let lower_bound = Point::new( T::MIN, point.y - closest_pair.distance);
            let upper_bound = Point::new( T::MAX, point.y + closest_pair.distance);
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

    fn drawings<'a>(&self, points: &'a [Point<T>]) -> Vec<Vec<Drawing<T>>> {
        let mut drawings = vec![];
       
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let mut closest_pair = ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]);
        let mut set: BTreeSet<&Point<T>> = BTreeSet::new();
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

           
           

            let lower_bound = Point::new( T::MIN, point.y - closest_pair.distance);
            let upper_bound = Point::new( T::MAX, point.y + closest_pair.distance);

            let mut current_drawing = vec![];
            current_drawing.push(Drawing::Line(
                closest_pair.point_a.clone(),
                closest_pair.point_b.clone(),
                Color32::RED,
            ));
            current_drawing.push(Drawing::Line(
                Point::new(point.x, T::MIN),
                Point::new(point.x, T::MAX),
                Color32::GREEN,
            ));
            current_drawing.push(Drawing::Line(
                Point::new(points_sorted_x[j.min(points_sorted_x.len()-1)].x, T::MIN),
                Point::new(points_sorted_x[j.min(points_sorted_x.len()-1)].x, T::MAX),
                Color32::GREEN,
            ));
            current_drawing.push(Drawing::Line(
                Point::new(point.x, point.y - closest_pair.distance),
                Point::new(points_sorted_x[j.min(points_sorted_x.len()-1)].x, point.y - closest_pair.distance),
                Color32::GREEN,
            ));
            current_drawing.push(Drawing::Line(
                Point::new(point.x, point.y + closest_pair.distance),
                Point::new(points_sorted_x[j.min(points_sorted_x.len()-1)].x, point.y + closest_pair.distance),
                Color32::GREEN,
            ));
            for point in points.iter() {
                current_drawing.push(Drawing::Point(point.clone(), Color32::WHITE));
            }
            drawings.push(current_drawing);

            for point_b in set.range(lower_bound..=upper_bound) {
                let current_pair = ClosestPair::euclidean(point, point_b);
                if current_pair < closest_pair {
                    closest_pair = current_pair;
                }
            }
            
           

            set.insert(point);
        }
        

        return drawings;
    }
}
