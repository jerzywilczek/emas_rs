use rand::rngs::ThreadRng;
use crate::ReproductionChance;

pub mod combat_functions;
pub mod fitness_functions;
pub mod mutation_functions;
pub mod reproduction_chance_functions;

pub struct RealConfiguration<const N: usize, R: rand::Rng> {
    /// TODO!
    domain: [(f64, f64); N],
    fitness: fn(args: &[f64; N]) -> f64,
    combat: fn(this_agent_fitness: f64, other_agent_fitness: f64) -> f64,
    reproduction_chance: fn(energy: u32) -> ReproductionChance,
    pick_genes: for<'a> fn(genes: &'a mut [f64; N], domain: &'a [(f64, f64); N], rng: &mut R) -> Vec<(&'a mut f64, &'a(f64, f64))>,
    mutate_gene: fn(value: &mut f64, domain: &(f64, f64), rng: &mut ThreadRng),
}

pub trait Configuration <const N: usize, R: rand::Rng, T>{
    fn fitness(&self, args: &[T; N]) -> T;

    fn combat(&self, this_agent_fitness: T, other_agent_fitness: T) -> T;

    fn reproduction_chance(&self, energy: u32) -> ReproductionChance;

    fn mutate(&self, genes: &mut [T; N], rng: &mut R);
}

impl<const N: usize, R: rand::Rng> Configuration<N, R, f64> for RealConfiguration<N, R> {
    fn fitness(&self, args: &[f64; N]) -> f64 {
        self.fitness(args)
    }

    fn combat(&self, this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
        self.combat(this_agent_fitness, other_agent_fitness)
    }

    fn reproduction_chance(&self, energy: u32) -> ReproductionChance {
        self.reproduction_chance(energy)
    }

    fn mutate(&self, genes: &mut [f64; N], rng: &mut R) {
        let picked_genes = self.pick_genes(genes, &self.domain, rng);

        for (mut value, domain) in picked_genes {
            self.mutate_gene(value, domain, rng)
        }
    }
}