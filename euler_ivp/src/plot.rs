use plotters::prelude::*;

pub fn plot_solutions(n: usize, t: &Vec<f64>, y_euler: &Vec<f64>, y_exact: &Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("output/iteration_{}.png", n);
    let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_min = y_euler.iter().copied().chain(y_exact.iter().copied()).fold(f64::INFINITY, f64::min);
    let y_max = y_euler.iter().copied().chain(y_exact.iter().copied()).fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Euler vs Exact (n = {})", n), ("Arial", 24))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(t[0]..t[t.len() - 1], y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        t.iter().zip(y_euler.iter()).map(|(&t, &y)| (t, y)),
        &BLUE,
    ))?.label("Euler y")
      .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    chart.draw_series(LineSeries::new(
        t.iter().zip(y_exact.iter()).map(|(&t, &y)| (t, y)),
        &RED,
    ))?.label("Exact y")
      .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    println!("📊 Plot saved to {}", filename);
    Ok(())
}
