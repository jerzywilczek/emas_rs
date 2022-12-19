use conf_functions::topologies;
use rand::prelude::SliceRandom;
use rand::{seq::IteratorRandom, Rng, SeedableRng};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub mod conf_functions;

type R = rand_chacha::ChaCha12Rng;

type Configuration<const N: usize, T> =
    dyn conf_functions::Configuration<N, T> + Send + Sync + 'static;

pub trait Gene: Default + Copy + Send {}
impl<T: Default + Copy + Send> Gene for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AgentId(usize, usize);

#[derive(Debug)]
struct Agent<const N: usize, T: Gene> {
    genes: [T; N],
    energy: u32,
    id: AgentId,
    fitness: f64,
}

impl<const N: usize, T: Gene> Clone for Agent<N, T> {
    fn clone(&self) -> Self {
        Self {
            genes: self.genes,
            energy: self.energy,
            id: self.id,
            fitness: self.fitness,
        }
    }
}

impl<const N: usize, T: Gene> Agent<N, T> {
    fn rand_agent(
        starting_energy: u32,
        id: AgentId,
        conf: &Configuration<N, T>,
        rng: &mut R,
    ) -> Agent<N, T> {
        let genes = conf.random_genes(rng);
        Agent {
            genes,
            energy: starting_energy,
            id,
            fitness: conf.fitness(&genes),
        }
    }

    fn reproduce(
        &mut self,
        other: &mut Agent<N, T>,
        energy_passed_percent: f64,
        child_ids: (AgentId, AgentId),
        conf: &Configuration<N, T>,
        rng: &mut R,
    ) -> (Agent<N, T>, Agent<N, T>) {
        let par1_en = (energy_passed_percent * self.energy as f64) as u32;
        let par2_en = (energy_passed_percent * other.energy as f64) as u32;
        self.energy -= par1_en;
        other.energy -= par2_en;

        let children_genes: ([T; N], [T; N]) = conf.recombine((&self.genes, &other.genes), rng);

        let mut ch1 = Agent {
            energy: (par1_en + par2_en) / 2,
            id: child_ids.0,
            genes: children_genes.0,
            fitness: 0.0,
        };
        let mut ch2 = Agent {
            energy: (par1_en + par2_en + 1) / 2,
            id: child_ids.1,
            genes: children_genes.1,
            fitness: 0.0,
        };

        ch1.mutate(conf, rng);
        ch2.mutate(conf, rng);

        ch1.fitness = conf.fitness(&ch1.genes);
        ch2.fitness = conf.fitness(&ch2.genes);

        (ch1, ch2)
    }

    fn mutate(&mut self, conf: &Configuration<N, T>, rng: &mut R) {
        conf.mutate(&mut self.genes, rng);
    }

    fn combat(
        &mut self,
        other: &mut Agent<N, T>,
        energy: u32,
        conf: &Configuration<N, T>,
        rng: &mut R,
    ) {
        let (winner, looser) = if conf.combat_winner(self.fitness, other.fitness, rng) {
            (self, other)
        } else {
            (other, self)
        };
        let energy = energy.min(looser.energy);
        looser.energy -= energy;
        winner.energy += energy;
    }

    fn pick_action(&self, conf: &Configuration<N, T>, rng: &mut R) -> Action {
        conf.pick_action(self.energy, rng)
    }
}

impl<const N: usize, T: Gene> PartialEq for Agent<N, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.0 == other.id.0 && self.id.1 == other.id.1
    }
}

impl<const N: usize, T: Gene> Eq for Agent<N, T> {}

pub enum Action {
    Combat,
    Reproduce,
}

#[derive(Debug)]
struct Island<const N: usize, T: Gene> {
    _id: usize,
    agents: HashMap<AgentId, Agent<N, T>>,
    migration_queue: Vec<Agent<N, T>>,
    last_agent_id: usize,
    historical_best: Agent<N, T>,
    conf: Arc<Configuration<N, T>>,
    rng: R,
}

