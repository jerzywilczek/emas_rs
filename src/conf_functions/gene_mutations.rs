use crate::R;
use rand::Rng;
use std::f64::consts::PI;

pub fn fixed_range_mutation(gene: &mut f64, domain: (f64, f64), rng: &mut R, mutation_range: f64) {
    *gene += mutation_range * (-1.0 + rng.gen::<f64>() * 2.0);
    *gene = gene.clamp(domain.0, domain.1);
}

pub fn normal_mutation(gene: &mut f64, domain: (f64, f64), rng: &mut R, mutation_range: f64) {
    *gene += mutation_range + (PI * (rng.gen::<f64>() - 0.5)).tan();
    *gene = gene.clamp(domain.0, domain.1);
}

pub fn normal_micro_macro_mutation(
    gene: &mut f64,
    domain: (f64, f64),
    rng: &mut R,
    mutation_range: f64,
    micro_macro_factor: f64,
    double_threshold: (f64, f64),
) {
    let rand = rng.gen::<f64>();
    let mut range = mutation_range;

    if rand < double_threshold.0 {
        range *= micro_macro_factor;
    } else if rand < double_threshold.1 {
        range /= micro_macro_factor;
    }

    *gene += range * (PI * (rng.gen::<f64>() - 0.5)).tan();
    *gene = gene.clamp(domain.0, domain.1);
}
