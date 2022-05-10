use std::collections::HashMap;
use std::f64::consts::PI;
use std::hash::Hash;
use rand::{random, Rng, thread_rng};
use rand::prelude::SliceRandom;

pub const RASTRIGIN_DIMS: usize = 2;

pub fn rastrigin(values: &[f64; RASTRIGIN_DIMS]) -> f64 {
    let a: f64 = 10.0;
    let mut fx = a * RASTRIGIN_DIMS as f64;
    fx += values.iter().map(|xi| xi.powf(2.0) - a * (2.0 * PI * xi).cos()).sum::<f64>();
    fx
}

#[derive(Debug)]
struct Agent {
    genes: [f64; RASTRIGIN_DIMS],
    energy: f64,
    // _id: (usize, usize),
}

impl Agent {
    fn rand_agent(starting_energy: f64) -> Agent {
        let mut genes = [0.0; RASTRIGIN_DIMS];
        for gene in genes.iter_mut() {
            *gene = random::<f64>() * 10.24 - 5.12;
        }
        Agent {
            genes,
            energy: starting_energy,
            // _id,
        }
    }


    fn from_parents(parent1: &mut Agent, parent2: &mut Agent, energy_passed: f64) -> Agent {
        let mut genes = [0.0; RASTRIGIN_DIMS];
        for i in 0..RASTRIGIN_DIMS {
            genes[i] = if random::<bool>() { parent1.genes[i] } else { parent2.genes[i] };
        };
        for i in 0..RASTRIGIN_DIMS {
            genes[i] = genes[i] + (random::<f64>() * genes[i] * 0.10 - genes[i] * 0.05);
        }

        let mut energy: f64 = 0.0;
        energy += parent1.energy * energy_passed;
        parent1.energy -= parent1.energy * energy_passed;

        energy += parent2.energy * energy_passed;
        parent2.energy -= parent2.energy * energy_passed;

        Agent {
            genes,
            energy,
            // _id,
        }
    }

    fn fitness(&self) -> f64 {
        rastrigin(&self.genes)
    }

    fn evaluate(&mut self, other: &mut Agent) {
        let d = (self.fitness() - other.fitness()) / (self.fitness() + other.fitness());

        let (loser, winner) = if d >= 0.0 { (self, other) } else { (other, self) };

        let d = d.abs() * loser.energy;

        loser.energy -= d;
        winner.energy += d;

        debug_assert!(loser.energy >= 0.0 && winner.energy >= 0.0);
    }
}

// impl Hash for Agent {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self._id.0.hash(state);
//         self._id.1.hash(state);
//     }
// }

// impl PartialEq for Agent {
//     fn eq(&self, other: &Self) -> bool {
//         self._id.0 == other._id.0 && self._id.1 == other._id.1
//     }
// }

// impl Eq for Agent {}

#[derive(Debug)]
struct Island {
    _id: usize,
    // agents: HashSet<Agent>,
    agents: Vec<Agent>,
    migration_queue: Vec<Agent>,
}

impl Island {
    fn new(agents_amount: usize, id: usize) -> Island {
        let agents: Vec<_> = (0..agents_amount).enumerate()
            .map(|(_, _)| Agent::rand_agent(1.0 / (agents_amount as f64)))
            .collect();

        Island {
            _id: id,
            agents,
            migration_queue: Vec::new(),
        }
    }

    fn step(
        &mut self,
        probabilities: &ActionProbabilities,
        death_level: f64,
        reproduction_level: f64,
        migration_level: f64,
        energy_passed: f64,
    ) {
        let mut action_sorted = (0..self.agents.len())
            .map(|i| (probabilities.randomize_action(), i))
            .fold(
                HashMap::from([
                    (Action::Death, Vec::new()),
                    (Action::Migrations, Vec::new()),
                    (Action::Reproduction, Vec::new()),
                    (Action::Evaluation, Vec::new()),
                ]),
                |mut acc, (action, i)| {
                    acc.get_mut(&action).unwrap().push(i);
                    acc
                },
            );

        self.evaluations(action_sorted.remove(&Action::Evaluation).unwrap());
        self.reproductions(action_sorted.remove(&Action::Reproduction).unwrap(), reproduction_level, energy_passed);
        let mut deaths = action_sorted.remove(&Action::Death).unwrap();
        self.migrations(action_sorted.remove(&Action::Migrations).unwrap(), migration_level, &mut deaths);
        self.deaths(deaths, death_level);
    }

