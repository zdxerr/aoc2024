use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;
use std::string;
use std::time::Instant;
use std::{env, fs, str};

const X: usize = 7;
const Y: usize = 7;
const N: usize = 12;

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let map = input
        .trim()
        .lines()
        .take(N)
        .map(|line| line.splitn(2, ',').map(str::parse::<usize>))
        .fold(['.'; X * Y], |mut map, mut d| {
            map[d.next().unwrap().unwrap() + d.next().unwrap().unwrap() * X] = '#';
            map
        });
    dbg!(map);
    let solution = 0;
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
