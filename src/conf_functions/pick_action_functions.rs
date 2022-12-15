use crate::{Action, R};
use rand::Rng;

pub fn default_pick_action(energy: u32, rng: &mut R) -> Action {
    let mut reproduce_with_chance = |chance: f64| {
        if rng.gen::<f64>() < chance {
            Action::Reproduce
        } else {
            Action::Combat
        }
    };

    if energy < 25 {
        return Action::Combat;
    }
    if energy < 50 {
        return reproduce_with_chance(0.25);
    }
    Action::Reproduce
}