    fn get_pair(&mut self, i: usize, j: usize) -> (&mut Agent, &mut Agent) {
        assert_ne!(i, j);
        let (i, j) = (i.min(j), i.max(j));

        let (a1, a2) = self.agents.split_at_mut(i + 1);
        (&mut a1[i], &mut a2[j - i - 1])
    }

    fn evaluations(&mut self, agents: Vec<usize>) {
        if self.agents.len() <= 1 {
            return;
        }

        let mut rng = thread_rng();
        for i in agents {
            let mut j = rng.gen_range(0..self.agents.len());
            while j == i { j = rng.gen_range(0..self.agents.len()); };


            let (a1, a2) = self.get_pair(i, j);
            a1.evaluate(a2);
        }
    }

    fn migrations(&mut self, mut agents: Vec<usize>, migration_level: f64, deaths_queue: &mut Vec<usize>) {
        // agents
        //     .into_iter()
        //     .filter(|&i| self.agents[i].energy >= migration_level).collect::<Vec<_>>().iter()
        //     .for_each(|&i| {
        //         let agent = self.agents.remove(i);
        //         self.migrate(agent)
        //     });

        for i in 0..agents.len() {
            if self.agents[agents[i]].energy >= migration_level {
                let agent = self.agents.remove(agents[i]);
                self.migrate(agent);

                for j in 0..agents.len() {
                    if agents[j] > agents[i] {
                        agents[j] -= 1;
                    }
                }

                for j in 0..deaths_queue.len() {
                    if deaths_queue[j] > agents[i] {
                        deaths_queue[j] -= 1;
                    }
                }
            }
        }
    }

    fn reproductions(&mut self, agents: Vec<usize>, reproduction_level: f64, energy_passed: f64) {
        let mut rng = thread_rng();

        let mut agents = agents
            .into_iter()
            .filter(|&i| self.agents[i].energy >= reproduction_level)
            .collect::<Vec<_>>();

        agents.shuffle(&mut rng);

        if agents.len() <= 1 {
            return;
        }

        for i in 0..agents.len() / 2 {
            let i = 2 * i;
            let j = i + 1;

            let (a1, a2) = self.get_pair(i, j);

            let new = Agent::from_parents(a1, a2, energy_passed);
            self.agents.push(new);
        }
    }

    fn deaths(&mut self, mut agents: Vec<usize>, death_level: f64) {
        // for i in agents {
        //     if self.agents[i].energy < death_level {
        //         self.agents.remove(i);
        //     }
        // }

        for i in 0..agents.len() {
            if self.agents[agents[i]].energy < death_level {
                self.agents.remove(agents[i]);

                for j in 0..agents.len() {
                    if agents[j] > agents[i] {
                        agents[j] -= 1;
                    }
                }
            }
        }
    }

    fn migrate(&mut self, agent: Agent) {
        self.migration_queue.push(agent);
    }
}


#[derive(Copy, Clone, Debug)]
pub struct ActionProbabilities {
    death: f64,
    migration: f64,
    reproduction: f64,
    evaluation: f64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Action {
    Death,
    Migrations,
    Reproduction,
    Evaluation,
}


impl ActionProbabilities {
    pub fn new(death: f64, migration: f64, reproduction: f64, evaluation: f64) -> ActionProbabilities {
        assert_eq!((death + migration + reproduction + evaluation), 1.0);
        ActionProbabilities {
            death,
            migration,
            reproduction,
            evaluation,
        }
    }

    fn death_threshold(&self) -> f64 {
        self.death
    }

    fn migration_threshold(&self) -> f64 {
        self.death_threshold() + self.migration
    }

    fn reproduction_threshold(&self) -> f64 {
        self.migration_threshold() + self.reproduction
    }
    #[allow(dead_code)]
    fn evaluation_threshold(&self) -> f64 {
        self.reproduction_threshold() + self.evaluation
    }

