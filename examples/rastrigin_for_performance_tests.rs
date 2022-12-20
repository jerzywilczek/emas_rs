use emas_rs::conf_functions::combat::default_combat;
use emas_rs::conf_functions::fitness::rastrigin_fitness::{domain, rastrigin};
use emas_rs::conf_functions::gene_mutations::fixed_range_mutation;
use emas_rs::conf_functions::gene_pickers::stochastic_pick;
use emas_rs::conf_functions::pick_action::default_pick_action;
use emas_rs::conf_functions::recombination::random_cut_point_recombine;
use emas_rs::conf_functions::{topologies, RealConfiguration};
use emas_rs::SystemBuilder;
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
        .steps(300_000)
        .migration_steps(250)
        .build(rand_chacha::ChaCha12Rng::from_seed([42; 32]));

    let sol = system.run();
    println!("{sol:?}")
}
