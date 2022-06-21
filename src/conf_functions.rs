use crate::ReproductionChance;

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
