use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs, str};

const X: usize = 71;
const Y: usize = 71;
const N: usize = 1024;

const BLOCK: u8 = b'#';

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
        .fold([[b'.'; X]; Y], |mut map, mut parsed_line| {
            let (x, y) = (
                parsed_line.next().unwrap().unwrap(),
                parsed_line.next().unwrap().unwrap(),
            );
            map[y][x] = BLOCK;
            map
        });

    // println!("{}", str::from_utf8(&map.join(&b'\n'))?);

    let mut queue: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    queue.push(Reverse((0, 0, 0)));
    while let Some(Reverse((d, x, y))) = queue.pop() {
        if x == X - 1 && y == Y - 1 {
            println!("Solution: {} / Duration: {:.6?}", d, t0.elapsed());
            return Ok(());
        }
        if map.get(y).unwrap_or(&[BLOCK; X]).get(x).unwrap_or(&BLOCK) == &BLOCK {
            continue;
        }
        if visited.get(&(x, y)).unwrap_or(&usize::MAX) <= &d {
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

    println!("No solution found / Duration: {:.6?}", t0.elapsed());
    Ok(())
}
