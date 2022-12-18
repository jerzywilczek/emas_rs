/// Topologies are `<Vec<Vec<usize>>` graph-like lists of adjacent islands. For *n* islands they are a `Vec` of *n* `Vec`s, each consisting of `usize` only from *0* to *n-1* (can be empty).

pub type Topology = Vec<Vec<usize>>;

/// No connections – all islands are disjoint.
pub fn empty(n: usize) -> Topology {
    (0..n).map(|_| Vec::new()).collect()
}

/// All islands are connected to each other.
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

/// Islands form a directed [ring](https://en.wikipedia.org/wiki/Ring_network).
pub fn ring(n: usize) -> Topology {
    if n == 1 {
        return empty(1);
    }
    (0..n).map(|node| vec![(node + 1) % n]).collect()
}

/// Island0 is connected to all other islands and all other islands are connected only to island0. See [Star network](https://en.wikipedia.org/wiki/Star_network).
pub fn star(n: usize) -> Topology {
    let mut adjacency_list: Topology = vec![vec![0]; n];
    adjacency_list[0] = (1..n).collect();
    adjacency_list
}

/// Checks if given `topology` is valid for given island number `n`.
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
