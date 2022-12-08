use rand::{thread_rng, Rng};

pub fn default_mutation<const N: usize>(genes: &mut [f64; N], domain: [(f64, f64); N]) {
    let mut rng = thread_rng();
    let gene_mut_chance = 1.0;
    for i in 0..N {
        let mutation_range = (domain[i].1 - domain[i].0) / 20.0;
        if rng.gen::<f64>() < gene_mut_chance {
            let mutation_value = rng.gen::<f64>() * mutation_range;
            if rng.gen() {
                genes[i] = (genes[i] + mutation_value).min(domain[i].1);
            } else {
                genes[i] = (genes[i] - mutation_value).max(domain[i].0);
            }
        }
    }
}
