let rosenbrock2d_configuration = RealConfiguration {
    domain: DOMAIN,
    fitness: rosenbrock_2d,
    combat_winner: my_combat,
    pick_action: my_pick_action,
    recombine: |parents_genes, _rng| my_cut_point_recombine(parents_genes, N / 3 as usize),
    pick_genes: my_genes_pick,
    mutate_gene: my_mutation,
};