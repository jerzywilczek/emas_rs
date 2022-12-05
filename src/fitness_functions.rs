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

struct AckleyFitneess<const N: usize> {}

impl<const N: usize> FitnessFn<N> for AckleyFitneess<N> {
    const DOMAIN: [(f64, f64); N] = [(-32.768, 32.768); N];

    fn call(args: &[f64; N]) -> f64 {
        let (a, b, c) = (20.0, 0.2, 2.0 * PI);
        let d = N as f64;
        let total1 =
            -a * f64::exp(-b * f64::sqrt(1.0 / d * args.iter().map(|xi| xi.powi(2)).sum::<f64>()));
        let total2 = -f64::exp(1.0 / d * args.iter().map(|xi| f64::cos(c * xi)).sum::<f64>());
        total1 + total2 + a + f64::exp(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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

    fn is_within_precision_err(val: f64, expected: f64, eps: Option<f64>) -> bool {
        let eps = eps.unwrap_or(1e-15);
        f64::abs(val - expected) < eps
    }

    #[test]
    fn ackley_correct_values() {
        let eps = 1e-15;
        // global minimum at (0, ..., 0)
        assert_approx_eq!(AckleyFitneess::<2>::call(&[0.0, 0.0]), 0.0, eps);
        assert_approx_eq!(AckleyFitneess::<5>::call(&[0.0; 5]), 0.0, eps);
        assert_approx_eq!(AckleyFitneess::<8>::call(&[0.0; 8]), 0.0, eps);

        // other input
        assert_approx_eq!(
            AckleyFitneess::<2>::call(&[-1.5, 0.3]),
            6.089244894966122,
            eps
        );
        assert_approx_eq!(
            AckleyFitneess::<5>::call(&[-15.242, 5.432, 6.958, 4.322, 20.385]),
            20.170366886584574,
            eps






            //TODO remove these empty \n





        );
    }
}
