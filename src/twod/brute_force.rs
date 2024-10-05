use super::*;

pub struct BruteForce;

impl<T: Number> ClosestPairAlgorithm<T> for BruteForce {
    fn limit(&self,) -> usize {
        10_000
     }
    fn name(&self) -> &'static str {
        "brute force"
    }
    fn execute<'a>(&self, points: &'a [Point<T>]) -> ClosestPair<'a, T> {
        let mut closest_pair = ClosestPair::euclidean(&points[0], &points[1]);
        for (i, point_a) in points.iter().enumerate() {
            for point_b in &points[i + 1..] {
                let current_pair = ClosestPair::euclidean(point_a, point_b);
                if current_pair < closest_pair {
                    closest_pair = current_pair;
                }
            }
        }
        return closest_pair;
    }

    fn drawings<'a>(&self, points: &'a [Point<T>]) -> Vec<Vec<Drawing<T>>> {
        let mut drawings = vec![];
        let mut closest_pair = ClosestPair::euclidean(&points[0], &points[1]);
        for (i, point_a) in points.iter().enumerate() {
            for point_b in &points[i + 1..] {
                let current_pair = ClosestPair::euclidean(point_a, point_b);

                {
                    let mut current_drawing = vec![];
                    current_drawing.push(Drawing::Point(point_a.clone(), Color32::RED));
                    for point in points.iter() {
                        current_drawing.push(Drawing::Point(point.clone(), Color32::WHITE));
                        if current_pair.point_a == point_a && current_pair.point_b == point {
                            continue;
                        }
                        if closest_pair.point_a == point_a && closest_pair.point_b == point {
                            continue;
                        }
                        current_drawing.push(Drawing::Line(
                            point_a.clone(),
                            point.clone(),
                            Color32::WHITE,
                        ));
                    }
                    current_drawing.push(Drawing::Line(
                        current_pair.point_a.clone(),
                        current_pair.point_b.clone(),
                        Color32::GREEN,
                    ));
                    current_drawing.push(Drawing::Line(
                        closest_pair.point_a.clone(),
                        closest_pair.point_b.clone(),
                        Color32::RED,
                    ));
                    drawings.push(current_drawing);
                }

                if current_pair < closest_pair {
                    closest_pair = current_pair;
                }
            }
        }

        return drawings;
    }
}
