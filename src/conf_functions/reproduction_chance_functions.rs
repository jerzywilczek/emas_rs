use crate::ReproductionChance;

pub fn default_reproduction_chance(energy: u32) -> ReproductionChance {
    if energy < 25 {
        return ReproductionChance(0.0);
    }
    if energy < 50 {
        return ReproductionChance(0.25);
    }
    ReproductionChance(1.0)
}
