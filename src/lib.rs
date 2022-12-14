use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand::{seq::IteratorRandom, thread_rng, Rng};
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

type Configuration<const N: usize, T, R> =
    dyn conf_functions::Configuration<N, T, R> + Send + Sync + 'static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AgentId(usize, usize);

// #[derive(Debug)]
struct Agent<const N: usize, T: Default + Copy + Send, R: Rng> {
    genes: [T; N],
    energy: u32,
    id: AgentId,
    fitness: f64,
    phantom: PhantomData<R>,
}

impl<const N: usize, T: Default + Copy + Send, R: Rng> Clone for Agent<N, T, R> {
    fn clone(&self) -> Self {
        Self {
            genes: self.genes,
            energy: self.energy,
            id: self.id,
            fitness: self.fitness,
            phantom: self.phantom,
        }
    }
}

pub struct ReproductionChance(pub f64);

impl<const N: usize, T: Default + Copy + Send, R: Rng> Agent<N, T, R> {
    fn rand_agent(
        starting_energy: u32,
        id: AgentId,
        conf: &Configuration<N, T, R>,
        rng: &mut R,
    ) -> Agent<N, T, R> {
        let genes = conf.random_genes(rng);
        Agent {
            genes,
            energy: starting_energy,
            id,
            fitness: conf.fitness(&genes),
            phantom: PhantomData,
        }
    }

    fn reproduce(
        &mut self,
        other: &mut Agent<N, T, R>,
        energy_passed_percent: f64,
        ch1_id: AgentId,
        ch2_id: AgentId,
        conf: &Configuration<N, T, R>,
        rng: &mut R,
    ) -> (Agent<N, T, R>, Agent<N, T, R>) {
        let par1_en = (energy_passed_percent * self.energy as f64) as u32;
        let par2_en = (energy_passed_percent * other.energy as f64) as u32;
        self.energy -= par1_en;
        other.energy -= par2_en;

        let mut ch1 = Agent {
            energy: (par1_en + par2_en) / 2,
            id: ch1_id,
            genes: [T::default(); N],
            fitness: 0.0,
            phantom: PhantomData,
        };
        let mut ch2 = Agent {
            energy: (par1_en + par2_en + 1) / 2,
            id: ch2_id,
            genes: [T::default(); N],
            fitness: 0.0,
            phantom: PhantomData,
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

        ch1.mutate(conf, rng);
        ch2.mutate(conf, rng);

        ch1.fitness = conf.fitness(&ch1.genes);
        ch2.fitness = conf.fitness(&ch2.genes);

        (ch1, ch2)
    }

    fn mutate(&mut self, conf: &Configuration<N, T, R>, rng: &mut R) {
        conf.mutate(&mut self.genes, rng);
    }

    fn combat(
        &mut self,
        other: &mut Agent<N, T, R>,
        energy: u32,
        conf: &Configuration<N, T, R>,
        rng: &mut R,
    ) {
        let (winner, looser) = if rng.gen::<f64>() < conf.combat(self.fitness, other.fitness) {
            (self, other)
        } else {
            (other, self)
        };
        let energy = energy.min(looser.energy);
        looser.energy -= energy;
        winner.energy += energy;
    }

    fn pick_action(&self, reproduction_chance: ReproductionChance) -> Action {
        let ReproductionChance(reproduction_chance) = reproduction_chance;
        if thread_rng().gen::<f64>() < reproduction_chance {
            Action::Reproduce
        } else {
            Action::Combat
        }
    }
}

impl<const N: usize, T: Default + Copy + Send, R: Rng> PartialEq for Agent<N, T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.id.0 == other.id.0 && self.id.1 == other.id.1
    }
}

impl<const N: usize, T: Default + Copy + Send, R: Rng> Eq for Agent<N, T, R> {}

enum Action {
    Combat,
    Reproduce,
}

// #[derive(Debug)]
struct Island<const N: usize, T: Default + Copy + Send, R: SeedableRng + Rng> {
    _id: usize,
    agents: HashMap<AgentId, Agent<N, T, R>>,
    migration_queue: Vec<Agent<N, T, R>>,
    last_agent_id: usize,
    historical_best: Agent<N, T, R>,
    conf: Arc<Configuration<N, T, R>>,
    rng: R,
}

impl<const N: usize, T: Default + Copy + Send, R: SeedableRng + Rng> Island<N, T, R> {
    fn new(
        agents_amount: usize,
        agent_energy: u32,
        id: usize,
        conf: Arc<Configuration<N, T, R>>,
        rng: impl Rng,
    ) -> Island<N, T, R> {
        let mut rng = R::from_rng(rng).unwrap();
        let agents: HashMap<AgentId, Agent<N, T, R>> = (0..agents_amount)
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
        map: &'a mut HashMap<AgentId, Agent<N, T, R>>,
        a1_id: &AgentId,
        a2_id: &AgentId,
    ) -> (&'a mut Agent<N, T, R>, &'a mut Agent<N, T, R>) {
        assert_ne!(a1_id, a2_id);

        let a1 = map.get_mut(a1_id).unwrap() as *mut Agent<N, T, R>;
        let a2 = map.get_mut(a2_id).unwrap() as *mut Agent<N, T, R>;
        unsafe { (&mut *a1, &mut *a2) }
    }

    fn step(&mut self, energy_reproduction_percent: f64, energy_combat: u32) {
        let mut to_reproduction = Vec::new();
        let mut to_combat = Vec::new();

        for (&id, agent) in self.agents.iter_mut() {
            match agent.pick_action(self.conf.reproduction_chance(agent.energy)) {
                Action::Reproduce => to_reproduction.push(id),
                Action::Combat => to_combat.push(id),
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

            let (a1, a2) = Self::get_pair_mut(&mut self.agents, &a1_id, &a2_id);

            let offspring = a1.reproduce(
                a2,
                energy_passed_percent,
                ch1_id,
                ch2_id,
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
        agents.shuffle(&mut thread_rng());
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

        let elite = best.iter().choose_multiple(&mut thread_rng(), elite_amount);
        for id in elite {
            self.migration_queue.push(self.agents.remove(id).unwrap());
        }
    }
}

// #[derive(Debug)]
pub struct System<const N: usize, T: Default + Copy + Send, R: SeedableRng + Rng> {
    islands: Vec<Island<N, T, R>>,
    steps: u32,
    energy_reproduction_percent: f64,
    energy_combat: u32,
    migration_steps: u32,
    migrations_best_amount: usize,
    migrations_elite_amount: usize,
    logs: Vec<String>,
    log_period: Duration,
    conf: Arc<Configuration<N, T, R>>,
}

impl<const N: usize, T: Default + Copy + Send, R: SeedableRng + Rng + Send> System<N, T, R> {
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
    R: SeedableRng + Rng,
    T: Default + Copy + Send,
    C: conf_functions::Configuration<N, T, R> + Send + Sync + 'static,
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
    phantom1: PhantomData<T>,
    phantom2: PhantomData<R>,
}

impl<
        const N: usize,
        T: Default + Copy + Send,
        R: SeedableRng + Rng,
        C: conf_functions::Configuration<N, T, R> + Send + Sync + 'static,
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

    pub fn build(self, mut rng: impl Rng) -> System<N, T, R> {
        // for (i, (d_min, d_max)) in F::DOMAIN.iter().enumerate() {
        //     if d_min > d_max {
        //         panic!("In the domain in argument {}, the first element is larger than the second element, which is not allowed", i)
        //     }
        // }

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
        }
    }
}
