use emas_rs::{SystemBuilder, RastriginFitness, FitnessFn};
use std::time::Instant;

fn main() {
    let mut system = SystemBuilder::<RastriginFitness>::new().steps(10_000).build();
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();
    println!("[{}, {}] => {}", sol[0], sol[1], RastriginFitness::call(&sol));
    println!("{}s", t0);
}