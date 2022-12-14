use crate::ReproductionChance;

pub mod combat_functions;
pub mod fitness_functions;
pub mod gene_mutations;
pub mod gene_pickers;
pub mod mutation_functions;
pub mod reproduction_chance_functions;

pub type FitnessFn<const N: usize, T> = fn(&[T; N]) -> f64;

pub struct RealConfiguration<const N: usize> {
    /// TODO!
    pub domain: [(f64, f64); N],
    pub fitness: fn(args: &[f64; N]) -> f64,
    pub combat: fn(this_agent_fitness: f64, other_agent_fitness: f64) -> f64,
    pub reproduction_chance: fn(energy: u32) -> ReproductionChance,
    #[allow(clippy::type_complexity)]
    pub pick_genes: for<'a> fn(
        genes: &'a mut [f64; N],
        domain: &'a [(f64, f64); N],
        rng: &mut super::R,
    ) -> Vec<(&'a mut f64, (f64, f64))>,
    pub mutate_gene: fn(value: &mut f64, domain: (f64, f64), rng: &mut super::R),
}

pub trait Configuration<const N: usize, T> {
    fn fitness(&self, args: &[T; N]) -> f64;

    fn combat(&self, this_agent_fitness: f64, other_agent_fitness: f64) -> f64;

    fn reproduction_chance(&self, energy: u32) -> ReproductionChance;

    fn mutate(&self, genes: &mut [T; N], rng: &mut super::R);

    fn random_genes(&self, rng: &mut super::R) -> [T; N];
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

    fn mutate(&self, genes: &mut [f64; N], rng: &mut super::R) {
        let picked_genes = (self.pick_genes)(genes, &self.domain, rng);

        for (value, domain) in picked_genes {
            (self.mutate_gene)(value, domain, rng)
        }
    }

    fn random_genes(&self, _rng: &mut super::R) -> [f64; N] {
        [1.1; N]
    }
}
