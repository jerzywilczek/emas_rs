/// [Rastrigin function](https://www.sfu.ca/~ssurjano/rastr.html)
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

/// [Rosenbrock function](https://www.sfu.ca/~ssurjano/rosen.html)
pub mod rosenbrock_fitness {
    pub const fn domain<const N: usize>() -> [(f64, f64); N] {
        [(-5.0, 5.0); N]
    }

    pub fn rosenbrock<const N: usize>(args: &[f64; N]) -> f64 {
        let a = 1.0;
        let b = 100.0;
        let mut total = 0.0;
        for xi in 1..args.len() {
            total += b * (args[xi] - args[xi - 1].powi(2)).powi(2) + (a - args[xi - 1]).powi(2);
        }
        total
    }
}

/// [Ackley function](https://www.sfu.ca/~ssurjano/ackley.html)
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

/// [Low Autocorrelation Binary Sequences](http://militzer.berkeley.edu/sequences.html)
pub mod labs_fitness {
    pub fn labs<const N: usize>(args: &[bool; N]) -> f64 {
        merit_factor::<N>(labs_energy(args) as f64)
    }

    fn merit_factor<const N: usize>(labs_energy: f64) -> f64 {
        N as f64 * N as f64 / (2.0 * labs_energy)
    }

    fn labs_energy<const N: usize>(args: &[bool; N]) -> i32 {
        (1..N).map(|k| autocorrelation(args, k).pow(2)).sum()
    }

    fn autocorrelation<const N: usize>(args: &[bool; N], distance: usize) -> i32 {
        (0..N - distance)
            .map(|i| {
                let s_i = if args[i] { 1 } else { -1 };
                let s_ik = if args[i + distance] { 1 } else { -1 };
                s_i * s_ik
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ackley_fitness::ackley;
    use assert_approx_eq::assert_approx_eq;
    use labs_fitness::labs;
    use rastrigin_fitness::rastrigin;
    use rosenbrock_fitness::rosenbrock;

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

    macro_rules! bool_arr {
        () => (
            [false; 0]
        );
         ($($e:expr),+) => (
            [$(if $e == 0 {false} else if $e == 1 {true} else {panic!("Expected only 0s or 1s, got {}", $e)},)+]
        );
    }

    #[test]
    fn labs_correct_values() {
        let test1 = (
            bool_arr![
                0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1,
                0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
                0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0
            ],
            8.20,
        );
        let test2 = (
            bool_arr![
                1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0,
                1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1,
                0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0,
                1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1,
                1
            ],
            8.49,
        );

        let eps = 0.01;
        assert_approx_eq!(labs(&test1.0), test1.1, eps);
        assert_approx_eq!(labs(&test2.0), test2.1, eps);
    }
}
