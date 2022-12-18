use emas_rs::conf_functions::combat_functions::default_combat;
use emas_rs::conf_functions::fitness_functions::labs_fitness::labs;
use emas_rs::conf_functions::gene_mutations::negation_mutation;
use emas_rs::conf_functions::gene_pickers::stochastic_pick;
use emas_rs::conf_functions::pick_action_functions::default_pick_action;
use emas_rs::conf_functions::recombination_functions::random_cut_point_recombine;
use emas_rs::conf_functions::BinaryConfiguration;
use emas_rs::SystemBuilder;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    const N: usize = 100;
    let labs100d_configuration = BinaryConfiguration {
        fitness: labs,
        combat_winner: default_combat,
        pick_action: default_pick_action,
        pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.5),
        mutate_gene: |gene, _rng| negation_mutation(gene),
        recombine: random_cut_point_recombine,
    };
    let mut system = SystemBuilder::<N, _, _, _>::new(labs100d_configuration)
        .steps(1_000_000)
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
