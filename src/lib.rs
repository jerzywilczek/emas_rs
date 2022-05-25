use rand::prelude::SliceRandom;
use rand::{random, seq::IteratorRandom, thread_rng, Rng};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::hash::Hash;

pub const RASTRIGIN_DIMS: usize = 2;

pub fn rastrigin(values: &[f64; RASTRIGIN_DIMS]) -> f64 {
    let a: f64 = 10.0;
    let mut fx = a * RASTRIGIN_DIMS as f64;
    fx += values
        .iter()
        .map(|xi| xi.powf(2.0) - a * (2.0 * PI * xi).cos())
        .sum::<f64>();
    fx
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AgentId(usize, usize);

#[derive(Debug, Clone)]
struct Agent {
    genes: [f64; RASTRIGIN_DIMS],
    energy: u32,
    _id: AgentId,
}

impl Agent {
    fn new(starting_energy: u32, id: AgentId) -> Agent {
        Agent {
            genes: [0.0; RASTRIGIN_DIMS],
            energy: starting_energy,
            _id: id,
        }
    }

    fn rand_agent(starting_energy: u32, id: AgentId) -> Agent {
        let mut genes = [0.0; RASTRIGIN_DIMS];
        for gene in genes.iter_mut() {
            *gene = random::<f64>() * 10.24 - 5.12;
        }
        Agent {
            genes,
            energy: starting_energy,
            _id: id,
        }
    }

    fn reproduce(
        &mut self,
        other: &mut Agent,
        energy_passed_percent: f64,
        ch1_id: AgentId,
        ch2_id: AgentId,
    ) -> (Agent, Agent) {
        let par1_en = (energy_passed_percent * self.energy as f64) as u32;
        let par2_en = (energy_passed_percent * other.energy as f64) as u32;
        self.energy -= par1_en;
        other.energy -= par2_en;

        let mut ch1 = Agent::new((par1_en + par2_en) / 2, ch1_id);
        let mut ch2 = Agent::new((par1_en + par2_en + 1) / 2, ch2_id);

        let cut_point = thread_rng().gen_range(0..self.genes.len());
        for i in 0..cut_point {
            ch1.genes[i] = self.genes[i];
            ch2.genes[i] = other.genes[i];
        }
        for i in cut_point..self.genes.len() {
            ch1.genes[i] = other.genes[i];
            ch1.genes[i] = self.genes[i];
        }

        (ch1, ch2)
    }

    fn combat(&mut self, other: &mut Agent, energy: u32) {
        let (winner, looser) = if self.fitness() <= other.fitness() {
            (self, other)
        } else {
            (other, self)
        };
        let energy = energy.min(looser.energy);
        looser.energy -= energy;
        winner.energy += energy;
    }

    fn fitness(&self) -> f64 {
        rastrigin(&self.genes)
    }

    fn pick_action(&self, reproduction_level: u32, combat_level: u32) -> Action {
        if self.energy >= combat_level {
            if self.energy >= reproduction_level {
                let mut rng = thread_rng();
                return if rng.gen::<bool>() {
                    Action::Reproduce
                } else {
                    Action::Combat
                };
            }
            return Action::Combat;
        }
        Action::Idle
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        self._id.0 == other._id.0 && self._id.1 == other._id.1
    }
}

impl Eq for Agent {}

enum Action {
    Combat,
    Reproduce,
    Idle,
}

#[derive(Debug)]
struct Island {
    _id: usize,
    agents: HashMap<AgentId, Agent>,
    migration_queue: Vec<Agent>,
    last_agent_id: usize,
    historical_best: Agent,
}

impl Island {
    fn new(agents_amount: usize, agent_energy: u32, id: usize) -> Island {
        let agents: HashMap<AgentId, Agent> = (0..agents_amount)
            .map(|a_id| {
                (
                    AgentId(id, a_id),
                    Agent::new(agent_energy, AgentId(id, a_id)),
                )
            })
            .collect();

        let historical_best = agents.get(&AgentId(0, 0)).unwrap().clone();
        Island {
            _id: id,
            agents,
            migration_queue: Vec::new(),
            last_agent_id: agents_amount - 1,
            historical_best,
        }
    }

    fn new_agent_id(&mut self) -> usize {
        self.last_agent_id += 1;
        self.last_agent_id
    }

    fn step(
        &mut self,
        reproduction_level: u32,
        combat_level: u32,
        energy_reproduction_percent: f64,
        energy_combat: u32,
    ) {
        let mut to_reproduction = Vec::new();
        let mut to_combat = Vec::new();

        for (_, agent) in self.agents.iter_mut() {
            match agent.pick_action(reproduction_level, combat_level) {
                Action::Reproduce => to_reproduction.push(agent),
                Action::Combat => to_combat.push(agent),
                Action::Idle => (),
            }
        }

        self.reproductions(to_reproduction, energy_reproduction_percent);
        self.combats(to_combat, energy_combat);
        self.deaths();
    }

    fn reproductions(&mut self, mut agents: Vec<&mut Agent>, energy_passed_percent: f64) {
        agents.shuffle(&mut thread_rng());
        while agents.len() >= 2 {
            let mut a1 = agents.pop().unwrap();
            let mut a2 = agents.pop().unwrap();
            let ch1_id = AgentId(self._id, self.new_agent_id());
            let ch2_id = AgentId(self._id, self.new_agent_id());
            let offspring = a1.reproduce(
                &mut a2,
                energy_passed_percent,
                ch1_id.clone(),
                ch2_id.clone(),
            );

            if offspring.0.fitness() > self.historical_best.fitness() {
                self.historical_best = offspring.0.clone();
            }
            if offspring.1.fitness() > self.historical_best.fitness() {
                self.historical_best = offspring.1.clone();
            }
            self.agents.insert(ch1_id, offspring.0);
            self.agents.insert(ch2_id, offspring.1);
        }
    }

    fn combats(&mut self, mut agents: Vec<&mut Agent>, energy: u32) {
        agents.shuffle(&mut thread_rng());
        while agents.len() >= 2 {
            let mut a1 = agents.pop().unwrap();
            let mut a2 = agents.pop().unwrap();
            a1.combat(&mut a2, energy);
        }
    }

    fn deaths(&mut self) {
        let to_remove: Vec<_> = self
            .agents
            .iter()
            .filter(|(_, agent)| agent.energy == 0)
            .map(|(id, agent)| id.clone())
            .collect();

        for id in to_remove.iter() {
            self.agents.remove(id);
        }
    }

    fn step_migrations(&mut self, best_amount: usize, elite_amount: usize) {
        let mut candidates: Vec<_> = self.agents.iter().collect::<Vec<_>>();
        candidates.sort_by(|a1, a2| a1.1.energy.cmp(&a2.1.energy));
        let best_amount = best_amount.min(candidates.len());
        let elite_amount = elite_amount.min(best_amount);
        let best = &candidates[..best_amount];

        let elite = best.iter().choose_multiple(&mut thread_rng(), elite_amount);
        for (id, agent) in elite {
            self.migration_queue.push(self.agents.remove(id).unwrap());
        }
    }
}

#[derive(Debug)]
pub struct System {
    islands: Vec<Island>,
    reproduction_level: u32,
    combat_level: u32,
    energy_reproduction_percent: f64,
    energy_combat: u32,
    migration_steps: u32,
}

impl System {
    fn migrate_agents(&mut self) {
        let mut rng = rand::thread_rng();
        let len = self.islands.len();

        let mut push_queue = Vec::new();

        for (i, island) in self.islands.iter_mut().enumerate() {
            while let Some(agent) = island.migration_queue.pop() {
                let mut new = rng.gen_range(0..len);
                while new == i {
                    new = rng.gen_range(0..len);
                }
                push_queue.push((new, agent));
            }
        }

        for (island_id, agent) in push_queue {
            self.islands[island_id].agents.insert(agent._id, agent);
        }
    }

    pub fn best_sol(&self) -> [f64; RASTRIGIN_DIMS] {
        self.islands
            .iter()
            .min_by(|island1, island2| {
                island1
                    .historical_best
                    .fitness()
                    .partial_cmp(&island2.historical_best.fitness())
                    .unwrap()
            })
            .unwrap()
            .historical_best
            .genes
    }

    pub fn run(&mut self) -> [f64; RASTRIGIN_DIMS] {
        for _ in 0..10_000 {
            for island in self.islands.iter_mut() {
                island.step(
                    self.reproduction_level,
                    self.combat_level,
                    self.energy_reproduction_percent,
                    self.energy_combat,
                );
            }
            self.migrate_agents();
        }

        self.best_sol()
    }
}

pub struct SystemBuilder {
    island_amount: usize,
    agents_per_island: usize,
    agent_energy: u32,
    reproduction_level: u32,
    combat_level: u32,
    energy_passed_on_reproduction: f64,
    energy_combat: u32,
    migration_steps: u32,
}

impl SystemBuilder {
    pub fn new() -> SystemBuilder {
        SystemBuilder {
            island_amount: 10,
            agents_per_island: 100,
            agent_energy: 10,
            reproduction_level: 20,
            combat_level: 15,
            energy_passed_on_reproduction: 0.25,
            energy_combat: 2,
            migration_steps: 50,
        }
    }

    pub fn island_amount(mut self, amount: usize) -> Self {
        self.island_amount = amount;
        self
    }

    pub fn agents_per_island(mut self, amount: usize) -> Self {
        self.agents_per_island = amount;
        self
    }

    pub fn agent_energy(mut self, amount: u32) -> Self {
        self.agent_energy = amount;
        self
    }

    pub fn reproduction_level(mut self, amount: u32) -> Self {
        self.reproduction_level = amount;
        self
    }

    pub fn combat_level(mut self, amount: u32) -> Self {
        self.combat_level = amount;
        self
    }

    pub fn energy_passed_on_reproduction(mut self, ratio: f64) -> Self {
        assert!(0.0 < ratio && ratio <= 1.0);
        self.energy_passed_on_reproduction = ratio;
        self
    }

    pub fn combat_energy(mut self, amount: u32) -> Self {
        self.energy_combat = amount;
        self
    }

    pub fn build(self) -> System {
        let islands = (0..self.island_amount)
            .map(|id| Island::new(self.agents_per_island, self.agent_energy, id))
            .collect();

        System {
            islands,
            reproduction_level: self.reproduction_level,
            combat_level: self.combat_level,
            energy_reproduction_percent: self.energy_passed_on_reproduction,
            energy_combat: self.energy_combat,
            migration_steps: self.migration_steps,
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

    fn len(&self) -> usize {
        self.vec.len()
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
    use crate::{rastrigin, HashSetGetRandom};

    #[test]
    fn rastrigin_min_test() {
        assert_eq!(rastrigin(&[0.0, 0.0]), 0.0)
    }

    // #[test]
    // fn general_test() {
    //     let mut system = SystemBuilder::new().build();
    //     let sol = system.run();
    //     println!("[{}, {}]", sol[0], sol[1]);
    // }

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
