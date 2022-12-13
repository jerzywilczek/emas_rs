pub fn complete<const N: usize>() -> [Vec<usize>; N] {
    const EMPTY_VEC: Vec<usize> = vec![];
    let mut adjacency_list = [EMPTY_VEC; N];
    for u in 0..N {
        for v in 0..N {
            adjacency_list[u].push(v);
        }
    }
    adjacency_list
}

pub fn ring<const N: usize>() -> [Vec<usize>; N] {
    const EMPTY_VEC: Vec<usize> = vec![];
    let mut adjacency_list = [EMPTY_VEC; N];
    for node in 0..N {
        adjacency_list[node].push((node + 1) % N);
    }
    adjacency_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn ring_test() {
        let ring1 = ring::<1>();
        assert_eq!(ring1, [vec![0]]); // assumes a single island is connected to itself
        let ring2 = ring::<2>();
        assert_eq!(ring2, [vec![1], vec![0]]);
        let ring5 = ring::<5>();
        assert_eq!(ring5, [vec![1], vec![2], vec![3], vec![4], vec![0]]);
    }

    fn validate_adjacency_lists<const N: usize>(g1: &[Vec<usize>; N], g2: &[Vec<usize>; N]) {
        let g1_sets: Vec<HashSet<_>> = g1.iter().map(|l| l.iter().collect()).collect();
        let g2_sets: Vec<HashSet<_>> = g2.iter().map(|l| l.iter().collect()).collect();

        for (set1, set2) in g1_sets.iter().zip(g2_sets) {
            assert_eq!(*set1, set2);
        }
    }

    #[test]
    fn complete_test() {
        // assumes a single island is connected to itself
        validate_adjacency_lists(&complete::<1>(), &[vec![0]]);
        validate_adjacency_lists(&complete::<2>(), &[vec![0, 1], vec![0, 1]]);
        validate_adjacency_lists(
            &complete::<3>(),
            &[vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]],
        );
    }
}