impl<const N: usize, T: Gene> Island<N, T> {
    fn new(
        agents_amount: usize,
        agent_energy: u32,
        id: usize,
        conf: Arc<Configuration<N, T>>,
        rng: impl Rng,
    ) -> Island<N, T> {
        let mut rng = R::from_rng(rng).unwrap();
        let agents: HashMap<AgentId, Agent<N, T>> = (0..agents_amount)
            .map(|a_id| {
                (
                    AgentId(id, a_id),
                    Agent::rand_agent(agent_energy, AgentId(id, a_id), conf.as_ref(), &mut rng),
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
            conf,
            rng,
        }
    }

    fn new_agent_id(&mut self) -> usize {
        self.last_agent_id += 1;
        self.last_agent_id
    }

    fn get_pair_mut<'a>(
        map: &'a mut HashMap<AgentId, Agent<N, T>>,
        a1_id: &AgentId,
        a2_id: &AgentId,
    ) -> (&'a mut Agent<N, T>, &'a mut Agent<N, T>) {
        assert_ne!(a1_id, a2_id);

        let a1 = map.get_mut(a1_id).unwrap() as *mut Agent<N, T>;
        let a2 = map.get_mut(a2_id).unwrap() as *mut Agent<N, T>;
        unsafe { (&mut *a1, &mut *a2) }
    }

    fn step(&mut self, energy_reproduction_percent: f64, energy_combat: u32) {
        let mut to_reproduction = Vec::new();
        let mut to_combat = Vec::new();

        for (&id, agent) in self.agents.iter_mut() {
            match agent.pick_action(self.conf.as_ref(), &mut self.rng) {
                Action::Reproduce => to_reproduction.push(id),
                Action::Combat => to_combat.push(id),
            }
        }

        self.reproductions(to_reproduction, energy_reproduction_percent);
        self.combats(to_combat, energy_combat);
        self.deaths();
    }

    fn reproductions(&mut self, mut agents: Vec<AgentId>, energy_passed_percent: f64) {
        agents.shuffle(&mut self.rng);
        while agents.len() >= 2 {
            let a1_id = agents.pop().unwrap();
            let a2_id = agents.pop().unwrap();

            let ch1_id = AgentId(self._id, self.new_agent_id());
            let ch2_id = AgentId(self._id, self.new_agent_id());

            let (a1, a2) = Self::get_pair_mut(&mut self.agents, &a1_id, &a2_id);

            let offspring = a1.reproduce(
                a2,
                energy_passed_percent,
                (ch1_id, ch2_id),
                self.conf.as_ref(),
                &mut self.rng,
            );

            if offspring.0.fitness < self.historical_best.fitness {
                self.historical_best = offspring.0.clone();
            }
            if offspring.1.fitness < self.historical_best.fitness {
                self.historical_best = offspring.1.clone();
            }
            self.agents.insert(ch1_id, offspring.0);
            self.agents.insert(ch2_id, offspring.1);
        }
    }

    fn combats(&mut self, mut agents: Vec<AgentId>, energy: u32) {
        agents.shuffle(&mut self.rng);
        while agents.len() >= 2 {
            let a1_id = agents.pop().unwrap();
            let a2_id = agents.pop().unwrap();

            let (a1, a2) = Self::get_pair_mut(&mut self.agents, &a1_id, &a2_id);

            a1.combat(a2, energy, self.conf.as_ref(), &mut self.rng);
        }
    }

    fn deaths(&mut self) {
        let to_remove: Vec<_> = self
            .agents
            .iter()
            .filter(|(_, agent)| agent.energy == 0)
            .map(|(id, _)| id)
            .cloned()
            .collect();

        for id in to_remove.iter() {
            self.agents.remove(id);
        }
    }

    fn step_migrations(&mut self, best_amount: usize, elite_amount: usize) {
        let mut candidates: Vec<_> = self.agents.keys().copied().collect::<Vec<_>>();
        candidates.sort_by(|a1, a2| {
            self.agents
                .get(a1)
                .unwrap()
                .energy
                .cmp(&self.agents.get(a2).unwrap().energy)
        });
        let best_amount = best_amount.min(candidates.len());
        let elite_amount = elite_amount.min(best_amount);
        let best = &candidates[..best_amount];

        let elite = best.iter().choose_multiple(&mut self.rng, elite_amount);
        for id in elite {
            self.migration_queue.push(self.agents.remove(id).unwrap());
        }
    }
}

#[derive(Debug)]
pub struct System<const N: usize, T: Gene> {
    islands: Vec<Island<N, T>>,
    steps: u32,
    energy_reproduction_percent: f64,
    energy_combat: u32,
    migration_steps: u32,
    migrations_best_amount: usize,
    migrations_elite_amount: usize,
    logs: Vec<String>,
    log_period: Duration,
    conf: Arc<Configuration<N, T>>,
    topology: topologies::Topology,
    rng: R,
}

impl<const N: usize, T: Gene> System<N, T> {
    fn log(&mut self, start: Instant) -> String {
        let timestamp = start.elapsed().as_secs_f32();
        let historical_best = self.conf.fitness(&self.best_sol());
        let agents_amount = self.islands.iter().map(|i| i.agents.len()).sum::<usize>();
        let energy_sum = self
            .islands
            .iter()
            .map(|i| i.agents.values().map(|a| a.energy).sum::<u32>())
            .sum::<u32>();
        let best_living = self
            .islands
            .iter()
            .flat_map(|i| i.agents.values())
            .min_by(|a1, a2| a1.fitness.partial_cmp(&a2.fitness).unwrap())
            .unwrap()
            .fitness;

        let average_fitness = self
            .islands
            .iter()
            .flat_map(|i| i.agents.values())
            .map(|a| a.fitness)
            .sum::<f64>()
            / agents_amount as f64;

        let average_energy = self
            .islands
            .iter()
            .flat_map(|i| i.agents.values())
            .map(|a| a.energy)
            .sum::<u32>() as f64
            / agents_amount as f64;

        format!(
            "{},{},{},{},{},{},{}\n",
            timestamp,
            historical_best,
            agents_amount,
            energy_sum,
            best_living,
            average_fitness,
            average_energy
        )
    }

