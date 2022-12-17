pub type Topology = Vec<Vec<usize>>;

pub fn empty(n: usize) -> Topology {
    (0..n).map(|_| Vec::new()).collect()
}

pub fn fully_connected(n: usize) -> Topology {
    let mut adjacency_list = Vec::new();
    for u in 0..n {
        adjacency_list.push(Vec::new());
        for v in 0..n {
            if u == v {
                continue;
            }
            adjacency_list[u].push(v);
        }
    }
    adjacency_list
}

pub fn ring(n: usize) -> Topology {
    if n == 1 {
        return empty(1);
    }
    (0..n).map(|node| vec![(node + 1) % n]).collect()
}

pub fn star(n: usize) -> Topology {
    let mut adjacency_list: Topology = (0..n).map(|_| vec![0]).collect();
    adjacency_list[0] = (1..n).collect();
    adjacency_list
}

pub fn is_valid(topology: &Topology, n: usize) -> bool {
    if topology.len() != n {
        return false;
    }
    topology.iter().flatten().all(|node| *node < n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn ring_test() {
        assert_eq!(ring(1), vec![vec![]]);
        assert_eq!(ring(2), vec![vec![1], vec![0]]);
        assert_eq!(ring(5), vec![vec![1], vec![2], vec![3], vec![4], vec![0]]);
    }

    fn validate_adjacency_lists(g1: &Topology, g2: &Topology) {
        let g1_sets: Vec<HashSet<_>> = g1.iter().map(|l| l.iter().collect()).collect();
        let g2_sets: Vec<HashSet<_>> = g2.iter().map(|l| l.iter().collect()).collect();

        for (set1, set2) in g1_sets.iter().zip(g2_sets) {
            assert_eq!(*set1, set2);
        }
    }

    #[test]
    fn fully_connected_test() {
        validate_adjacency_lists(&fully_connected(1), &vec![vec![]]);
        validate_adjacency_lists(&fully_connected(2), &vec![vec![1], vec![0]]);
        validate_adjacency_lists(
            &fully_connected(3),
            &vec![vec![1, 2], vec![0, 2], vec![0, 1]],
        );
    }

    #[test]
    fn star_test() {
        validate_adjacency_lists(&star(1), &vec![vec![]]);
        validate_adjacency_lists(&star(2), &vec![vec![1], vec![0]]);
        validate_adjacency_lists(
            &star(5),
            &vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]],
        )
    }
}
