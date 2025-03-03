use core::panicking::panic_const::panic_const_shr_overflow;
use core::str;
use std::error::Error;
use std::io::BufRead;
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

    // // dbg!(numcoords, dcoords, &padpaths, &padpaths.len());
    // for (a, b) in padpaths2.keys() {
    //     println!("{}->{}", *a as char, *b as char);
    // }
    // let mut c = 0;
    // for (a, p) in padpaths {
    //     for (b, paths) in p {
    //         c += 1;
    //         println!("{c} {} -> {}", a as char, b as char);

    //         for path in paths {
    //             println!("    {}", str::from_utf8(&path)?);
    //         }
    //     }
    // }
    //i
    //
    for code in input.split(|c| *c == b'\n') {
        solve(code, &padpaths);
    }
    let mut pos = b'A';
    // let mut pos2;
    let mut seq1: Vec<Vec<u8>> = vec![];
    for c in input {
        if c == b'\n' {
            break;
        }

        if c == pos {
            seq1.push(b'A');
        }

        let paths = padpaths.get(&pos).unwrap().get(&c).unwrap();

        print!("{}", c as char);

        for path in paths {}
        pos = c;
    }
    println!();
    for c in seq1 {
        print!("{}", c as char);
    }

    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}

fn solve(code: &[u8], padpaths: &HashMap<u8, HashMap<u8, Vec<Vec<u8>>>>) {
    let mut pos = b'A';

    let mut possible_paths: Vec<Vec<u8>> = vec![vec![]];

    for c in code {
        let paths0 = padpaths.get(&pos).unwrap().get(&c).unwrap();

        let mut new_paths: Vec<Vec<u8>> = possible_paths
            .into_iter()
            .map(move |p| {
                paths0
                    .into_iter()
                    .map(move |pn| p.into_iter().chain(*pn).collect::<Vec<u8>>())
            })
            .flatten()
            .collect();

        // possible_paths.iter().c

        // for pathx in possible_paths {
        //     for path_ in paths0 {
        //         panic_const_shr_overflow()
        //     }
        // }

        pos = *c;
    }
}

fn compute_padpaths(padpaths: &mut HashMap<u8, HashMap<u8, Vec<Vec<u8>>>>, pad: &[[u8; 3]]) {
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
            if from == to {
                continue;
            }

            let paths = padpaths.entry(*from).or_default().entry(*to).or_default();

            paths.push(
                repeat_n(if x0 < x1 { b'>' } else { b'<' }, x1.abs_diff(*x0))
                    .chain(repeat_n(
                        if y0 < y1 { b'^' } else { b'v' },
                        y1.abs_diff(*y0),
                    ))
                    .collect(),
            );

            if x0 != x1 && y0 != y1 {
                paths.push(paths[0].iter().copied().rev().collect());
            }
        }
    }
}
