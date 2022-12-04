pub mod rastrigin_fitness {
    pub const fn domain<const N: usize>() -> [(f64, f64); N] {
        [(-5.12, 5.12); N]
    }

    pub fn rastrigin<const N: usize>(args: &[f64; N]) -> f64 {
        use std::f64::consts::PI;
        let a: f64 = 10.0;
        let mut fx = a * N as f64;
        fx += args
            .iter()
            .map(|xi| xi.powf(2.0) - a * (2.0 * PI * xi).cos())
            .sum::<f64>();
        fx
    }
}
