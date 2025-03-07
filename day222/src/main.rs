use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let mut sequences: HashMap<i64, i64> = HashMap::new();
    let mut seen: HashSet<i64> = HashSet::new();

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

    for sequence in &secrets {
        let mut key = 0;

        let mut last = 0;

        for sn in sequence {
            let delta = sn % 10 - last % 10;
            key = ((key << 8) | (delta + 9)) & 0xFFFF_FFFF;

            if seen.contains(&key) {
                last = *sn;
                continue;
            }
            seen.insert(key);
            sequences
                .entry(key)
                .and_modify(|v| *v += sn % 10)
                .or_insert(sn % 10);
            last = *sn;
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
