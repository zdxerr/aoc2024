use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");

    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let solution: u64 = input
        .lines()
        .map(|line| {
            [
                line.chars().find(|c| c.is_numeric()).unwrap(),
                line.chars().rfind(|c| c.is_numeric()).unwrap(),
            ]
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
        })
        .sum();

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());

    Ok(())
}
