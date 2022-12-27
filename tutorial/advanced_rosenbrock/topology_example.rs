/// Islands form a directed [ring](https://en.wikipedia.org/wiki/Ring_network).
pub fn ring(n: usize) -> Topology {
    if n == 1 {
        return empty(1);
    }
    (0..n).map(|node| vec![(node + 1) % n]).collect()
}