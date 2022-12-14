use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::marker::PhantomData;
use crate::R;

pub fn stochastic_pick<const N: usize>(genes: &mut [f64; N], domain: &[(f64, f64); N], mutation_chance: f64, rng: &mut R) -> Vec<(&mut f64, (f64, f64))> {
    let mut mutated_bits_count: usize = (mutation_chance * genes.len() as f64) as usize;
    let chance_for_extra_bit: f64 = (mutation_chance * genes.len() as f64) - mutated_bits_count as f64;
    if rng.gen::<f64>() < chance_for_extra_bit {
        mutated_bits_count += 1;
    }

    let picked_genes: Vec<(&mut f64, (f64, f64))> = genes
        .iter_mut()
        .zip(*domain)
        .choose_multiple(rng, mutated_bits_count);
    picked_genes
}