    fn migrate_agents(&mut self) {
        let mut push_queue = Vec::new();

        for island in self.islands.iter_mut() {
            let connected_islands = &self.topology[island._id];
            while let Some(agent) = island.migration_queue.pop() {
                let new = match connected_islands.is_empty() {
                    true => island._id,
                    false => connected_islands[self.rng.gen_range(0..connected_islands.len())],
                };
                push_queue.push((new, agent));
            }
        }

        for (island_id, agent) in push_queue {
            self.islands[island_id].agents.insert(agent.id, agent);
        }
    }

    pub fn best_sol(&self) -> [T; N] {
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

    pub fn run(&mut self) -> [T; N] {
        fs::remove_file("outputs.csv").unwrap();
        let mut f = File::create("outputs.csv").unwrap();
        f.write_all(
            self.logs
                .iter()
                .fold(String::new(), |mut s, i| {
                    s.push_str(i);
                    s
                })
                .as_bytes(),
        )
        .unwrap();

        let start = Instant::now();

        let log_notification = crossbeam::channel::tick(self.log_period);

        for _ in 0..self.steps / self.migration_steps {
            self.islands.par_iter_mut().for_each(|island| {
                for _ in 0..self.migration_steps {
                    island.step(self.energy_reproduction_percent, self.energy_combat);
                }
            });

            for island in self.islands.iter_mut() {
                island.step_migrations(self.migrations_best_amount, self.migrations_elite_amount);
            }
            self.migrate_agents();

            if log_notification.try_recv().is_ok() {
                f.write_all(self.log(start).as_bytes())
                    .expect("Can't write logs to the log file");
            }
        }

        self.best_sol()
    }
}

pub struct SystemBuilder<
    const N: usize,
    R: Rng,
    T: Gene,
    C: conf_functions::Configuration<N, T> + Send + Sync + 'static,
> {
    island_amount: usize,
    steps: u32,
    agents_per_island: usize,
    agent_energy: u32,
    energy_passed_on_reproduction: f64,
    energy_combat: u32,
    migration_steps: u32,
    migrations_best_amount: usize,
    migrations_elite_amount: usize,
    log_period: Duration,
    conf: Arc<C>,
    topology_fn: fn(usize) -> topologies::Topology,
    phantom1: PhantomData<T>,
    phantom2: PhantomData<R>,
}

impl<
        const N: usize,
        T: Gene,
        R: Rng,
        C: conf_functions::Configuration<N, T> + Send + Sync + 'static,
    > SystemBuilder<N, R, T, C>
{
    pub fn new(conf: C) -> Self {
        SystemBuilder {
            island_amount: 5,
            agents_per_island: 100,
            steps: 10_000,
            agent_energy: 10,
            energy_passed_on_reproduction: 0.25,
            energy_combat: 2,
            migration_steps: 50,
            migrations_best_amount: 10,
            migrations_elite_amount: 5,
            log_period: Duration::from_secs(1),
            conf: Arc::new(conf),
            topology_fn: topologies::fully_connected,
            phantom1: PhantomData,
            phantom2: PhantomData,
        }
    }

    pub fn island_amount(mut self, amount: usize) -> Self {
        self.island_amount = amount;
        self
    }

    pub fn steps(mut self, amount: u32) -> Self {
        self.steps = amount;
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

    pub fn energy_passed_on_reproduction(mut self, ratio: f64) -> Self {
        assert!(0.0 < ratio && ratio <= 1.0);
        self.energy_passed_on_reproduction = ratio;
        self
    }

    pub fn combat_energy(mut self, amount: u32) -> Self {
        self.energy_combat = amount;
        self
    }

    pub fn migration_steps(mut self, amount: u32) -> Self {
        self.migration_steps = amount;
        self
    }

    pub fn log_period(mut self, amount: Duration) -> Self {
        self.log_period = amount;
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

    pub fn topology_fn(mut self, topology_fn: fn(n: usize) -> topologies::Topology) -> Self {
        self.topology_fn = topology_fn;
        self
    }

    pub fn build(self, mut rng: R) -> System<N, T> {
        // for (i, (d_min, d_max)) in F::DOMAIN.iter().enumerate() {
        //     if d_min > d_max {
        //         panic!("In the domain in argument {}, the first element is larger than the second element, which is not allowed", i)
        //     }
        // }

        if !topologies::is_valid(&(self.topology_fn)(self.island_amount), self.island_amount) {
            panic!("Invalid topology");
        }

        let islands = (0..self.island_amount)
            .map(|id| {
                Island::new(
                    self.agents_per_island,
                    self.agent_energy,
                    id,
                    self.conf.clone(),
                    &mut rng,
                )
            })
            .collect();

        let logs = vec![
            "timestamp,historical best,agents amount,energy sum,best living,average fitness,average energy\n".to_string()
        ];

        System {
            islands,
            steps: self.steps,
            energy_reproduction_percent: self.energy_passed_on_reproduction,
            energy_combat: self.energy_combat,
            migration_steps: self.migration_steps,
            migrations_best_amount: self.migrations_best_amount,
            migrations_elite_amount: self.migrations_elite_amount,
            logs,
            log_period: self.log_period,
            conf: self.conf,
            topology: (self.topology_fn)(self.island_amount),
            rng: crate::R::from_rng(rng).unwrap(),
        }
    }
}
