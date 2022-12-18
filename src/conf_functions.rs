use std::fmt::Debug;

use crate::{Action, R};

pub mod combat_functions;
pub mod fitness_functions;
pub mod gene_mutations;
pub mod gene_pickers;
pub mod pick_action_functions;
pub mod recombination_functions;

type GenesForMutation<'a> = Vec<(&'a mut f64, (f64, f64))>;

type PickGenesFn<const N: usize> = for<'a> fn(
    genes: &'a mut [f64; N],
    domain: &[(f64, f64); N],
    rng: &mut R,
) -> GenesForMutation<'a>;

type BinaryGenesForMutation<'a> = Vec<(&'a mut bool)>;

type PickBinaryGenesFn<const N: usize> = for<'a> fn(
    genes: &'a mut [f64; N],
    rng: &mut R,
) -> BinaryGenesForMutation<'a>;

type RecombinationFn<const N: usize, T: Default + Copy + Clone> =
    fn(parents_genes: (&[T; N], &[T; N]), rng: &mut R) -> ([T; N], [T; N]);

pub struct RealConfiguration<const N: usize> {
    pub domain: [(f64, f64); N],
    pub fitness: fn(args: &[f64; N]) -> f64,
    pub combat_winner: fn(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool,
    pub pick_action: fn(energy: u32, rng: &mut R) -> Action,
    pub pick_genes: PickGenesFn<N>,
    pub mutate_gene: fn(value: &mut f64, domain: (f64, f64), rng: &mut R),
    pub recombine: RecombinationFn<N, f64>,
}

pub struct BinaryConfiguration<const N: usize> {
    pub fitness: fn(args: &[bool; N]) -> f64,
    pub combat_winner: fn(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool,
    pub pick_action: fn(energy: u32, rng: &mut R) -> Action,
    pub pick_genes: PickBinaryGenesFn<N>,
    pub mutate_gene: fn(value: &mut bool, rng: &mut R),
    pub recombine: RecombinationFn<N, bool>,
}

impl<const N: usize> Debug for RealConfiguration<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RealConfiguration").finish()
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

    fn random_genes(&self, _rng: &mut R) -> [f64; N] {
        [1.1; N] // FIXME
    }

    fn recombine(
        &self,
        parents_genes: (&[f64; N], &[f64; N]),
        rng: &mut R,
    ) -> ([f64; N], [f64; N]) {
        (self.recombine)(parents_genes, rng)
    }
}
