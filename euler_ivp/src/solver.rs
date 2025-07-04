pub fn euler(t0: f64, y0: f64, tend: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    let h = (tend - t0) / n as f64;
    let mut t = vec![0.0; n + 1];
    let mut y = vec![0.0; n + 1];

    t[0] = t0;
    y[0] = y0;

    for i in 0..n {
        t[i + 1] = t[i] + h;
        y[i + 1] = y[i] + h * (t[i].cos() - y[i]);
    }

    (t, y)
}
