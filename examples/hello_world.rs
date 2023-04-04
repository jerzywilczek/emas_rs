use emas_rs::{conf_functions, SystemBuilder};
use rand::thread_rng;

fn main() {
    // "10" is function domain dimension, i.e. f: R^10 -> R.
    let solution = SystemBuilder::new(conf_functions::predefined::rastrigin::<10>())
        .build(thread_rng())
        .run();

    println!(
        "Fitness: {}",
        conf_functions::fitness::rastrigin_fitness::rastrigin(&solution)
    );
}
