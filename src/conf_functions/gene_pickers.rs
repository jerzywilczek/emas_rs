use crate::R;
use rand::seq::IteratorRandom;
use rand::Rng;

pub fn stochastic_pick<'a, const N: usize>(
    genes: &'a mut [f64; N],
    domain: &[(f64, f64); N],
    rng: &mut R,
    mutation_chance: f64,
) -> Vec<(&'a mut f64, (f64, f64))> {
    let mut mutated_bits_count: usize = (mutation_chance * genes.len() as f64) as usize;
    let chance_for_extra_bit: f64 =
        (mutation_chance * genes.len() as f64) - mutated_bits_count as f64;
    if rng.gen::<f64>() < chance_for_extra_bit {
        mutated_bits_count += 1;
    }

    let picked_genes: Vec<(&mut f64, (f64, f64))> = genes
        .iter_mut()
        .zip(*domain)
        .choose_multiple(rng, mutated_bits_count);
    picked_genes
}
