use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use emas_rs::conf_functions::topologies::fully_connected;
use emas_rs::conf_functions::RealConfiguration;
use emas_rs::{Action, Gene, SystemBuilder};

use emas_rs::R;

const N: usize = 2;
const DOMAIN: [(f64, f64); N] = [(-5.0, 5.0); N];

fn rosenbrock_2d(args: &[f64; N]) -> f64 {
    let a: f64 = 1.0;
    let b: f64 = 100.0;
    let [x, y] = args;

    (a - x).powi(2) + b * (y - x.powi(2)).powi(2)
}

fn my_combat(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool {
    let mut win_chance = 0.2;
    if this_agent_fitness < other_agent_fitness {
        win_chance = 0.8;
    }
    if rng.gen::<f64>() < win_chance {
        return true;
    }
    false
}

pub fn my_pick_action(energy: u32, rng: &mut R) -> Action {
    let mut reproduce_with_chance = |chance: f64| {
        if rng.gen::<f64>() < chance {
            Action::Reproduce
        } else {
            Action::Combat
        }
    };

    if energy < 5 {
        return Action::Combat;
    }
    if energy < 25 {
        return reproduce_with_chance(0.5);
    }
    Action::Reproduce
}

pub fn my_genes_pick<'a, const N: usize, T: Gene>(
    genes: &'a mut [T; N],
    domain: &[(T, T); N],
    rng: &mut R,
) -> Vec<(&'a mut T, (T, T))> {
    let mutated_bits_count: usize = (0.25 * genes.len() as f64) as usize;

    let picked_genes: Vec<(&mut T, (T, T))> = genes
        .iter_mut()
        .zip(*domain)
        .choose_multiple(rng, mutated_bits_count);
    picked_genes
}

pub fn my_mutation(gene: &mut f64, domain: (f64, f64), rng: &mut R) {
    *gene += 3.5 * (rng.gen::<f64>() - 0.5);
    *gene = gene.clamp(domain.0, domain.1);
}

pub fn my_cut_point_recombine<const N: usize, T: Gene>(
    parents_genes: (&[T; N], &[T; N]),
    cut_point: usize,
) -> ([T; N], [T; N]) {
    let mut children_genes = ([T::default(); N], [T::default(); N]);

    for i in 0..cut_point {
        children_genes.0[i] = parents_genes.0[i];
        children_genes.1[i] = parents_genes.1[i];
    }
    for i in cut_point..N {
        children_genes.0[i] = parents_genes.1[i];
        children_genes.1[i] = parents_genes.0[i];
    }
    children_genes
}

fn main() {
    let rosenbrock2d_configuration = RealConfiguration {
        domain: DOMAIN,
        fitness: rosenbrock_2d,
        combat_winner: my_combat,
        pick_action: my_pick_action,
        recombine: |parents_genes, _rng| my_cut_point_recombine(parents_genes, N / 3_usize),
        pick_genes: my_genes_pick,
        mutate_gene: my_mutation,
    };

    let mut list_dir = Command::new("python3");

    // Execute `ls` in the current directory of the program.
    let mut child = list_dir
        .arg("./plotting/live_plotting.py")
        .spawn()
        .expect("process failed to execute");
    sleep(Duration::from_millis(2000));
    let mut system = SystemBuilder::new(rosenbrock2d_configuration)
        .island_amount(17)
        .steps(1_000_000)
        .agents_per_island(20)
        .agent_energy(15)
        .energy_passed_on_reproduction(0.25)
        .combat_energy(3)
        .migration_steps(10_000)
        .migrations_best_amount(4)
        .migrations_elite_amount(2)
        .log_period(Duration::from_secs(5))
        .topology_fn(fully_connected)
        .build(Pcg64Mcg::seed_from_u64(1234));
    let start_time = Instant::now();
    let solution = system.run();
    let end_time = start_time.elapsed().as_secs_f32();
    println!(
        "time = {}, sulution = [{}, {}] => {}",
        end_time,
        solution[0],
        solution[1],
        rosenbrock_2d(&solution)
    );

    sleep(Duration::from_millis(5000));
    child.kill().expect("Can''t kill the child process");
}
