use crate::R;
use rand::Rng;

pub fn cut_point_recombine<const N: usize, T: Default + Copy + Clone>(
    parents_genes: (&[T; N], &[T; N]),
    cut_point: usize,
) -> ([T; N], [T; N]) {
    let mut children_genes = ([T::default(); N], [T::default(); N]);

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

pub fn random_cut_point_recombine<const N: usize, T: Default + Copy + Clone>(
    parents_genes: (&[T; N], &[T; N]),
    rng: &mut R,
) -> ([T; N], [T; N]) {
    let cut_point = rng.gen_range(0..N);
    let mut children_genes = ([T::default(); N], [T::default(); N]);

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

pub fn average_recombine<const N: usize>(
    parents_genes: (&[f64; N], &[f64; N]),
) -> ([f64; N], [f64; N]) {
    let mut children_genes = ([0.0; N], [0.0; N]);

    for i in 0..N {
        let average: f64 = (parents_genes.0[i] + parents_genes.1[i]) / 2.0;
        children_genes.0[i] = average;
        children_genes.1[i] = average;
    }
    children_genes
}

pub fn random_recombine<const N: usize>(
    parents_genes: (&[f64; N], &[f64; N]),
    rng: &mut R,
) -> ([f64; N], [f64; N]) {
    let mut choose_random =
        |value1: f64, value2: f64| rng.gen_range(value1.min(value2)..=value1.max(value2));
    let mut children_genes = ([0.0; N], [0.0; N]);

    for i in 0..N {
        children_genes.0[i] = choose_random(parents_genes.0[i], parents_genes.1[i]);
        children_genes.1[i] = choose_random(parents_genes.0[i], parents_genes.1[i]);
    }
    children_genes
}
