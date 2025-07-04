pub fn exact_solution(t: f64) -> f64 {
    0.5 * (t.cos() + t.sin() + (-t).exp())
}
