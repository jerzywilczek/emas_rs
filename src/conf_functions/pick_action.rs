use crate::{Action, R};
use rand::Rng;

pub fn double_threshold_pick_action(
    energy: u32,
    rng: &mut R,
    thresholds: (u32, u32),
    reproduction_chances: (f64, f64),
) -> Action {
    let mut reproduce_with_chance = |chance: f64| {
        if rng.gen::<f64>() < chance {
            Action::Reproduce
        } else {
            Action::Combat
        }
    };

    if energy < thresholds.0 {
        return reproduce_with_chance(reproduction_chances.0);
    }
    if energy < thresholds.1 {
        return reproduce_with_chance(reproduction_chances.1);
    }
    Action::Reproduce
}

pub fn default_pick_action(energy: u32, rng: &mut R) -> Action {
    double_threshold_pick_action(energy, rng, (25, 50), (0.0, 0.25))
}
