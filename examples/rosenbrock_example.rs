use emas_rs::conf_functions::combat_functions::default_combat;
use emas_rs::conf_functions::reproduction_chance_functions::default_reproduction_chance;
use emas_rs::conf_functions::ConfigurableFunctions;
use emas_rs::SystemBuilder;
use std::process::Command;
use std::thread::sleep;
use std::time;
use std::time::Instant;

const DOMAIN: [(f64, f64); 2] = [(-5.0, 5.0); 2];

fn rosenbrock_2d(args: &[f64; 2]) -> f64 {
    let a: f64 = 1.0;
    let b: f64 = 100.0;
    let [x, y] = args;

    (a - x).powi(2) + b * (y - x.powi(2)).powi(2)
}

struct Configuration;

impl ConfigurableFunctions<2> for Configuration {
    const DOMAIN: [(f64, f64); 2] = DOMAIN;

    fn fitness(args: &[f64; 2]) -> f64 {
        rosenbrock_2d(args)
    }

    fn combat(this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
        default_combat(this_agent_fitness, other_agent_fitness)
    }

    fn reproduction_chance(energy: u32) -> emas_rs::ReproductionChance {
        default_reproduction_chance(energy)
    }
}

fn main() {
    let mut list_dir = Command::new("python3");

    // Execute `ls` in the current directory of the program.
    let mut child = list_dir
        .arg("./plotting/live_plotting.py")
        .spawn()
        .expect("process failed to execute");
    sleep(time::Duration::from_millis(2000));
    let mut system = SystemBuilder::<2, Configuration>::new()
        .steps(1_000_000)
        .build();
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();
    println!("[{}, {}] => {}", sol[0], sol[1], rosenbrock_2d(&sol));
    println!("{}s", t0);
    sleep(time::Duration::from_millis(5000));
    child.kill().expect("Can''t kill the child process");
}
