use std::f64::consts::PI;

pub trait FitnessFn<const N: usize> {
    fn call(args: &[f64; N]) -> f64;
}

pub struct RastriginFitness<const N: usize> {}

impl<const N: usize> FitnessFn<N> for RastriginFitness<N> {
    fn call(args: &[f64; N]) -> f64 {
        let a: f64 = 10.0;
        let mut fx = a * N as f64;
        fx += args
            .iter()
            .map(|xi| xi.powf(2.0) - a * (2.0 * PI * xi).cos())
            .sum::<f64>();
        fx
    }
}

#[cfg(test)]
mod tests {
    use crate::fitness_functions::{RastriginFitness, FitnessFn};

    #[test]
    fn rastrigin_min_test() {
        assert_eq!(RastriginFitness::<2>::call(&[0.0, 0.0]), 0.0)
    }

}