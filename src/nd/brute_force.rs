use super::*;

pub struct BruteForce;

impl ClosestPairAlgorithm for BruteForce {
    fn execute<'a, const DIMENSION: usize>(&self, points: &'a[Point<DIMENSION>]) -> ClosestPair<'a, DIMENSION> {
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
}