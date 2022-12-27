fn rosenbrock_2d(args: &[f64; N]) -> f64 {
    let a: f64 = 1.0;
    let b: f64 = 100.0;
    let [x, y] = args;

    (a - x).powi(2) + b * (y - x.powi(2)).powi(2)
}