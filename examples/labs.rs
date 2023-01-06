use emas_rs::conf_functions::{
    combat::default_combat, fitness::labs_fitness::labs, gene_mutations::negation_mutation,
    gene_pickers::stochastic_pick, pick_action::default_pick_action,
    recombination::random_cut_point_recombine, BinaryConfiguration,
};
use emas_rs::{StopCondition, SystemBuilder};

use rand::thread_rng;
use std::time::{Duration, Instant};

fn main() {
    const N: usize = 100;
    let labs100d_configuration = BinaryConfiguration {
        fitness: labs::<N>,
        combat_winner: default_combat,
        pick_action: default_pick_action,
        pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.5),
        mutate_gene: |gene, _rng| negation_mutation(gene),
        recombine: random_cut_point_recombine,
    };

    let mut system = SystemBuilder::new(labs100d_configuration)
        .stop_condition(StopCondition::TimeLimit(Duration::from_secs(10)))
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
    println!("] => {}", labs(&sol));
    println!("{}s", t0);
}
