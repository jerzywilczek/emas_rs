use emas_rs::conf_functions::{
    RealConfiguration,
    combat::default_combat,
    fitness::rastrigin_fitness::{domain, rastrigin},
    gene_mutations::normal_mutation,
    gene_pickers::stochastic_pick,
    pick_action::default_pick_action,
    recombination::random_cut_point_recombine,
};
use emas_rs::{StopCondition, SystemBuilder};

use std::time::{Duration, Instant};

fn main() {
    const N: usize = 100;
    let rastrigin100d_configuration = RealConfiguration {
        domain: domain(),
        fitness: rastrigin::<N>,
        combat_winner: default_combat,
        pick_action: default_pick_action,
        recombine: random_cut_point_recombine,
        pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.5),
        mutate_gene: |gene, domain, rng| normal_mutation(gene, domain, rng, 1.0),
    };

    let mut system = SystemBuilder::new(rastrigin100d_configuration)
        .stop_condition(StopCondition::TimeLimit(Duration::from_secs(10)))
        .build(rand::rng());

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
