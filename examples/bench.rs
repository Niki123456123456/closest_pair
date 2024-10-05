use closest_pair::{
    BruteForce, ClosestPairAlgorithm, DivideAndConquer, DivideAndConquerPresortedY, GridAlgorithmConst, Number, Point, SweepLine
};
use rand::{distributions::Standard, prelude::Distribution};
use std::{fmt::Debug, time::Duration};

fn print_thousands(i: impl ToString) -> String {
    i.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}

fn main() {
    let mut rng = rand::thread_rng();
    //let sizes = vec![10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];
    let mut sizes: Vec<usize> = vec![4, 4];
    for _ in 0..18_usize {
        // 18 25 
        sizes.push(sizes.last().unwrap() * 2);
    }
    println!("{}", print_thousands(sizes.last().unwrap()));
    bench::<f64>(&sizes, 10, &mut rng);
    //bench::<f64>(&sizes, 1, &mut rng);

}

fn bench<T: Number + Debug + 'static>(sizes: &Vec<usize>, repeat: usize, rng: &mut rand::prelude::ThreadRng) where
Standard: Distribution<T>, {
    let mut results = vec![];
    for size in sizes {
        let mut row = vec![];
        row.push(size.to_string());
        let points = generate_points::<T>(*size, rng);

        let algorithms: Vec<Box<dyn ClosestPairAlgorithm<T>>> = vec![
            //Box::new(BruteForce),
            Box::new(DivideAndConquerPresortedY),
            Box::new(DivideAndConquer),
            //Box::new(SweepLine),
            //Box::new(GridAlgorithmConst),
        ];
        for algo in algorithms {
            if algo.limit() > *size {
                let mut durations = vec![];
                let mut distance = T::MIN;
                for _ in 0..repeat {
                    let start = web_time::Instant::now();
                    let result = algo.execute(&points);
                    let duration: std::time::Duration = start.elapsed();
                    distance = result.distance;
                    durations.push(duration);
                }
                let duration: Duration = durations.iter().sum::<Duration>() / repeat as u32;
                let duration_per_point = duration.as_micros() as f64 / *size as f64;
                println!(
                    "{}: {:?}: {}ns {:.2}ms/point ({})",
                    print_thousands(size),
                    distance,
                    duration.as_nanos(),
                    duration_per_point,
                    algo.name(),
                );
                row.push(duration.as_nanos().to_string());
            } else {
                row.push("-".to_string());
            }
        }
   
        results.push(row.join(", "));
    }

    println!("{}", results.join("\n"));
    }

fn generate_points<T: Number>(len: usize, rng: &mut rand::prelude::ThreadRng) -> Vec<Point<T>>
where
    Standard: Distribution<T>,
{
    let mut points: Vec<Point<T>> = vec![];
    for _ in 0..len {
        points.push(Point::ran(rng));
    }
    return points;
}
