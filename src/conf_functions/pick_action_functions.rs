use crate::Action::{Combat, Reproduce};
use crate::{Action, R};
use rand::Rng;

pub fn default_pick_action(energy: u32, rng: &mut R) -> Action {
    let mut reproduce_with_chance = |chance: f64| {
        if rng.gen::<f64>() < chance {
            Reproduce
        } else {
            Combat
        }
    };

    if energy < 25 {
        return Combat;
    }
    if energy < 50 {
        return reproduce_with_chance(0.25);
    }
    Reproduce
}
