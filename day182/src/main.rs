use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const X: usize = 71;
const Y: usize = 71;
const N: usize = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let map = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .enumerate()
        .fold(
            [[usize::MAX; X]; Y],
            |mut map: [[usize; X]; Y], (index, (x, y)): (usize, (usize, usize))| {
                map[y][x] = index;
                map
            },
        );

    let mut lower = N;
    let mut upper = input.lines().count();

    if let Some(n) = loop {
        let pivot = lower + (upper - lower).div_ceil(2);

        if let Some(_) = solve(&map, pivot) {
            if lower == pivot {
                break None;
            }
            lower = pivot;
        } else {
            if upper == pivot {
                break Some(pivot);
            }
            upper = pivot;
        }
    } {
        println!(
            "Solution: {} {} / Duration: {:.6?}",
            n,
            input.lines().nth(n).ok_or("input line not found")?,
            t0.elapsed()
        );
    } else {
        println!("No solution found / Duration: {:.6?}", t0.elapsed());
    }
    Ok(())
}

fn solve(map: &[[usize; X]; Y], n: usize) -> Option<usize> {
    let mut queue: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    queue.push(Reverse((0, 0, 0)));
    while let Some(Reverse((d, x, y))) = queue.pop() {
        if x == X - 1 && y == Y - 1 {
            return Some(d);
        }
        if !(0..X).contains(&x) || !(0..Y).contains(&y) {
            continue;
        }
        if let Some(previos_d) = visited.get(&(x, y)) {
            if *previos_d <= d {
                continue;
            }
        }
        // is this position blocked now?
        if map[y][x] <= n {
            continue;
        }

        visited.insert((x, y), d);

        queue.extend(
            [
                Reverse((d.wrapping_add(1), x, y.wrapping_sub(1))),
                Reverse((d.wrapping_add(1), x.wrapping_add(1), y)),
                Reverse((d.wrapping_add(1), x, y.wrapping_add(1))),
                Reverse((d.wrapping_add(1), x.wrapping_sub(1), y)),
            ]
            .iter(),
        );
    }
    None
}
