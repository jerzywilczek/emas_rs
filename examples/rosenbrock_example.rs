use emas_rs::SystemBuilder;
use emas_rs::fitness_functions::FitnessFn;
use std::time::Instant;
use std::process::Command;
use std::time;
use std::thread::sleep;

struct RosenbrockFitness {}

impl FitnessFn<2> for RosenbrockFitness {
    const DOMAIN: [(f64, f64); 2] = [(-5.0, 5.0); 2];

    fn call(args: &[f64; 2]) -> f64 {
        let a: f64 = 1.0;
        let b: f64 = 100.0;
        let [x, y] = args;

        (a - x).powi(2) + b * (y- x.powi(2)).powi(2)
    }
}

fn main() {
    let mut list_dir = Command::new("python3");

// Execute `ls` in the current directory of the program.
    let mut child = list_dir.arg("./plotting/live_plotting.py").spawn().expect("process failed to execute");
    sleep(time::Duration::from_millis(2000));
    let mut system = SystemBuilder::<2, RosenbrockFitness>::new().steps(10_000).build();
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();
    println!("[{}, {}] => {}", sol[0], sol[1], RosenbrockFitness::call(&sol));
    println!("{}s", t0);
    sleep(time::Duration::from_millis(5000));
    child.kill().expect("Can''t kill the child process");
}