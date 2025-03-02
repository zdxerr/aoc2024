use core::{panic, str};
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let numpad = [
        [b'7', b'8', b'9'],
        [b'4', b'5', b'6'],
        [b'1', b'2', b'3'],
        [b' ', b'0', b'A'],
    ];
    let dpad = [[b' ', b'^', b'A'], [b'<', b'v', b'>']];

    let mut padpaths: HashMap<u8, HashMap<u8, Vec<Vec<u8>>>> = HashMap::new();
    // let mut padpaths2: HashMap<(u8, u8), Vec<Vec<u8>>> = HashMap::new();
    //

    let stack: Vec<(u8, usize, usize)> = numpad
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, num)| **num != b' ')
                .map(move |(x, num)| (*num, x, y))
        })
        .flatten()
        .collect();

    for (from, x0, y0) in &stack {
        for (to, x1, y1) in &stack {
            if from == to {
                continue;
            }

            let horizontal = vec![if x0 < x1 { b'>' } else { b'<' }; x1.abs_diff(*x0)];
            let vertical = vec![if y0 < y1 { b'^' } else { b'v' }; y1.abs_diff(*y0)];

            padpaths
                .entry(*from)
                .or_default()
                .entry(*to)
                .or_default()
                .extend(
                    [
                        [horizontal.clone(), vertical.clone()].concat(), //.iter().flatten().copied().collect(),
                        [vertical, horizontal].concat(),
                        // [vertical, horizontal].iter().flatten().copied().collect(),
                    ]
                    .into_iter(),
                );
        }
    }

    // let mut padpaths3: HashMap<(u8, u8), Vec<Vec<u8>>> = HashMap::new();
    // for ((a, b), paths) in &padpaths2 {
    //     padpaths3
    //         .entry((*b, *a))
    //         .or_default()
    //         .extend(paths.iter().map(|path| {
    //             path.iter()
    //                 .map(|c| match c {
    //                     b'^' => b'v',
    //                     b'<' => b'>',
    //                     _ => panic!("AHH"),
    //                 })
    //                 .collect()
    //         }));
    // }
    // padpaths2.extend(padpaths3.into_iter());
    // // while pos != (0, 0) {}

    // // dbg!(numcoords, dcoords, &padpaths, &padpaths.len());
    // for (a, b) in padpaths2.keys() {
    //     println!("{}->{}", *a as char, *b as char);
    // }
    let mut c = 0;
    for (a, p) in padpaths {
        for (b, paths) in p {
            c += 1;
            println!("{c} {} -> {}", a as char, b as char);

            for path in paths {
                println!("    {}", str::from_utf8(&path)?);
            }
        }
    }

    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
