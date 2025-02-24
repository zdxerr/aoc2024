use core::panic;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs, str};

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
    let start = grid.iter().position(|v| *v == START).unwrap();
    let mut times = vec![usize::MAX; grid.len()];
    let mut jumps: Vec<(usize, usize)> = vec![];
    let mut pos = start;
    times[pos] = 0;

    loop {
        let (n, e, s, w) = (pos - xlen - 1, pos + 1, pos + xlen + 1, pos - 1);
        let (e2, s2) = (pos + 2, pos + 2 * (xlen + 1));

        jumps.extend([(pos, e2), (pos, s2)]);

        if grid[pos] == END {
            break;
        }
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
    // let mut hm: HashMap<usize, usize> = HashMap::new();
    let mut solution = 0_usize;
    for (a, b) in jumps {
        match grid.get(b) {
            Some(&WALL | &NL) | None => continue,
            _ => {}
        }
        let x = times[b].abs_diff(times[a]).saturating_sub(2);
        // if x > 0 {
        //     hm.entry(x).and_modify(|v| *v += 1).or_insert(1);
        //     // println!(
        //     //     "JUMP {} {a}({}) -> {b}({}) {}",
        //     //     grid[b] as char, times[a], times[b], x,
        //     // );
        // }
        if x >= 100 {
            solution += 1;
        }
    }
    // println!("{hm:#?}");
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
