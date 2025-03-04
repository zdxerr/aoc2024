use core::str;
use std::error::Error;
use std::iter::repeat_n;
use std::time::Instant;
use std::{env, fs};

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let mut padpaths = HashMap::new();
    compute_padpaths(
        &mut padpaths,
        &[
            [b'7', b'8', b'9'],
            [b'4', b'5', b'6'],
            [b'1', b'2', b'3'],
            [b' ', b'0', b'A'],
        ],
    );
    compute_padpaths(&mut padpaths, &[[b' ', b'^', b'A'], [b'<', b'v', b'>']]);

    let mut cache: HashMap<(u8, u8, usize), usize> = HashMap::new();

    let mut solution = 0_usize;
    for code in input.trim_ascii().split(|c| *c == b'\n') {
        let number: usize = str::from_utf8(code.split_last_chunk::<1>().unwrap().0)?.parse()?;
        let length = solve(code, &padpaths, 25, &mut cache);
        let complexity = number * length;
        // println!("{} * {} = {}", length, number, complexity);

        solution += complexity;
    }

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

fn solve(
    code: &[u8],
    padpaths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    depth: usize,
    cache: &mut HashMap<(u8, u8, usize), usize>,
) -> usize {
    let mut pos = b'A';
    let mut sum = 0_usize;
    for next in code {
        if depth == 0 {
            sum += padpaths.get(&(pos, *next)).unwrap().get(0).unwrap().len();
        } else {
            let key = (pos, *next, depth - 1);
            sum += if cache.contains_key(&key) {
                *cache.get(&key).unwrap()
            } else {
                let len = padpaths
                    .get(&(pos, *next))
                    .unwrap()
                    .iter()
                    .map(|path| solve(path, padpaths, depth - 1, cache))
                    .min()
                    .unwrap();
                cache.insert(key, len);
                len
            };
        }

        pos = *next;
    }
    sum
}

fn compute_padpaths(padpaths: &mut HashMap<(u8, u8), Vec<Vec<u8>>>, pad: &[[u8; 3]]) {
    let stack: Vec<(u8, usize, usize)> = pad
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, num)| **num != b' ')
                .map(move |(x, num)| (*num, x, y))
        })
        .collect();

    for (from, x0, y0) in &stack {
        for (to, x1, y1) in &stack {
            let paths = padpaths.entry((*from, *to)).or_default();
            if from == to {
                paths.push(vec![b'A']);
                continue;
            }
            let path: Vec<u8> = repeat_n(if x0 < x1 { b'>' } else { b'<' }, x1.abs_diff(*x0))
                .chain(repeat_n(
                    if y0 < y1 { b'v' } else { b'^' },
                    y1.abs_diff(*y0),
                ))
                .collect();
            if pad[*y0][*x1] != b' ' {
                paths.push(path.iter().copied().chain([b'A'].into_iter()).collect());
            }

            if pad[*y1][*x0] != b' ' && x0 != x1 && y0 != y1 {
                paths.push(
                    path.iter()
                        .copied()
                        .rev()
                        .chain([b'A'].into_iter())
                        .collect(),
                );
            }
        }
    }
}
