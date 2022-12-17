use emas_rs::conf_functions::combat::default_combat;
use emas_rs::conf_functions::fitness::rastrigin_fitness::{domain, rastrigin};
use emas_rs::conf_functions::gene_mutations::normal_mutation;
use emas_rs::conf_functions::gene_pickers::stochastic_pick;
use emas_rs::conf_functions::reproduction_chance::default_reproduction_chance;
use emas_rs::conf_functions::RealConfiguration;
use emas_rs::SystemBuilder;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    const N: usize = 100;
    let rastrigin100d_configuration = RealConfiguration {
        domain: domain(),
        fitness: rastrigin::<N>,
        combat: default_combat,
        reproduction_chance: default_reproduction_chance,
        pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.5),
        mutate_gene: |gene, domain, rng| normal_mutation(gene, domain, rng, 1.0),
    };
    let mut system = SystemBuilder::new(rastrigin100d_configuration)
        .steps(100_000)
        .build(thread_rng());
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
