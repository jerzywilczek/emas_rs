use emas_rs::SystemBuilder;
use emas_rs::fitness_functions::{RastriginFitness, FitnessFn};
use std::time::Instant;

fn main() {
    let mut system = SystemBuilder::<2, RastriginFitness<2>>::new().steps(10_000).build();
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();
    println!("[{}, {}] => {}", sol[0], sol[1], RastriginFitness::call(&sol));
    println!("{}s", t0);
}