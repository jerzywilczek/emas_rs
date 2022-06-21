use emas_rs::SystemBuilder;
use emas_rs::fitness_functions::{RastriginFitness, FitnessFn};
use std::time::Instant;

fn main() {
    const N: usize = 100;
    let mut system = SystemBuilder::<N, RastriginFitness<N>>::new().steps(1_000_000).build();
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();

    print!("[");
    let mut first = true;
    for arg in sol {
        if first {
            print!("{}", arg);
            first = false;
        } else {
            print!(", {}", arg);
        }
    }
    println!("] => {}", RastriginFitness::call(&sol));
    println!("{}s", t0);
}