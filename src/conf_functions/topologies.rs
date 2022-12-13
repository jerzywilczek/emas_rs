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

    #[test]
    fn ring_test() {
        let ring2 = ring::<2>();
        assert_eq!(ring2, [vec![1], vec![0]]);
        let ring5 = ring::<5>();
        assert_eq!(ring5, [vec![1], vec![2], vec![3], vec![4], vec![0]]);
        let ring1 = ring::<1>();

        // assumes a single island is connected to itself
        assert_eq!(ring1, [vec![0]]);
    }
}
