use emas_rs::conf_functions::{
    RealConfiguration,
    combat::default_combat,
    fitness::rastrigin_fitness::{domain, rastrigin},
    gene_mutations::fixed_range_mutation,
    gene_pickers::stochastic_pick,
    pick_action::default_pick_action,
    recombination::random_cut_point_recombine,
    topologies,
};
use emas_rs::{StopCondition, SystemBuilder};

use rand::SeedableRng;

fn main() {
    const N: usize = 100;
    let rastrigin100d_configuration = RealConfiguration {
        domain: domain(),
        fitness: rastrigin::<N>,
        combat_winner: default_combat,
        pick_action: default_pick_action,
        pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.5),
        mutate_gene: |gene, domain, rng| fixed_range_mutation(gene, domain, rng, 0.1),
        recombine: random_cut_point_recombine,
    };
    let mut system = SystemBuilder::new(rastrigin100d_configuration)
        .island_amount(32)
        .topology_fn(topologies::fully_connected)
        .stop_condition(StopCondition::TotalSteps(300_000))
        .migration_steps(250)
        .build(rand_chacha::ChaCha12Rng::from_seed([42; 32]));

    let sol = system.run();
    println!("{sol:?}")
}
