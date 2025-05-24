use emas_rs::{SystemBuilder, conf_functions};

fn main() {
    // "10" is function domain dimension, i.e. f: R^10 -> R
    let solution = SystemBuilder::new(conf_functions::predefined::rastrigin::<10>())
        .build(rand::rng())
        .run();

    println!(
        "Fitness: {}",
        conf_functions::fitness::rastrigin_fitness::rastrigin(&solution)
    );
}
