use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");

    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    let mut is_lock: Option<bool> = None;
    let mut current = [0_u8; 5];
    for line in input.split(|c| *c == b'\n') {
        if is_lock.is_none() {
            is_lock = Some(line == [b'#'; 5]);
            continue;
        }
        if line.is_empty() {
            if let Some(true) = is_lock {
                locks.push(current);
            } else {
                keys.push(current.map(|c| c - 1));
            }
            is_lock = None;
            current = [0_u8; 5];
        }
        for (index, column) in line.iter().enumerate() {
            if *column == b'#' {
                current[index] += 1;
            }
        }
    }
    let mut solution = 0_usize;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(a, b)| a + b <= 5) {
                solution += 1;
            }
        }
    }
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());

    Ok(())
}
