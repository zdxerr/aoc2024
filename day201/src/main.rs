use core::panic;
use std::error::Error;
use std::time::Instant;
use std::{env, fs, str};

const WAY: u8 = b'.';
const WALL: u8 = b'#';
const START: u8 = b'S';
const END: u8 = b'E';
const NL: u8 = b'\n';

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let grid = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let xlen = grid.iter().position(|v| *v == NL).unwrap();
    let mut pos = grid.iter().position(|v| *v == START).unwrap();

    let mut times: Vec<u64> = vec![u64::MAX; grid.len()];

    times[pos] = 0;

    while grid[pos] != END {
        let (n, e, s, w) = (pos - xlen - 1, pos + 1, pos + xlen + 1, pos - 1);

        let directions = (
            grid[n] == WAY,
            grid[e] == WAY,
            grid[s] == WAY,
            grid[w] == WAY,
        );

        let c = (times[pos], times[n], times[e], times[s], times[w]);
        println!(
            "{xlen} {pos} {} {n} {e} {s} {w} {directions:#?} {c:#?}",
            grid[pos] as char
        );

        if grid[n] == WAY && times[n] > times[pos] {
            times[n] = times[pos] + 1;
            pos = n;
        } else if grid[e] == WAY && times[e] > times[pos] {
            times[e] = times[pos] + 1;
            pos = e;
        } else if grid[s] == WAY && times[s] > times[pos] {
            times[s] = times[pos] + 1;
            pos = s;
        } else if grid[w] == WAY && times[w] > times[pos] {
            times[w] = times[pos] + 1;
            pos = w;
        } else {
            panic!("Something went wrong.");
        }
    }
    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
