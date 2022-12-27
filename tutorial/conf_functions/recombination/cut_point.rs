pub fn cut_point_recombine<const N: usize, T: Gene>(
    parents_genes: (&[T; N], &[T; N]),
    cut_point: usize,
) -> ([T; N], [T; N])