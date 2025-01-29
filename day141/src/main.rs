use regex::Regex;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const X: i64 = 101;
const Y: i64 = 103;
// const X: i64 = 11;
// const Y: i64 = 7;
const SECONDS: i64 = 100;

// define quadrant ranges
const QX0: i64 = 0;
const QX1: i64 = X / 2;
const QX2: i64 = X - X / 2;
const QX3: i64 = X;

const QY0: i64 = 0;
const QY1: i64 = Y / 2;
const QY2: i64 = Y - Y / 2;
const QY3: i64 = Y;

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(r"p=(\d+),(\d+)\s*v=(-?\d+),(-?\d+)").unwrap();

    let mut quadrants: (u64, u64, u64, u64) = (0, 0, 0, 0);
    // let mut quadrants_a: [u64; 4] = [0; 4];
    for (_, [x, y, vx, vy]) in expr.captures_iter(input.as_str()).map(|c| c.extract()) {
        let (x, y, vx, vy) = (
            x.parse::<i64>()?,
            y.parse::<i64>()?,
            vx.parse::<i64>()?,
            vy.parse::<i64>()?,
        );

        let mut xf = (x + SECONDS * vx) % X;
        let mut yf = (y + SECONDS * vy) % Y;

        if xf < 0 {
            xf += X;
        }
        if yf < 0 {
            yf += Y;
        }
        match (xf, yf) {
            (QX0..QX1, QY0..QY1) => quadrants.0 += 1,
            (QX2..QX3, QY0..QY1) => quadrants.1 += 1,
            (QX0..QX1, QY2..QY3) => quadrants.2 += 1,
            (QX2..QX3, QY2..QY3) => quadrants.3 += 1,
            _ => {}
        }
        println!("x={x}, y={y}, vx={vx}, vy={vy}, xf={xf}, yf={yf}, quadrants={quadrants:?}");
    }
    // println!("{}", &input);
    let solution = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
