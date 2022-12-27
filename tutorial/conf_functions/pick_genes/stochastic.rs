pub fn stochastic_pick<'a, const N: usize, T: Gene>(
    genes: &'a mut [T; N],
    domain: &[(T, T); N],
    rng: &mut R,
    mutation_chance: f64,
) -> Vec<(&'a mut T, (T, T))>