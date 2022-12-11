use emas_rs::conf_functions::combat_functions::default_combat;
use emas_rs::conf_functions::fitness_functions::rastrigin_fitness::{domain, rastrigin};
use emas_rs::conf_functions::mutation_functions::default_mutation;
use emas_rs::conf_functions::reproduction_chance_functions::default_reproduction_chance;
use emas_rs::conf_functions::ConfigurableFunctions;
use emas_rs::SystemBuilder;
use std::time::Instant;

const N: usize = 100;
struct Configuration;

impl ConfigurableFunctions<N> for Configuration {
    const DOMAIN: [(f64, f64); N] = domain();

    fn fitness(args: &[f64; N]) -> f64 {
        rastrigin(args)
    }

    fn combat(this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
        default_combat(this_agent_fitness, other_agent_fitness)
    }

    fn reproduction_chance(energy: u32) -> emas_rs::ReproductionChance {
        default_reproduction_chance(energy)
    }

    fn mutate(genes: &mut [f64; N]) {
        default_mutation(genes, &domain());
    }
}

fn main() {
    const N: usize = 100;
    let mut system = SystemBuilder::<N, Configuration>::new()
        .steps(1_000_000)
        .build();
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
    println!("] => {}", rastrigin(&sol));
    println!("{}s", t0);
}
