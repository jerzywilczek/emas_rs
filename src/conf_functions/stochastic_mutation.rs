use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::marker::PhantomData;

pub fn stochastic_mutate<const N: usize>(genes: &mut [f64; N], domain: &[(f64, f64); N], mutation_chance: f64) -> Vec<(&mut f64, &(f64, f64))> {
    let mut rng = thread_rng();
    let mut mutated_bits_count: usize = (mutation_chance * genes.len() as f64) as usize;
    let chance_for_extra_bit: f64 = (mutation_chance * genes.len() as f64) - mutated_bits_count as f64;
    if rng.gen::<f64>() < chance_for_extra_bit {
        mutated_bits_count += 1;
    }

    let picked_genes: Vec<(&mut f64, &(f64, f64))> = genes
        .iter_mut()
        .zip(domain)
        .choose_multiple(&mut rng, mutated_bits_count);
    picked_genes
}

fn limit_gene_value(gene: &mut f64, range: &(f64, f64)) {
    *gene = gene.max((*range).0).min((*range).1);
}

pub fn fixed_range_mutation(gene: &mut f64, domain: &(f64, f64), rng: &mut ThreadRng, mutation_range: f64) {
    *gene += mutation_range * (-1.0 + rng.gen::<f64>() * 2.0);
    limit_gene_value(gene, domain);
}

pub fn normal_mutation(gene: &mut f64, domain: &(f64, f64), rng: &mut ThreadRng) {
    *gene += mutation_range + (PI * (rng.gen::<f64>() - 0.5)).tan();
    limit_gene_value(gene, domain);
}


pub fn normal_micro_macro_mutation(gene: &mut f64, domain: &(f64, f64), rng: &mut ThreadRng, mutation_range: f64, micro_macro_factor: f64, double_threshold: (f64, f64)) {
    let rand = rng.gen::<f64>();
    let mut range = mutation_range;

    if rand < double_threshold.0 {
        range *= micro_macro_factor;
    } else if rand < double_threshold.1 {
        range /= micro_macro_factor;
    }

    *gene += mutation_range * (PI * (rng.gen::<f64>() - 0.5)).tan();
    limit_gene_value(gene, domain);
}
