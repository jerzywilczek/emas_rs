pub fn normal_micro_macro_mutation(
    gene: &mut f64,
    domain: (f64, f64),
    rng: &mut R,
    mutation_range: f64,
    micro_macro_factor: f64,
    double_threshold: (f64, f64),
)