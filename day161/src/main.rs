use std::error::Error;
use std::io::stdin;
use std::time::Instant;
use std::{env, fs, str};

const NL: u8 = b'\n';

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let map = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let xlen = map.iter().position(|v| *v == NL).expect("missing new line");
    println!("{}", str::from_utf8(&map)?);
    let solution = 0;
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
