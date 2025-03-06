use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let secrets: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.parse::<i64>().expect("unable to parse integer"))
        .map(|mut number| {
            (0..2000)
                .map(|_| {
                    number = (number ^ (number << 6)) & 16_777_215;
                    number = (number ^ (number >> 5)) & 16_777_215;
                    number = (number ^ (number << 11)) & 16_777_215;
                    number
                })
                .collect()
        })
        .collect();
    let deltas: Vec<Vec<(i64, i64)>> = secrets
        .iter()
        .map(|sequence| {
            sequence
                .windows(2)
                .map(|numbers| (numbers[1] % 10 - numbers[0] % 10, numbers[1] % 10))
                .collect()
        })
        .collect();

    let mut sequences: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for sequence in &deltas {
        for window in sequence.windows(4) {
            let key = (window[0].0, window[1].0, window[2].0, window[3].0);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);
            sequences
                .entry(key)
                .and_modify(|v| *v += window[3].1)
                .or_insert(window[3].1);
        }
        seen.clear();
    }

    println!(
        "Solution: {:#?} / Duration: {:.6?}",
        sequences.into_values().max().unwrap(),
        t0.elapsed()
    );
    Ok(())
}
