use std::f64::consts::PI;
use crate::RASTRIGIN_DIMS;

pub trait FitnessFn {
    fn call(args: &[f64; 2]) -> f64;
}

pub struct RastriginFitness {}

impl FitnessFn for RastriginFitness {
    fn call(args: &[f64; 2]) -> f64 {
        let a: f64 = 10.0;
        let mut fx = a * RASTRIGIN_DIMS as f64;
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
        assert_eq!(RastriginFitness::call(&[0.0, 0.0]), 0.0)
    }

}