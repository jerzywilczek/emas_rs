use emas_rs::{conf_functions, StopCondition, SystemBuilder};
use rand::thread_rng;
use std::time::Duration;

fn main() {
    SystemBuilder::new(conf_functions::predefined::rastrigin::<10>())
        .stop_condition(StopCondition::TimeLimit(Duration::from_secs(30)))
        .live_plotting(true)
        .build(thread_rng())
        .run();
}
