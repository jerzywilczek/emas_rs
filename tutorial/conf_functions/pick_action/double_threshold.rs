pub fn double_threshold_pick_action(
    energy: u32,
    rng: &mut R,
    thresholds: (u32, u32),
    reproduction_chances: (f64, f64),
) -> Action