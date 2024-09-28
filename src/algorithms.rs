pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub struct ClosestPair{
    pub pair : Vec2<usize>,
    pub distance : f32,
}

impl ClosestPair {
    fn new(x : usize, y : usize, distance : f32) -> Self {
        Self { pair: Vec2 { x, y }, distance }
    }
}

pub fn closest_pair_index(points: &[Vec2<f32>]) -> Option<ClosestPair> {
    if points.len() < 2 {
        return None;
    }
    let mut closest_pair = ClosestPair::new(0, 1, distance(&points[0], &points[1]));

    for x in 0..points.len() {
        for y in (x + 1)..points.len() {
            let current_distance = distance(&points[x], &points[y]);
            if current_distance < closest_pair.distance {
                closest_pair = ClosestPair::new(x, y, current_distance);
            }
        }
    }
    Some(closest_pair)
}

pub fn distance(a: &Vec2<f32>, b: &Vec2<f32>) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub fn divide_and_conquer(points: &[Vec2<f32>]) -> Option<ClosestPair> {
    let n = points.len();
    match n {
        0 | 1 => {
            return None;
        }
        2 => {
            return Some(ClosestPair::new(0, 1, distance(&points[0], &points[1])));
        }
        3 => {
            let d0 = distance(&points[0], &points[1]);
            let d1 = distance(&points[1], &points[2]);
            if d0 < d1 {
                return Some(ClosestPair::new(0, 1, d0));
            } 
            return Some(ClosestPair::new(1, 2, d1));
        }
        _ => {
            let m = n / 2;
            let d0 = divide_and_conquer(&points[..m]).unwrap();
            let d1 = divide_and_conquer(&points[m+1..]).unwrap();
            let d = d0.distance.min(d1.distance);

            
        }
    }


    return None;
}

// https://www.cs.cmu.edu/~15451-s20/lectures/lec23-closest-pair.pdf