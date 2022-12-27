let rastrigin100d_configuration = RealConfiguration {
    domain: domain(),
    fitness: rastrigin::<N>,
    combat_winner: default_combat,
    pick_action: default_pick_action,
    recombine: random_cut_point_recombine,
    pick_genes: half_pick,
    mutate_gene: |gene, domain, rng| normal_mutation(gene, domain, rng, 1.0),
};
