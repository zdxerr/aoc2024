use std::collections::BTreeSet;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};
const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");

    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");
    let mut s: BTreeSet<(usize, usize)> = BTreeSet::new();
    let solution: usize = input
        .lines()
        .map(|line| {
            s.clear();
            for (number, number_str) in NUMBERS.iter().enumerate() {
                if let Some(pos) = line.find(&format!("{number}")) {
                    s.insert((pos, number));
                }
                if let Some(pos) = line.find(number_str) {
                    s.insert((pos, number));
                }
                if let Some(pos) = line.rfind(&format!("{number}")) {
                    s.insert((pos, number));
                }
                if let Some(pos) = line.rfind(number_str) {
                    s.insert((pos, number));
                }
            }

            s.first().unwrap().1 * 10 + s.last().unwrap().1
        })
        .sum();

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());

    Ok(())
}
