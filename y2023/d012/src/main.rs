use std::collections::BTreeSet;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");

    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let solution: u64 = input
        .lines()
        .map(|line| {
            let mut s: BTreeSet<(usize, usize)> = BTreeSet::new();
            for (number, number_str) in numbers.iter().enumerate() {
                if let Some(pos) = line.find(&format!("{number}")) {
                    s.insert((pos, number));
                }
                if let Some(pos) = line.find(number_str) {
                    s.insert((pos, number));
                }
            }
            [s.first().unwrap(), s.last().unwrap()]
                .iter()
                .map(|(_, number)| format!("{number}"))
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum();

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());

    Ok(())
}
