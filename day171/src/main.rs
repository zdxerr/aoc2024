use regex::Regex;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(
        r"Register\s*A:\s*(\d+)\s*Register\s*B:\s*(\d+)\s*Register\s*C:\s*(\d+)\s*Program:\s*([\d,]+)",
    )
    .unwrap();

    let (_, [rega, regb, regc, program]) = expr
        .captures(input.as_str())
        .expect("unable to find pattern in input")
        .extract();

    dbg!(rega, regb, regc, program);

    let program: Vec<u8> = program.split(',').map(|s| s.parse().unwrap()).collect();
    dbg!(program);
    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
