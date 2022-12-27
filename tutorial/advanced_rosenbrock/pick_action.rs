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