use std::error::Error;
use std::fs::create_dir_all;
use csv::Writer;

pub fn write_csv(n: usize, t: &Vec<f64>, y: &Vec<f64>, y_exact: &Vec<f64>) -> Result<(), Box<dyn Error>> {
    create_dir_all("output")?;
    let filename = format!("output/iteration_{}.csv", n);
    let mut wtr = Writer::from_path(filename)?;
    wtr.write_record(&["t", "euler_y", "exact_y", "error"])?;

    for i in 0..t.len() {
        let err = (y[i] - y_exact[i]).abs();
        wtr.write_record(&[
            t[i].to_string(),
            y[i].to_string(),
            y_exact[i].to_string(),
            err.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
