use rand::Rng;
use std::fmt::Debug;

use crate::{Action, R};

pub mod combat;
pub mod fitness;
pub mod gene_mutations;
pub mod gene_pickers;
pub mod pick_action;
pub mod recombination;
pub mod topologies;

type GenesForMutation<'a, T> = Vec<(&'a mut T, (T, T))>;

type PickGenesFn<const N: usize, T> =
    for<'a> fn(genes: &'a mut [T; N], domain: &[(T, T); N], rng: &mut R) -> GenesForMutation<'a, T>;

type RecombinationFn<const N: usize, T> =
    fn(parents_genes: (&[T; N], &[T; N]), rng: &mut R) -> ([T; N], [T; N]);

pub struct RealConfiguration<const N: usize> {
    pub domain: [(f64, f64); N],
    pub fitness: fn(args: &[f64; N]) -> f64,
    pub combat_winner: fn(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool,
    pub pick_action: fn(energy: u32, rng: &mut R) -> Action,
    pub pick_genes: PickGenesFn<N, f64>,
    pub mutate_gene: fn(value: &mut f64, domain: (f64, f64), rng: &mut R),
    pub recombine: RecombinationFn<N, f64>,
}

pub struct BinaryConfiguration<const N: usize> {
    pub fitness: fn(args: &[bool; N]) -> f64,
    pub combat_winner: fn(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool,
    pub pick_action: fn(energy: u32, rng: &mut R) -> Action,
    pub pick_genes: PickGenesFn<N, bool>,
    pub mutate_gene: fn(value: &mut bool, rng: &mut R),
    pub recombine: RecombinationFn<N, bool>,
}

pub struct IntegerConfiguration<const N: usize> {
    pub domain: [(i32, i32); N],
    pub fitness: fn(args: &[i32; N]) -> f64,
    pub combat_winner: fn(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool,
    pub pick_action: fn(energy: u32, rng: &mut R) -> Action,
    pub pick_genes: PickGenesFn<N, i32>,
    pub mutate_gene: fn(value: &mut i32, domain: (i32, i32), rng: &mut R),
    pub recombine: RecombinationFn<N, i32>,
}

impl<const N: usize> Debug for RealConfiguration<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RealConfiguration").finish()
    }
}

impl<const N: usize> Debug for BinaryConfiguration<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryConfiguration").finish()
    }
}

impl<const N: usize> Debug for IntegerConfiguration<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntegerConfiguration").finish()
    }
}

pub trait Configuration<const N: usize, T>: Debug {
    fn fitness(&self, args: &[T; N]) -> f64;

    fn combat_winner(&self, this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R)
        -> bool;

    fn pick_action(&self, energy: u32, rng: &mut R) -> Action;

    fn mutate(&self, genes: &mut [T; N], rng: &mut R);

    fn random_genes(&self, rng: &mut R) -> [T; N];

    fn recombine(&self, parents_genes: (&[T; N], &[T; N]), rng: &mut R) -> ([T; N], [T; N]);
}

impl<const N: usize> Configuration<N, f64> for RealConfiguration<N> {
    fn fitness(&self, args: &[f64; N]) -> f64 {
        (self.fitness)(args)
    }

    fn combat_winner(
        &self,
        this_agent_fitness: f64,
        other_agent_fitness: f64,
        rng: &mut R,
    ) -> bool {
        (self.combat_winner)(this_agent_fitness, other_agent_fitness, rng)
    }

    fn pick_action(&self, energy: u32, rng: &mut R) -> Action {
        (self.pick_action)(energy, rng)
    }

    fn mutate(&self, genes: &mut [f64; N], rng: &mut R) {
        let picked_genes = (self.pick_genes)(genes, &self.domain, rng);

        for (value, domain) in picked_genes {
            (self.mutate_gene)(value, domain, rng)
        }
    }

    fn random_genes(&self, rng: &mut R) -> [f64; N] {
        let mut genes = [0.0; N];
        for (gene, domain) in genes.iter_mut().zip(self.domain.iter()) {
            *gene = rng.gen_range(domain.0..domain.1);
        }

        genes
    }

    fn recombine(
        &self,
        parents_genes: (&[f64; N], &[f64; N]),
        rng: &mut R,
    ) -> ([f64; N], [f64; N]) {
        (self.recombine)(parents_genes, rng)
    }
}

impl<const N: usize> Configuration<N, i32> for IntegerConfiguration<N> {
    fn fitness(&self, args: &[i32; N]) -> f64 {
        (self.fitness)(args)
    }

    fn combat_winner(
        &self,
        this_agent_fitness: f64,
        other_agent_fitness: f64,
        rng: &mut R,
    ) -> bool {
        (self.combat_winner)(this_agent_fitness, other_agent_fitness, rng)
    }

    fn pick_action(&self, energy: u32, rng: &mut R) -> Action {
        (self.pick_action)(energy, rng)
    }

    fn mutate(&self, genes: &mut [i32; N], rng: &mut R) {
        let picked_genes = (self.pick_genes)(genes, &self.domain, rng);

        for (value, domain) in picked_genes {
            (self.mutate_gene)(value, domain, rng)
        }
    }

    fn random_genes(&self, rng: &mut R) -> [i32; N] {
        let mut genes = [0; N];
        for (gene, domain) in genes.iter_mut().zip(self.domain.iter()) {
            *gene = rng.gen_range(domain.0..domain.1);
        }

        genes
    }

    fn recombine(
        &self,
        parents_genes: (&[i32; N], &[i32; N]),
        rng: &mut R,
    ) -> ([i32; N], [i32; N]) {
        (self.recombine)(parents_genes, rng)
    }
}

impl<const N: usize> Configuration<N, bool> for BinaryConfiguration<N> {
    fn fitness(&self, args: &[bool; N]) -> f64 {
        (self.fitness)(args)
    }

    fn combat_winner(
        &self,
        this_agent_fitness: f64,
        other_agent_fitness: f64,
        rng: &mut R,
    ) -> bool {
        (self.combat_winner)(this_agent_fitness, other_agent_fitness, rng)
    }

    fn pick_action(&self, energy: u32, rng: &mut R) -> Action {
        (self.pick_action)(energy, rng)
    }

    fn mutate(&self, genes: &mut [bool; N], rng: &mut R) {
        let picked_genes = (self.pick_genes)(genes, &[(false, true); N], rng);

        for (value, _domain) in picked_genes {
            (self.mutate_gene)(value, rng)
        }
    }

    fn random_genes(&self, rng: &mut R) -> [bool; N] {
        let mut genes = [false; N];
        for gene in genes.iter_mut() {
            if rng.gen::<f64>() < 0.5 {
                *gene = !*gene;
            }
        }
        genes
    }

    fn recombine(
        &self,
        parents_genes: (&[bool; N], &[bool; N]),
        rng: &mut R,
    ) -> ([bool; N], [bool; N]) {
        (self.recombine)(parents_genes, rng)
    }
}

pub mod predefined {
    use super::*;

    pub fn rastrigin<const N: usize>() -> RealConfiguration<N> {
        RealConfiguration {
            domain: fitness::rastrigin_fitness::domain(),
            fitness: fitness::rastrigin_fitness::rastrigin,
            combat_winner: combat::default_combat,
            pick_action: pick_action::default_pick_action,
            recombine: recombination::random_cut_point_recombine,
            pick_genes: |genes, domain, rng| gene_pickers::stochastic_pick(genes, domain, rng, 0.5),
            mutate_gene: |gene, domain, rng| {
                gene_mutations::normal_mutation(gene, domain, rng, 1.0)
            },
        }
    }
}
