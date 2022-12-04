use std::marker::PhantomData;
use crate::{FitnessFn, ReproductionChance};
use rand::{thread_rng, Rng};

pub trait CombatWinChanceFn {
    fn call(this_agent_fitness: f64, other_agent_fitness: f64) -> f64;
}

pub struct DefaultCombatWinChanceFn;

impl CombatWinChanceFn for DefaultCombatWinChanceFn {
    fn call(this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
        if this_agent_fitness < other_agent_fitness {
            return 0.8;
        }
        0.2
    }
}

pub trait ReproductionChanceFn {
    fn call(energy: u32) -> ReproductionChance;
}

pub struct DefaultReproductionChanceFn;

impl ReproductionChanceFn for DefaultReproductionChanceFn {
    fn call(energy: u32) -> ReproductionChance {
        if energy < 25 {
            return ReproductionChance(0.0);
        }
        if energy < 50 {
            return ReproductionChance(0.25);
        }
        ReproductionChance(1.0)
    }
}

pub trait MutationFn<const N: usize, F: FitnessFn<N>> {
    fn call(genes: &mut [f64; N]);
}

pub struct DefaultMutationFn<const N: usize, F: FitnessFn<N>> {
    f_phantom: PhantomData<F>,
}

impl<const N: usize, F: FitnessFn<N>> MutationFn<N, F> for DefaultMutationFn<N, F> {
    fn call(genes: &mut [f64; N]) {
        let mut rng = thread_rng();
        let gene_mut_chance = 1.0;
        for i in 0..N {
            let mutation_range = (F::DOMAIN[i].1 - F::DOMAIN[i].0) / 20.0;
            if rng.gen::<f64>() < gene_mut_chance {
                let mutation_value = rng.gen::<f64>() * mutation_range;
                if rng.gen() {
                    genes[i] = (genes[i] + mutation_value).min(F::DOMAIN[i].1);
                } else {
                    genes[i] = (genes[i] - mutation_value).max(F::DOMAIN[i].0);
                }
            }
        }
    }
}
