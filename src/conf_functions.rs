use crate::{ReproductionChance, R};

pub mod combat_functions;
pub mod fitness_functions;
pub mod gene_mutations;
pub mod gene_pickers;
pub mod mutation_functions;
pub mod reproduction_chance_functions;
pub mod recombination_functions;

type GenesForMutation<'a> = Vec<(&'a mut f64, (f64, f64))>;

type PickGenesFn<const N: usize> = for<'a> fn(
    genes: &'a mut [f64; N],
    domain: &[(f64, f64); N],
    rng: &mut R,
) -> GenesForMutation<'a>;

type RecombinationFn<const N: usize> =
    fn(parents_genes: (&[f64; N], &[f64; N]), rng: &mut R) -> ([f64; N], [f64; N]);

pub struct RealConfiguration<const N: usize> {
    pub domain: [(f64, f64); N],
    pub fitness: fn(args: &[f64; N]) -> f64,
    pub combat: fn(this_agent_fitness: f64, other_agent_fitness: f64) -> f64,
    pub reproduction_chance: fn(energy: u32) -> ReproductionChance,
    pub pick_genes: PickGenesFn<N>,
    pub mutate_gene: fn(value: &mut f64, domain: (f64, f64), rng: &mut R),
    pub recombine: RecombinationFn<N>,
}

pub trait Configuration<const N: usize, T> {
    fn fitness(&self, args: &[T; N]) -> f64;

    fn combat(&self, this_agent_fitness: f64, other_agent_fitness: f64) -> f64;

    fn reproduction_chance(&self, energy: u32) -> ReproductionChance;

    fn mutate(&self, genes: &mut [T; N], rng: &mut R);

    fn random_genes(&self, rng: &mut R) -> [T; N];

    fn recombine(&self, parents_genes: (&[T; N], &[T; N]), rng: &mut R) -> ([T; N], [T; N]);
}

impl<const N: usize> Configuration<N, f64> for RealConfiguration<N> {
    fn fitness(&self, args: &[f64; N]) -> f64 {
        (self.fitness)(args)
    }

    fn combat(&self, this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
        (self.combat)(this_agent_fitness, other_agent_fitness)
    }

    fn reproduction_chance(&self, energy: u32) -> ReproductionChance {
        (self.reproduction_chance)(energy)
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
