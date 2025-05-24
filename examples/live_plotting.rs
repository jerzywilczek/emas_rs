use emas_rs::{StopCondition, SystemBuilder, conf_functions};
use std::time::Duration;

fn main() {
    SystemBuilder::new(conf_functions::predefined::rastrigin::<10>())
        .stop_condition(StopCondition::TimeLimit(Duration::from_secs(30)))
        .live_plotting(true)
        .build(rand::rng())
        .run();
}
