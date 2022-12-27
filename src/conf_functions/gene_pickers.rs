use crate::{Gene, R};
use rand::seq::IteratorRandom;
use rand::Rng;

pub fn stochastic_pick<'a, const N: usize, T: Gene>(
    genes: &'a mut [T; N],
    domain: &[(T, T); N],
    rng: &mut R,
    mutation_chance: f64,
) -> Vec<(&'a mut T, (T, T))> {
    let mut mutated_bits_count: usize = (mutation_chance * genes.len() as f64) as usize;
    let chance_for_extra_bit: f64 =
        (mutation_chance * genes.len() as f64) - mutated_bits_count as f64;
    if rng.gen::<f64>() < chance_for_extra_bit {
        mutated_bits_count += 1;
    }

    let picked_genes: Vec<(&mut T, (T, T))> = genes
        .iter_mut()
        .zip(*domain)
        .choose_multiple(rng, mutated_bits_count);
    picked_genes
}

pub fn half_pick<'a, const N: usize, T: Gene>(
    genes: &'a mut [T; N],
    domain: &[(T, T); N],
    rng: &mut R,
) -> Vec<(&'a mut T, (T, T))> {
    stochastic_pick(genes, domain, rng, 0.5)
}
