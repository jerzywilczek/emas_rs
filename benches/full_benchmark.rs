use criterion::{criterion_group, criterion_main, Criterion};
use emas_rs::{
    conf_functions::{
        combat::default_combat, fitness::rastrigin_fitness, gene_mutations::fixed_range_mutation,
        gene_pickers::stochastic_pick, pick_action::default_pick_action,
        recombination::random_cut_point_recombine, RealConfiguration,
    },
    SystemBuilder,
};
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

fn benchmark(c: &mut Criterion) {
    c.bench_function("benchmark", |b| {
        b.iter_batched(
            || {
                let conf = RealConfiguration {
                    fitness: rastrigin_fitness::rastrigin,
                    domain: rastrigin_fitness::domain(),
                    combat_winner: default_combat,
                    pick_genes: |genes, domain, rng| stochastic_pick(genes, domain, rng, 0.3),
                    mutate_gene: |gene, domain, rng| fixed_range_mutation(gene, domain, rng, 0.1),
                    pick_action: default_pick_action,
                    recombine: random_cut_point_recombine,
                };

                let s = SystemBuilder::<10, _, _, _>::new(conf)
                    .island_amount(16)
                    .topology_fn(emas_rs::conf_functions::topologies::fully_connected)
                    .migration_steps(50)
                    .steps(100);

                s.build(ChaCha12Rng::from_seed([42; 32]))
            },
            |mut s| s.run(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    name = bench;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(20)).measurement_time(std::time::Duration::from_secs(10));
    targets = benchmark
);
criterion_main!(bench);
