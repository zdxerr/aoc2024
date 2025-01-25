use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const BLINKS0: u8 = 25;
const BLINKS1: u8 = 75;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("no input path");
    let stones: Vec<u64> = fs::read_to_string(&input_path)?
        .trim()
        .split(' ')
        .map(|value| u64::from_str_radix(value, 10).unwrap())
        .collect();
    println!("Read {input_path}.");

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    let t0 = Instant::now();
    let solution0: u64 = stones
        .iter()
        .map(|stone| solve(*stone, BLINKS0, &mut cache))
        .sum();
    println!(
        "Stones after {} blinks: {} / Duration: {:.6?}",
        BLINKS0,
        solution0,
        t0.elapsed()
    );

    let t1 = Instant::now();
    let solution1: u64 = stones
        .iter()
        .map(|stone| solve(*stone, BLINKS1, &mut cache))
        .sum();
    println!(
        "Stones after {} blinks: {} / Duration: {:.6?}",
        BLINKS1,
        solution1,
        t1.elapsed()
    );
    Ok(())
}

fn solve(stone: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(number_of_stones) = cache.get(&(stone, blinks)) {
        return *number_of_stones;
    }

    let digits = stone.checked_ilog10().unwrap_or(0) + 1;
    let number_of_stones = if stone == 0 {
        solve(1, blinks - 1, cache)
    } else if digits % 2 == 0 {
        let factor: u64 = u64::pow(10, digits / 2);
        return solve(stone / factor, blinks - 1, cache) + solve(stone % factor, blinks - 1, cache);
    } else {
        solve(stone * 2024, blinks - 1, cache)
    };
    cache.insert((stone, blinks), number_of_stones);
    number_of_stones
}
