use emas_rs::fitness_functions::{FitnessFn, RastriginFitness};
use emas_rs::SystemBuilder;

fn main() {
    let mut system = SystemBuilder::<3, RastriginFitness<3>>::new()
        .steps(1_000_000)
        .build();

    let solution = system.run();

    println!("[{}, {}, {}] => {}", solution[0], solution[1], solution[2], RastriginFitness::call(&solution));
}