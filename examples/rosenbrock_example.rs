use emas_rs::{SystemBuilder, FitnessFn};
use std::time::Instant;

struct RosenbrockFitness {}

impl FitnessFn for RosenbrockFitness {
    fn call(args: &[f64; 2]) -> f64 {
        let a: f64 = 1.0;
        let b: f64 = 100.0;
        let [x, y] = args;

        (a - x).powi(2) + b * (y- x.powi(2)).powi(2)
    }
}

fn main() {
    let mut system = SystemBuilder::<RosenbrockFitness>::new().steps(10_000).build();
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();
    println!("[{}, {}] => {}", sol[0], sol[1], RosenbrockFitness::call(&sol));
    println!("{}s", t0);
}