use crate::R;
use rand::Rng;

pub fn default_combat(this_agent_fitness: f64, other_agent_fitness: f64, rng: &mut R) -> bool {
    let mut win_chance = 0.2;
    if this_agent_fitness < other_agent_fitness {
        win_chance = 0.8;
    }
    if rng.random::<f64>() < win_chance {
        return true;
    }
    false
}
