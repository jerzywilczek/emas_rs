pub fn my_mutation(gene: &mut f64, domain: (f64, f64), rng: &mut R) {
    *gene += 3.5 * (rng.gen::<f64>() - 0.5);
    *gene = gene.clamp(domain.0, domain.1);
}