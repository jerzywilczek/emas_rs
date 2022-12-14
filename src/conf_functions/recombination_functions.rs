use crate::R;
use rand::Rng;

pub fn random_cut_point_recombination<const N: usize>(
    parents_genes: (&[f64; N], &[f64; N]),
    rng: &mut R,
) -> ([f64; N], [f64; N]) {
    let cut_point = rng.gen_range(0..N);
    let mut children_genes = ([0.0; N], [0.0; N]);

    for i in 0..cut_point {
        children_genes.0[i] = parents_genes.0[i];
        children_genes.1[i] = parents_genes.1[i];
    }
    for i in cut_point..N {
        children_genes.0[i] = parents_genes.1[i];
        children_genes.1[i] = parents_genes.0[i];
    }
    children_genes
}
