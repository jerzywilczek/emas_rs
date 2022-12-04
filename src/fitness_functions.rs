use std::f64::consts::PI;

pub trait FitnessFn<const N: usize> {
    const DOMAIN: [(f64, f64); N];
    fn call(args: &[f64; N]) -> f64;
}

pub struct RastriginFitness<const N: usize> {}

impl<const N: usize> FitnessFn<N> for RastriginFitness<N> {
    const DOMAIN: [(f64, f64); N] = [(-5.12, 5.12); N];

    fn call(args: &[f64; N]) -> f64 {
        let a: f64 = 10.0;
        let mut total = a * N as f64;
        total += args
            .iter()
            .map(|xi| xi.powf(2.0) - a * (2.0 * PI * xi).cos())
            .sum::<f64>();
        total
    }
}


struct RosenbrockFitness<const N: usize> {}

impl<const N: usize> FitnessFn<N> for RosenbrockFitness<N> {
    const DOMAIN: [(f64, f64); N] = [(-5.0, 5.0); N];

    fn call(args: &[f64; N]) -> f64 {
        let a = 1.0;
        let b = 100.0;
        let mut total = 0.0;
        for xi in 0..args.len() - 1 {
            total += b * (args[xi + 1] - args[xi].powi(2)).powi(2) + (a - args[xi]).powi(2);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use std::env::args;
    use super::*;

    #[test]
    fn rastrigin_correct_values() {
        // global minimum at (0, ..., 0)
        assert_eq!(RastriginFitness::<2>::call(&[0.0, 0.0]), 0.0);
        assert_eq!(RastriginFitness::<5>::call(&[0.0; 5]), 0.0);
        assert_eq!(RastriginFitness::<8>::call(&[0.0; 8]), 0.0);

        // global maximum at (4,5, ..., 4.5)
        assert_eq!(RastriginFitness::<2>::call(&[4.5, 4.5]), 80.5);
        assert_eq!(RastriginFitness::<5>::call(&[4.5; 5]), 201.25);
        assert_eq!(RastriginFitness::<8>::call(&[4.5; 8]), 322.0);

        // reflected input
        assert_eq!(RastriginFitness::<2>::call(&[-1.5, -2.0]), 26.25);
        assert_eq!(RastriginFitness::<2>::call(&[-1.5, 2.0]), 26.25);
        assert_eq!(RastriginFitness::<2>::call(&[1.5, -2.0]), 26.25);
        assert_eq!(RastriginFitness::<2>::call(&[1.5, 2.0]), 26.25);
    }

    #[test]
    fn rosenbrock_correct_values() {
        // global minimum at (1, ..., 1)
        assert_eq!(RosenbrockFitness::<2>::call(&[1.0, 1.0]), 0.0);
        assert_eq!(RosenbrockFitness::<5>::call(&[1.0; 5]), 0.0);
        assert_eq!(RosenbrockFitness::<8>::call(&[1.0; 8]), 0.0);

        // other input
        assert_eq!(RosenbrockFitness::<2>::call(&[2.0, -3.0]), 4901.0);
        assert_eq!(RosenbrockFitness::<2>::call(&[1.5, -0.6]), 812.5);
    }
}
