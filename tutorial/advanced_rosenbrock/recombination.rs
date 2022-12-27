pub fn my_cut_point_recombine<const N: usize, T: Gene>(
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