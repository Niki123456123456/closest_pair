use std::usize;

use super::*;

pub struct DivideAndConquerPresortedY;

impl<T: Number> ClosestPairAlgorithm<T> for DivideAndConquerPresortedY {
    fn name(&self) -> &'static str {
        "divide and conquer presorted y"
    }
    fn limit(&self) -> usize {
        usize::MAX
    }
    fn execute<'a>(&self, points: &'a [Point<T>]) -> ClosestPair<'a, T> {
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        let mut points_sorted_y: Vec<_> = points.iter().collect();
        points_sorted_y.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        return closest_pair_recursive(&points_sorted_x, &points_sorted_y);
    }

    fn drawings<'a>(&self, points: &'a [Point<T>]) -> Vec<Vec<Drawing<T>>> {
       return vec![];
    }
}

fn closest_pair_recursive<'a, T: Number>(points_sorted_x: &[&'a Point<T>], points_sorted_y: &[&'a Point<T>]) -> ClosestPair<'a, T> {
    if points_sorted_x.len() == 2 {
        return ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]);
    } else if points_sorted_x.len() == 3 {
        return [
            ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]),
            ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[2]),
            ClosestPair::euclidean(&points_sorted_x[1], &points_sorted_x[2]),
        ]
        .into_iter()
        .min()
        .unwrap();
    }

    let mid = points_sorted_x.len() / 2;
    let mid_point = points_sorted_x[mid];

    let (left_points, right_points) = points_sorted_x.split_at(mid);
    let mut left_points_y = vec![];
    let mut right_points_y = vec![];
    for &point in points_sorted_y.iter() {
        let push_left = point.x < mid_point.x || (point.x == mid_point.x && point.y < mid_point.y);
        if push_left {
            left_points_y.push(point);
        } else {
            right_points_y.push(point);
        }
    }

    let left_pair = closest_pair_recursive(left_points, &left_points_y);
    let right_pair = closest_pair_recursive(right_points, &right_points_y);
    let closest_pair = left_pair.min(right_pair);

    let strip: Vec<_> = points_sorted_y
        .iter()
        .map(|x| *x)
        .filter(|p| (p.x - mid_point.x).abs() < closest_pair.distance)
        .collect();

    return strip_closest(&strip, closest_pair);
}

fn strip_closest<'a, T: Number>(
    points: &[&'a Point<T>],
    mut closest_pair: ClosestPair<'a, T>,
) -> ClosestPair<'a, T> {
    for (i, &point_a) in points.iter().enumerate() {
        for &point_b in &points[i + 1..] {
            if (point_a.y - point_b.y).abs() > closest_pair.distance {
                break;
            }
            let current_pair = ClosestPair::euclidean(point_a, point_b);
            if current_pair < closest_pair {
                closest_pair = current_pair;
            }
        }
    }

    return closest_pair;
}
