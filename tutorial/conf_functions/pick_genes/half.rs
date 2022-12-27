pub fn half_pick<'a, const N: usize, T: Gene>(
    genes: &'a mut [T; N],
    domain: &[(T, T); N],
    rng: &mut R,
) -> Vec<(&'a mut T, (T, T))>