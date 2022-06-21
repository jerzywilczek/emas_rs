use emas_rs::{SystemBuilder, ReproductionChance, fitness_functions::FitnessFn, conf_functions::{CombatWinChanceFn, ReproductionChanceFn}};


struct Rosenbrock2d;
impl FitnessFn<2> for Rosenbrock2d {
    const DOMAIN: [(f64, f64); 2] = [(-5.0, 5.0), (-5.0, 5.0)];

    fn call(args: &[f64; 2]) -> f64 {
        let a: f64 = 1.0;
        let b: f64 = 100.0;
        let [x, y] = args;

        (a - x).powi(2) + b * (y- x.powi(2)).powi(2)
    }
}

struct CombatFunction;
impl CombatWinChanceFn for CombatFunction {
    fn call(this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
        if this_agent_fitness > other_agent_fitness {
            return 0.9;
        }
        return 0.1;
    }
}

struct ReproductionFn;
impl ReproductionChanceFn for ReproductionFn {
    fn call(energy: u32) -> ReproductionChance {
        if energy >= 30 {
            return ReproductionChance(1.0);
        }
        ReproductionChance(0.0)
    }
}

fn main() {
    let mut system = SystemBuilder::<2, Rosenbrock2d, CombatFunction, ReproductionFn>::new()
        .island_amount(10)
        .steps(10_000)
        .agents_per_island(100)
        .agent_energy(20)
        .energy_passed_on_reproduction(0.3)
        .combat_energy(3)
        .migration_steps(300)
        .log_steps(100)
        .migrations_best_amount(20)
        .migrations_elite_amount(10)
        .build();
    
    let [sol_x, sol_y] = system.run();

    println!("[{}, {}] => {}", sol_x, sol_y, Rosenbrock2d::call(&[sol_x, sol_y]))
}