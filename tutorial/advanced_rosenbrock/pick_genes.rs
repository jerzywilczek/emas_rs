pub fn my_pick_genes<'a, const N: usize, T: Gene>(
    genes: &'a mut [T; N],
    domain: &[(T, T); N],
    rng: &mut R,
) -> Vec<(&'a mut T, (T, T))> {
    let mut mutated_bits_count: usize = (0.25 * genes.len() as f64) as usize;

    let picked_genes: Vec<(&mut T, (T, T))> = genes
        .iter_mut()
        .zip(*domain)
        .choose_multiple(rng, mutated_bits_count);
    picked_genes
}