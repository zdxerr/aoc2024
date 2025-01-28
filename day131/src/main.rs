use regex::Regex;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    // println!("{}", &input);

    let expr = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X\=(\d+), Y\=(\d+)",
    )
    .unwrap();

    let mut tokens: u64 = 0;

    for (_, [xa, ya, xb, yb, x, y]) in expr.captures_iter(input.as_str()).map(|c| c.extract()) {
        let (xa, ya, xb, yb, x, y, xa_, ya_, xb_, yb_, x_, y_) = (
            xa.parse::<u64>().unwrap(),
            ya.parse::<u64>().unwrap(),
            xb.parse::<u64>().unwrap(),
            yb.parse::<u64>().unwrap(),
            x.parse::<u64>().unwrap(),
            y.parse::<u64>().unwrap(),
            xa.parse::<f64>().unwrap(),
            ya.parse::<f64>().unwrap(),
            xb.parse::<f64>().unwrap(),
            yb.parse::<f64>().unwrap(),
            x.parse::<f64>().unwrap(),
            y.parse::<f64>().unwrap(),
        );

        let a_ = ((-y_ / ya_) + ((x_ * yb_) / (xb_ * ya_))) / ((xa_ * yb_) / (xb_ * ya_) - 1.0);
        let b_ = (x_ - a_ * xa_) / xb_;
        let (a, b) = (a_.round() as u64, b_.round() as u64);

        let valid = (a * xa + b * xb) == x && (a * ya + b * yb) == y;

        if valid {
            tokens += 3 * a + b;
        }
    }

    println!("Solution: {} / Duration: {:.6?}", tokens, t0.elapsed());

    Ok(())
}
