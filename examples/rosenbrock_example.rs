use rand::thread_rng;
use std::process::Command;
use std::thread::sleep;
use std::time;
use std::time::Instant;

use emas_rs::conf_functions::combat_functions::default_combat;
use emas_rs::conf_functions::gene_mutations::normal_mutation;
use emas_rs::conf_functions::gene_pickers::stochastic_pick;
use emas_rs::conf_functions::recombination_functions::random_cut_point_recombine;
use emas_rs::conf_functions::reproduction_chance_functions::default_reproduction_chance;
use emas_rs::conf_functions::RealConfiguration;
use emas_rs::SystemBuilder;

const DOMAIN: [(f64, f64); 2] = [(-5.0, 5.0); 2];

fn rosenbrock_2d(args: &[f64; 2]) -> f64 {
    let a: f64 = 1.0;
    let b: f64 = 100.0;
    let [x, y] = args;

    (a - x).powi(2) + b * (y - x.powi(2)).powi(2)
}

fn main() {
    let rosenbrock2d_configuration = RealConfiguration {
        domain: DOMAIN,
        fitness: rosenbrock_2d,
        combat: default_combat,
        reproduction_chance: default_reproduction_chance,
        pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.5),
        mutate_gene: |gene, domain, rng| normal_mutation(gene, domain, rng, 1.0),
        recombine: random_cut_point_recombine,
    };

    let mut list_dir = Command::new("python3");

    // Execute `ls` in the current directory of the program.
    let mut child = list_dir
        .arg("./plotting/live_plotting.py")
        .spawn()
        .expect("process failed to execute");
    sleep(time::Duration::from_millis(2000));
    let mut system = SystemBuilder::new(rosenbrock2d_configuration)
        .steps(1_000_000)
        .build(thread_rng());
    let t0 = Instant::now();
    let sol = system.run();
    let t0 = t0.elapsed().as_secs_f32();
    println!("[{}, {}] => {}", sol[0], sol[1], rosenbrock_2d(&sol));
    println!("{}s", t0);
    sleep(time::Duration::from_millis(5000));
    child.kill().expect("Can''t kill the child process");
}
