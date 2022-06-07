use rand::prelude::SliceRandom;
use rand::{random, seq::IteratorRandom, thread_rng, Rng};
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AgentId(usize, usize);

#[derive(Debug, Clone)]
struct Agent {
    genes: [f64; RASTRIGIN_DIMS],
    energy: u32,
    id: AgentId,
    fitness: f64,
}

fn combat_win_chance(fit1: f64, fit2: f64) -> f64 {
    if fit1 >= fit2 {
        return 0.8;
    }
    0.2
}

impl Agent {
    fn rand_agent(starting_energy: u32, id: AgentId) -> Agent {
        let mut genes = [0.0; RASTRIGIN_DIMS];
        for gene in genes.iter_mut() {
            *gene = random::<f64>() * 10.24 - 5.12;
        }
        Agent {
            genes,
            energy: starting_energy,
            id,
            fitness: rastrigin(&genes),
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

        let mut ch1 = Agent {
            energy: (par1_en + par2_en) / 2,
            id: ch1_id,
            genes: [0.0, 0.0],
            fitness: 0.0,
        };
        let mut ch2 = Agent {
            energy: (par1_en + par2_en + 1) / 2,
            id: ch2_id,
            genes: [0.0, 0.0],
            fitness: 0.0,
        };

        let cut_point = thread_rng().gen_range(0..self.genes.len());
        for i in 0..cut_point {
            ch1.genes[i] = self.genes[i];
            ch2.genes[i] = other.genes[i];
        }
        for i in cut_point..self.genes.len() {
            ch1.genes[i] = other.genes[i];
            ch1.genes[i] = self.genes[i];
        }

        ch1.fitness = rastrigin(&ch1.genes);
        ch2.fitness = rastrigin(&ch2.genes);

        (ch1, ch2)
    }

    fn combat(&mut self, other: &mut Agent, energy: u32, win_chance_fn: fn(f64, f64) -> f64) {
        let (winner, looser) =
            if thread_rng().gen::<f64>() < win_chance_fn(self.fitness, other.fitness) {
                (self, other)
            } else {
                (other, self)
            };
        let energy = energy.min(looser.energy);
        looser.energy -= energy;
        winner.energy += energy;
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
        self.id.0 == other.id.0 && self.id.1 == other.id.1
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
                    Agent::rand_agent(agent_energy, AgentId(id, a_id)),
                )
            })
            .collect();

        let historical_best = agents.get(&AgentId(id, 0)).unwrap().clone();
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

    fn get_pair_mut(&mut self, a1_id: &AgentId, a2_id: &AgentId) -> (&mut Agent, &mut Agent) {
        assert_ne!(a1_id, a2_id);

        let a1 = self.agents.get_mut(&a1_id).unwrap() as *mut Agent;
        let a2 = self.agents.get_mut(&a2_id).unwrap() as *mut Agent;
        unsafe { (&mut *a1, &mut *a2) }
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

        for (&id, agent) in self.agents.iter_mut() {
            match agent.pick_action(reproduction_level, combat_level) {
                Action::Reproduce => to_reproduction.push(id),
                Action::Combat => to_combat.push(id),
                Action::Idle => (),
            }
        }

        self.reproductions(to_reproduction, energy_reproduction_percent);
        self.combats(to_combat, energy_combat);
        self.deaths();
    }

    fn reproductions(&mut self, mut agents: Vec<AgentId>, energy_passed_percent: f64) {
        agents.shuffle(&mut thread_rng());
        while agents.len() >= 2 {
            let a1_id = agents.pop().unwrap();
            let a2_id = agents.pop().unwrap();

            let ch1_id = AgentId(self._id, self.new_agent_id());
            let ch2_id = AgentId(self._id, self.new_agent_id());

            let (a1, a2) = self.get_pair_mut(&a1_id, &a2_id);


            let offspring = a1.reproduce(
                a2,
                energy_passed_percent,
                ch1_id.clone(),
                ch2_id.clone(),
            );

            if offspring.0.fitness > self.historical_best.fitness {
                self.historical_best = offspring.0.clone();
            }
            if offspring.1.fitness > self.historical_best.fitness {
                self.historical_best = offspring.1.clone();
            }
            self.agents.insert(ch1_id, offspring.0);
            self.agents.insert(ch2_id, offspring.1);
        }
    }

    fn combats(&mut self, mut agents: Vec<AgentId>, energy: u32) {
        agents.shuffle(&mut thread_rng());
        while agents.len() >= 2 {
            let a1_id = agents.pop().unwrap();
            let a2_id = agents.pop().unwrap();

            let (a1, a2) = self.get_pair_mut(&a1_id, &a2_id);

            a1.combat(a2, energy, combat_win_chance);
        }
    }

    fn deaths(&mut self) {
        let to_remove: Vec<_> = self
            .agents
            .iter()
            .filter(|(_, agent)| agent.energy == 0)
            .map(|(id, _)| id.clone())
            .collect();

        for id in to_remove.iter() {
            self.agents.remove(id);
        }
    }

    fn step_migrations(&mut self, best_amount: usize, elite_amount: usize) {
        let mut candidates: Vec<_> = self.agents.keys().copied().collect::<Vec<_>>();
        candidates.sort_by(|a1, a2| self.agents.get(a1).unwrap().energy.cmp(&self.agents.get(a2).unwrap().energy));
        let best_amount = best_amount.min(candidates.len());
        let elite_amount = elite_amount.min(best_amount);
        let best = &candidates[..best_amount];

        let elite = best.iter().choose_multiple(&mut thread_rng(), elite_amount);
        for id in elite {
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
    migrations_best_amount: usize,
    migrations_elite_amount: usize,
}

impl System {
    fn migrate_agents(&mut self) {
        let mut rng = thread_rng();
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
            self.islands[island_id].agents.insert(agent.id, agent);
        }
    }

    pub fn best_sol(&self) -> [f64; RASTRIGIN_DIMS] {
        self.islands
            .iter()
            .min_by(|island1, island2| {
                island1
                    .historical_best
                    .fitness
                    .partial_cmp(&island2.historical_best.fitness)
                    .unwrap()
            })
            .unwrap()
            .historical_best
            .genes
    }

    pub fn run(&mut self) -> [f64; RASTRIGIN_DIMS] {
        for i in 0..10_000 {
            for island in self.islands.iter_mut() {
                island.step(
                    self.reproduction_level,
                    self.combat_level,
                    self.energy_reproduction_percent,
                    self.energy_combat,
                );
            }

            if i % self.migration_steps == 0 {
                for island in self.islands.iter_mut() {
                    island.step_migrations(self.migrations_best_amount, self.migrations_elite_amount);
                }
                self.migrate_agents();
            }
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
    migrations_best_amount: usize,
    migrations_elite_amount: usize,
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
            migrations_best_amount: 10,
            migrations_elite_amount: 5,
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

    pub fn migrations_best_amount(mut self, amount: usize) -> Self {
        self.migrations_best_amount = amount;
        self
    }

    pub fn migrations_elite_amount(mut self, amount: usize) -> Self {
        self.migrations_elite_amount = amount;
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
            migrations_best_amount: self.migrations_best_amount,
            migrations_elite_amount: self.migrations_elite_amount,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{rastrigin};

    #[test]
    fn rastrigin_min_test() {
        assert_eq!(rastrigin(&[0.0, 0.0]), 0.0)
    }
}