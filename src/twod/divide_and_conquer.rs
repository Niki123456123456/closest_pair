use super::*;

pub struct DivideAndConquer;

impl<T: Number> ClosestPairAlgorithm<T> for DivideAndConquer {
    fn name(&self) -> &'static str {
        "divide and conquer"
    }
    fn execute<'a>(&self, points: &'a [Point<T>]) -> ClosestPair<'a,T> {
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        return closest_pair_recursive(&points_sorted_x);
    }

    fn drawings<'a>(&self, points: &'a [Point<T>]) -> Vec<Vec<Drawing<T>>> {
        let mut points_sorted_x: Vec<_> = points.iter().collect();
        points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        return closest_pair_recursive_draw(&points_sorted_x).1;
    }
    fn limit(&self,) -> usize {
        10_000_000
     }
}

fn closest_pair_recursive_draw<'a, T: Number>(points_sorted_x: &[&'a Point<T>]) -> (ClosestPair<'a, T>, Vec<Vec<Drawing<T>>>) {
    if points_sorted_x.len() == 2 {
        return (ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]), vec![]);
    } else if points_sorted_x.len() == 3 {
        return ([
            ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[1]),
            ClosestPair::euclidean(&points_sorted_x[0], &points_sorted_x[2]),
            ClosestPair::euclidean(&points_sorted_x[1], &points_sorted_x[2]),
        ]
        .into_iter()
        .min()
        .unwrap(), vec![]);
    }

    let mut drawings = vec![];
    let mid = points_sorted_x.len() / 2;
    let mid_point = points_sorted_x[mid];

    let (left_x, right_x) = points_sorted_x.split_at(mid);

    let (dl,mut left_drawings) = closest_pair_recursive_draw(left_x);
    let (dr,mut right_drawings) = closest_pair_recursive_draw(right_x);

    drawings.append(&mut left_drawings);
    drawings.append(&mut right_drawings);
    let mut current_drawing = vec![];
    for &point in points_sorted_x {
        current_drawing.push(Drawing::Point(point.clone(), Color32::WHITE));
    }
    current_drawing.push(Drawing::Line(
        dl.point_a.clone(),
        dl.point_b.clone(),
        Color32::RED,
    ));
    current_drawing.push(Drawing::Line(
        dr.point_a.clone(),
        dr.point_b.clone(),
        Color32::RED,
    ));
    current_drawing.push(Drawing::Line(
        Point::new(mid_point.x, T::MIN),
        Point::new(mid_point.x, T::MAX),
        Color32::GREEN,
    ));
    current_drawing.push(Drawing::Line(
        Point::new(points_sorted_x[0].x, T::MIN),
        Point::new(points_sorted_x[0].x, T::MAX),
        Color32::GREEN,
    ));
    current_drawing.push(Drawing::Line(
        Point::new(points_sorted_x[points_sorted_x.len()-1].x, T::MIN),
        Point::new(points_sorted_x[points_sorted_x.len()-1].x, T::MAX),
        Color32::GREEN,
    ));

    drawings.push(current_drawing);

    let d = dl.min(dr);
    


    let strip: Vec<_> = points_sorted_x
        .iter()
        .map(|x| *x)
        .filter(|p| (p.x - mid_point.x).abs() < d.distance)
        .collect();

    let closest = strip_closest(&strip, d);
    return (closest, drawings);
}


fn closest_pair_recursive<'a, T : Number>(points_sorted_x: &[&'a Point<T>]) -> ClosestPair<'a,T> {
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

fn strip_closest<'a, T :Number>(points: &[&'a Point<T>], mut closest_pair: ClosestPair<'a,T>) -> ClosestPair<'a, T> {
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
