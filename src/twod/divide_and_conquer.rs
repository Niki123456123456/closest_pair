use super::*;

pub struct DivideAndConquer;

impl ClosestPairAlgorithm for DivideAndConquer {
    fn name(&self) -> &'static str {
        "divide and conquer"
    }
    fn execute<'a>(&self, points: &'a [Point]) -> ClosestPair<'a> {
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        return closest_pair_recursive(&points_sorted_x);
    }

    fn drawings<'a>(&self, points: &'a [Point]) -> Vec<Vec<Drawing>> {
        let mut drawings = vec![];
        return drawings;
    }
}

fn closest_pair_recursive<'a>(points_sorted_x: &[&'a Point]) -> ClosestPair<'a> {
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

    let (left_x, right_x) = points_sorted_x.split_at(mid);

    let dl = closest_pair_recursive(left_x);
    let dr = closest_pair_recursive(right_x);
    let d = dl.min(dr);

    let strip: Vec<_> = points_sorted_x
        .iter()
        .map(|x| *x)
        .filter(|p| (p.x - mid_point.x).abs() < d.distance)
        .collect();

    return strip_closest(&strip, d);
}

fn strip_closest<'a>(points: &[&'a Point], mut closest_pair: ClosestPair<'a>) -> ClosestPair<'a> {
    let mut points_sorted_y: Vec<_> = points.iter().collect();
    points_sorted_y.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    for (i,&&point_a) in points_sorted_y.iter().enumerate() {
        for &point_b in &points_sorted_y[i + 1..] {
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
