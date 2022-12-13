use crate::ReproductionChance;

pub mod combat_functions;
pub mod fitness_functions;
pub mod mutation_functions;
pub mod reproduction_chance_functions;
pub mod topologies;

pub trait ConfigurableFunctions<const N: usize> {
    const DOMAIN: [(f64, f64); N];

    fn fitness(args: &[f64; N]) -> f64;
    fn combat(this_agent_fitness: f64, other_agent_fitness: f64) -> f64;
    fn reproduction_chance(energy: u32) -> ReproductionChance;
    fn mutate(genes: &mut [f64; N]);

    //TODO using N as a placeholder for now - it needs to be changed to islands_amount (somehow)
    fn topology() -> [Vec<usize>; N];
}
