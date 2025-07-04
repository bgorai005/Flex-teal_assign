mod solver;
mod exact;
mod utils;

use std::io;
use solver::euler;
use exact::exact_solution;
use utils::write_csv;

fn main() {
    // Get user input for number of steps
    let mut input = String::new();
    println!("Enter the number of iterations (n): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    // Initial conditions
    let t0 = 0.0;
    let y0 = 1.0;
    let tend = 5.0;

    // Call Euler solver
    let (t, y) = euler(t0, y0, tend, n);
    let y_exact: Vec<f64> = t.iter().map(|&ti| exact_solution(ti)).collect();

    // Display results
    println!("{:<10} {:<20} {:<20} {:<20}", "t", "Euler y", "Exact y", "Error");
    for i in 0..=n {
        let error = (y[i] - y_exact[i]).abs();
        println!("{:<10.4} {:<20.6} {:<20.6} {:<20.6}", t[i], y[i], y_exact[i], error);
    }

    // Save to CSV
    if let Err(e) = write_csv("output/euler_solution.csv", &t, &y, &y_exact) {
        eprintln!("Error writing CSV: {}", e);
    } else {
        println!("\nCSV saved to 'output/euler_solution.csv'");
    }
}
