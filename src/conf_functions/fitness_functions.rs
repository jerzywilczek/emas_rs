pub mod rastrigin_fitness {
    pub const fn domain<const N: usize>() -> [(f64, f64); N] {
        [(-5.12, 5.12); N]
    }

    pub fn rastrigin<const N: usize>(args: &[f64; N]) -> f64 {
        use std::f64::consts::PI;
        let a: f64 = 10.0;
        let mut total = a * N as f64;
        total += args
            .iter()
            .map(|xi| xi.powf(2.0) - a * (2.0 * PI * xi).cos())
            .sum::<f64>();
        total
    }
}

pub mod rosenbrock_fitness {
    pub const fn domain<const N: usize>() -> [(f64, f64); N] {
        [(-5.0, 5.0); N]
    }

    pub fn rosenbrock<const N: usize>(args: &[f64; N]) -> f64 {
        let a = 1.0;
        let b = 100.0;
        let mut total = 0.0;
        for xi in 0..args.len() - 1 {
            total += b * (args[xi + 1] - args[xi].powi(2)).powi(2) + (a - args[xi]).powi(2);
        }
        total
    }
}

pub mod ackley_fitness {
    pub const fn domain<const N: usize>() -> [(f64, f64); N] {
        [(-32.768, 32.768); N]
    }

    pub fn ackley<const N: usize>(args: &[f64; N]) -> f64 {
        use std::f64::consts::PI;
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
    use crate::conf_functions::fitness_functions::*;
    use ackley_fitness::ackley;
    use rastrigin_fitness::rastrigin;
    use rosenbrock_fitness::rosenbrock;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn rastrigin_correct_values() {
        // global minimum at (0, ..., 0)
        assert_eq!(rastrigin(&[0.0, 0.0]), 0.0);
        assert_eq!(rastrigin(&[0.0; 5]), 0.0);
        assert_eq!(rastrigin(&[0.0; 8]), 0.0);

        // global maximum at (4,5, ..., 4.5)
        assert_eq!(rastrigin(&[4.5, 4.5]), 80.5);
        assert_eq!(rastrigin(&[4.5; 5]), 201.25);
        assert_eq!(rastrigin(&[4.5; 8]), 322.0);

        // reflected input
        assert_eq!(rastrigin(&[-1.5, -2.0]), 26.25);
        assert_eq!(rastrigin(&[-1.5, 2.0]), 26.25);
        assert_eq!(rastrigin(&[1.5, -2.0]), 26.25);
        assert_eq!(rastrigin(&[1.5, 2.0]), 26.25);
    }

    #[test]
    fn rosenbrock_correct_values() {
        // global minimum at (1, ..., 1)
        assert_eq!(rosenbrock(&[1.0, 1.0]), 0.0);
        assert_eq!(rosenbrock(&[1.0; 5]), 0.0);
        assert_eq!(rosenbrock(&[1.0; 8]), 0.0);

        // other input
        assert_eq!(rosenbrock(&[2.0, -3.0]), 4901.0);
        assert_eq!(rosenbrock(&[1.5, -0.6]), 812.5);
    }

    #[test]
    fn ackley_correct_values() {
        let eps = 1e-15;
        // global minimum at (0, ..., 0)
        assert_approx_eq!(ackley(&[0.0, 0.0]), 0.0, eps);
        assert_approx_eq!(ackley(&[0.0; 5]), 0.0, eps);
        assert_approx_eq!(ackley(&[0.0; 8]), 0.0, eps);

        // other input
        assert_approx_eq!(ackley(&[-1.5, 0.3]), 6.089244894966122, eps);
        assert_approx_eq!(
            ackley(&[-15.242, 5.432, 6.958, 4.322, 20.385]),
            20.170366886584574,
            eps
        );
    }
}
