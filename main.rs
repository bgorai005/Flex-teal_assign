use std::f64::consts::PI;
use std::io;

fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

fn exact_sol(t: f64) -> f64 {
    0.5 * (t.cos() + t.sin() + (-t).exp())
}

fn main() {
    // Input from user
    let mut input = String::new();
    println!("Enter the number of iterations: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Enter a valid integer");

    // Initial conditions
    let t0 = 0.0;
    let y0 = 1.0;
    let tend = 5.0;
    let h = (tend - t0) / n as f64;

    // Create vectors
    let mut t = vec![0.0; n + 1];
    let mut y = vec![0.0; n + 1];
    let mut y_exact = vec![0.0; n + 1];

    // Set initial values
    t[0] = t0;
    y[0] = y0;

    // Euler's method
    for i in 0..n {
        t[i + 1] = t[i] + h;
        y[i + 1] = y[i] + h * f(t[i], y[i]);
    }

    // Exact solution
    for i in 0..=n {
        y_exact[i] = exact_sol(t[i]);
    }

    // Print results (limited)
    println!("\n{:>10} {:>15} {:>15}", "t", "Euler y", "Exact y");
    for i in 0..=n.min(10) {  // print first 10 values only
        println!("{:>10.4} {:>15.6} {:>15.6}", t[i], y[i], y_exact[i]);
    }
}
