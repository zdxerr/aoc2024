use regex::Regex;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).ok_or("invalid arguments")?;
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(
        r"Register\s*A:\s*(\d+)\s*Register\s*B:\s*(\d+)\s*Register\s*C:\s*(\d+)\s*Program:\s*([\d,]+)",
    )?;

    let (_, [rega, regb, regc, program]) = expr
        .captures(input.as_str())
        .ok_or("failed to parse input")?
        .extract();

    dbg!(rega, regb, regc, program);

    // let (mut rega, mut regb, mut regc): (i64, i64, i64) =
    //     [rega, regb, regc].map(|s| s.parse()).iter();

    let mut program = program.split(',').map(|s| s.parse::<u8>().unwrap());

    loop {
        if let Some(opcode) = program.next() {
            dbg!(opcode);
        } else {
            break;
        }
        if let Some(operand) = program.next() {
            dbg!(operand);
        }
    }
    // dbg!(program);
    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
