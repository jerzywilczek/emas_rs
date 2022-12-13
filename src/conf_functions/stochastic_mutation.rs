use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::marker::PhantomData;

pub fn stochastic_mutate<const N: usize, M: Mutation>(
    genes: &mut [f64; N],
    domain: &[(f64, f64); N],
    mutation: &M,
) {
    let mut rng = thread_rng();
    let mut_chance = 0.5;
    let mut mutated_bits_count: usize = (mut_chance * genes.len() as f64) as usize;
    let chance_for_extra_bit: f64 = (mut_chance * genes.len() as f64) - mutated_bits_count as f64;
    if rng.gen::<f64>() < chance_for_extra_bit {
        mutated_bits_count += 1;
    }

    let genes_to_mutate: Vec<(&mut f64, &(f64, f64))> = genes
        .iter_mut()
        .zip(domain)
        .choose_multiple(&mut rng, mutated_bits_count);

    for (mut gene, gene_domain) in genes_to_mutate {
        mutation.mutate_gene(gene, gene_domain, &mut rng);
    }
}

fn limit_gene_value(gene: &mut f64, range: &(f64, f64)) {
    *gene = gene.max((*range).0).min((*range).1);
}

pub trait Mutation {
    fn mutate_gene(&self, gene: &mut T, range: &(T, T), rng: &mut ThreadRng);
}

#[derive(Clone, Copy)]
pub struct AddXMutation {
    x: f64,
}

impl Mutation for AddXMutation {
    fn mutate_gene(&self, gene: &mut f64, _range: &(f64, f64), _rng: &mut ThreadRng) {
        *gene += self.x;
    }
}

#[derive(Clone, Copy)]
pub struct FixedRangeMutation {
    mutation_range: f64,
}

impl Mutation for FixedRangeMutation {
    fn mutate_gene(&self, gene: &mut f64, range: &(f64, f64), rng: &mut ThreadRng) {
        *gene += self.mutation_range * (-1.0 + rng.gen::<f64>() * 2.0);
        limit_gene_value(gene, range);
    }
}

#[derive(Clone, Copy)]
pub struct NormalMutation {
    mutation_range: f64,
}

impl Mutation for NormalMutation {
    fn mutate_gene(&self, gene: &mut f64, range: &(f64, f64), rng: &mut ThreadRng) {
        *gene += self.mutation_range + (PI * (rng.gen::<f64>() - 0.5)).tan();
        limit_gene_value(gene, range);
    }
}

#[derive(Clone, Copy)]
pub struct NormalMicroMacroMutation {
    double_threshold: (f64, f64),
    micro_macro_factor: f64,
    mutation_range: f64,
}

impl Mutation for NormalMicroMacroMutation {
    fn mutate_gene(&self, gene: &mut f64, range: &(f64, f64), rng: &mut ThreadRng) {
        let rand = rng.gen::<f64>();
        let mut mutation_range = self.mutation_range;

        if rand < self.double_threshold.0 {
            mutation_range *= self.micro_macro_factor;
        } else if rand < self.double_threshold.1 {
            mutation_range /= self.micro_macro_factor;
        }

        *gene += mutation_range + (PI * (rng.gen::<f64>() - 0.5)).tan();
        limit_gene_value(gene, range);
    }
}
