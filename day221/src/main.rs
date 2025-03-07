use std::error::Error;
use std::ops::{BitXor, Mul};
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    println!(
        "Solution: {:#?} / Duration: {:.6?}",
        input
            .lines()
            .map(|line| line.parse::<u64>().expect("unable to parse integer"))
            .map(|number| {
                let mut next = number;
                for _ in 0..2000 {
                    next = next.mul(64).bitxor(next).rem_euclid(16_777_216);
                    next = next.div_euclid(32).bitxor(next).rem_euclid(16_777_216);
                    next = next.mul(2048).bitxor(next).rem_euclid(16_777_216);
                }
                // println!("{number} - {next}");
                next
            })
            .sum::<u64>(),
        t0.elapsed()
    );
    Ok(())
}