    fn randomize_action(&self) -> Action {
        let r = random::<f64>();
        return if r < self.death_threshold() {
            Action::Death
        } else if r < self.migration_threshold() {
            Action::Migrations
        } else if r < self.reproduction_threshold() {
            Action::Reproduction
        } else {
            Action::Evaluation
        };
    }
}

#[derive(Debug)]
pub struct System {
    islands: Vec<Island>,
    reproduction_level: f64,
    death_level: f64,
    migration_level: f64,
    energy_passed_on_reproduction: f64,
    probabilities: ActionProbabilities,
}

impl System {
    fn migrate_agents(&mut self) {
        let mut rng = rand::thread_rng();
        let len = self.islands.len();

        let mut push_queue = Vec::new();

        for (i, island) in self.islands.iter_mut().enumerate() {
            while let Some(agent) = island.migration_queue.pop() {
                let mut new = rng.gen_range(0..len);
                while new == i { new = rng.gen_range(0..len); }
                push_queue.push((new, agent));
            }
        }

        push_queue
            .into_iter()
            .for_each(
                |(i, agent)| self.islands[i].agents.push(agent)
            );
    }

    pub fn best_sol(&self) -> [f64; RASTRIGIN_DIMS] {
        self.islands
            .iter()
            .flat_map(|island| island.agents.iter())
            .min_by(|a1, a2| a1.fitness().partial_cmp(&a2.fitness()).unwrap())
            .map(|agent| agent.genes.clone())
            .unwrap()
    }

    pub fn run(&mut self) -> [f64; RASTRIGIN_DIMS] {
        for _ in 0..10_000 {
            for island in self.islands.iter_mut() {
                island.step(
                    &self.probabilities,
                    self.death_level,
                    self.reproduction_level,
                    self.migration_level,
                    self.energy_passed_on_reproduction,
                );
            }
            self.migrate_agents();
        }

        self.best_sol()
    }
}

pub struct SystemBuilder {
    energy_passed_on_reproduction: f64,
    island_amount: usize,
    agents_per_island: usize,
    death_ratio: f64,
    migration_ratio: f64,
    reproduction_ratio: f64,
    probabilities: ActionProbabilities,
}

impl SystemBuilder {
    pub fn new() -> SystemBuilder {
        SystemBuilder {
            island_amount: 10,
            agents_per_island: 100,
            energy_passed_on_reproduction: 0.25,
            death_ratio: 0.0,
            migration_ratio: 2.0,
            reproduction_ratio: 1.5,
            probabilities: ActionProbabilities {
                death: 0.1,
                migration: 0.1,
                reproduction: 0.3,
                evaluation: 0.5,
            },
        }
    }

    fn death_level(&self) -> f64 {
        self.death_ratio * self.avg_energy_per_agent()
    }

    fn migration_level(&self) -> f64 {
        self.migration_ratio * self.avg_energy_per_agent()
    }

    fn reproduction_level(&self) -> f64 {
        self.reproduction_ratio * self.avg_energy_per_agent()
    }

    fn avg_energy_per_agent(&self) -> f64 {
        1.0 / self.agents_per_island as f64
    }

    pub fn island_amount(mut self, amount: usize) -> Self {
        self.island_amount = amount;
        self
    }

    pub fn agents_per_island(mut self, amount: usize) -> Self {
        self.agents_per_island = amount;
        self
    }

    pub fn energy_passed_on_reproduction(mut self, ratio: f64) -> Self {
        assert!(0.0 < ratio && ratio <= 1.0);
        self.energy_passed_on_reproduction = ratio;
        self
    }

    pub fn death_ratio(mut self, ratio: f64) -> Self {
        assert!(0.0 < ratio && ratio <= 1.0);
        self.death_ratio = ratio;
        self
    }

    pub fn migration_ratio(mut self, ratio: f64) -> Self {
        assert!(0.0 < ratio && ratio <= 1.0);
        self.migration_ratio = ratio;
        self
    }

    pub fn reproduction_ratio(mut self, ratio: f64) -> Self {
        assert!(0.0 < ratio && ratio <= 1.0);
        self.reproduction_ratio = ratio;
        self
    }

    pub fn probabilities(mut self, ratios: ActionProbabilities) -> Self {
        assert_eq!(ratios.death + ratios.evaluation + ratios.migration + ratios.reproduction, 1.0);
        self.probabilities = ratios;
        self
    }

