use core::panic;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const WALL: u8 = b'#';
const START: u8 = b'S';
const END: u8 = b'E';
const NL: u8 = b'\n';

const R: usize = 20;
const SAVED: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let grid = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let xlen = grid.iter().position(|v| *v == NL).unwrap();
    let start = grid.iter().position(|v| *v == START).unwrap();
    let mut times = vec![usize::MAX; grid.len()];
    let mut jumps: Vec<(usize, usize, usize)> = vec![];
    let mut pos = start;
    times[pos] = 0;

    loop {
        for r in 2..=R {
            for x in 0..=r {
                if ((pos % (xlen + 1)) + x) > xlen {
                    continue;
                }
                let next_pos = pos + x + r.saturating_sub(x) * (xlen + 1);
                match grid.get(next_pos) {
                    Some(&WALL | &NL) | None => {}
                    _ => jumps.push((pos, next_pos, r)),
                }
            }
            for x in 1..r {
                if (pos % (xlen + 1)).saturating_sub(x) == 0 {
                    continue;
                }
                let next_pos = pos.saturating_sub(x) + r.saturating_sub(x) * (xlen + 1);
                match grid.get(next_pos) {
                    Some(&WALL | &NL) | None => {}
                    _ => jumps.push((pos, next_pos, r)),
                }
            }
        }

        if grid[pos] == END {
            break;
        }
        let (n, e, s, w) = (pos - xlen - 1, pos + 1, pos + xlen + 1, pos - 1);
        if grid[n] != WALL && times[n] > times[pos] {
            times[n] = times[pos] + 1;
            pos = n;
        } else if grid[e] != WALL && times[e] > times[pos] {
            times[e] = times[pos] + 1;
            pos = e;
        } else if grid[s] != WALL && times[s] > times[pos] {
            times[s] = times[pos] + 1;
            pos = s;
        } else if grid[w] != WALL && times[w] > times[pos] {
            times[w] = times[pos] + 1;
            pos = w;
        } else {
            panic!("Something went wrong.");
        }
    }

    let solution = jumps
        .iter()
        .map(|(a, b, c)| times[*b].abs_diff(times[*a]).saturating_sub(*c))
        .filter(|d| *d >= SAVED)
        .count();
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
