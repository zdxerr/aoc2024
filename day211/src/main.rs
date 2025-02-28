use core::str;
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

    let numcoords: Vec<(usize, usize, u8)> = numpad
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, num)| (x, y, *num)))
        .flatten()
        .collect();
    let dcoords: Vec<(usize, usize, u8)> = dpad
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, num)| (x, y, *num)))
        .flatten()
        .collect();

    let mut padpaths: HashMap<u8, HashMap<u8, Vec<Vec<u8>>>> = HashMap::new();

    let start = (2_usize, 3_usize);
    // let next = (start.0, start.1 - 1);

    let mut stack = vec![(numpad[start.1][start.0], 2_usize, 3_usize, vec![])];

    while let Some((start, x, y, path)) = stack.pop() {
        // dbg!(start, x, y, &path);c
        if x > 0 {
            stack.push((
                start,
                x - 1,
                y,
                path.iter().copied().chain([b'<'].into_iter()).collect(),
            ));
            if start != numpad[y][x] {
                stack.push((numpad[y][x], x - 1, y, vec![b'<']));
            }
        }
        if y > 0 {
            stack.push((
                start,
                x,
                y - 1,
                path.iter().copied().chain([b'^'].into_iter()).collect(),
            ));
            if start != numpad[y][x] {
                stack.push((numpad[y][x], x, y - 1, vec![b'^']));
            }
        }
        // dbg!(start, x, y, &path);
        println!(
            "{} {} {}",
            start as char,
            numpad[y][x] as char,
            str::from_utf8(&path)?
        );
        padpaths
            .entry(start)
            .or_default()
            .entry(numpad[y][x])
            .or_default()
            .push(path);
        // break;
    }

    // while pos != (0, 0) {}

    // dbg!(numcoords, dcoords, &padpaths, &padpaths.len());

    for (a, p) in padpaths {
        for (b, paths) in p {
            println!("{} -> {}", a as char, b as char);

            for path in paths {
                println!("    {}", str::from_utf8(&path)?);
            }
        }
    }

    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