    pub fn build(self) -> System {
        let islands = (0..self.island_amount)
            .map(|i| Island::new(self.agents_per_island, i))
            .collect();

        System {
            islands,
            energy_passed_on_reproduction: self.energy_passed_on_reproduction,
            death_level: self.death_level(),
            migration_level: self.migration_level(),
            reproduction_level: self.reproduction_level(),
            probabilities: self.probabilities,
        }
    }
}


// O(1): insert(), remove(), contains(), get_random()
struct HashSetGetRandom<T> {
    map: HashMap<T, usize>,
    vec: Vec<T>,
}

#[allow(dead_code)]
impl<T: Clone + Eq + Hash> HashSetGetRandom<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    fn insert(&mut self, elem: T) -> bool {
        if !self.map.contains_key(&elem) {
            self.map.insert(elem.clone(), self.vec.len());
            self.vec.push(elem);
            return true;
        }
        false
    }

    fn remove(&mut self, elem: &T) -> bool {
        match self.map.remove(elem) {
            Some(index) => {
                self.vec.swap_remove(index);
                if index < self.vec.len() {
                    self.map.insert(self.vec[index].clone(), index);
                }
                true
            }
            None => false,
        }
    }

    fn contains(&self, elem: &T) -> bool {
        self.map.contains_key(elem)
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn get_random(&self) -> &T {
        &self.vec[thread_rng().gen_range(0..self.vec.len())]
    }

    fn get_random_mut(&mut self) -> &mut T {
        let rand_idx = thread_rng().gen_range(0..self.vec.len());
        &mut self.vec[rand_idx]
    }

    fn remove_random(&mut self) -> T {
        let el = self.get_random().clone(); // todo: nie wiem jak inaczej niż clone
        self.remove(&el);
        el
    }
}

impl<T: Clone + Eq + Hash> FromIterator<T> for HashSetGetRandom<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = HashSetGetRandom::new();
        for i in iter {
            c.insert(i);
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use crate::{rastrigin, SystemBuilder, HashSetGetRandom};

    #[test]
    fn rastrigin_min_test() {
        assert_eq!(rastrigin(&[0.0, 0.0]), 0.0)
    }


    #[test]
    fn general_test() {
        let mut system = SystemBuilder::new()
            .build();
        let sol = system.run();
        println!("[{}, {}]", sol[0], sol[1]);
    }

    #[test]
    fn hash_set_get_random() {
        let mut c: HashSetGetRandom<_> = [2, 1, 3, 7].into_iter().collect();

        assert!(c.contains(&1));
        assert!(!c.contains(&2137));
        assert!(c.remove(&3));
        assert!(c.contains(&2));
        assert!(!c.contains(&3));
        assert!(c.insert(3));
        assert!(c.contains(&3));
        assert!(!c.remove(&2137));

        // test if map gets updated when removing element
        c.remove(&1);
        c.remove(&7);
        assert!(!c.contains(&7));

        // test is_empty()
        assert!(!c.is_empty());
        for el in [2, 1, 3, 7] {
            c.remove(&el);
        }
        assert!(c.is_empty());

        // test remove_random()
        c = [2, 1, 3, 7].into_iter().collect();
        assert!(!c.is_empty());
        let mut el = c.remove_random();
        for _ in 0..3 {
            let new_el = c.remove_random();
            assert_ne!(el, new_el);
            el = new_el;
        }
        assert!(c.is_empty());
    }
}

//  Energy is transferred between agents in the process of evaluation.
//When the agent finds out that one of its neighbours (e.g. randomly chosen),
//has lower fitness, it takes a part of its neighbour’s energy,
//otherwise it passes part of its own energy to the evaluated neighbour.
//  The level of life energy triggers the following actions:
//- Reproduction (energy > reproduction level)
//- Death (energy < death level)
//- Migration (energy > migration level)
// Each action is attempted randomly with a certain probability,
// and it is performed only when their basic preconditions are met
// (e.g. an agent may attempt to perform the action of reproduction,
// but it will reproduce only if its energy rises above certain level
// and it meets an appropriate neighbour).

//evaluation of agents, or more generally, the way a phenotype
// (behaviour of the agent) is developed from a genotype (inherited information)
// depends on its interaction with the environment, like in co- evolutionary algorithms.
//???? o to trzeba zapytac
//

