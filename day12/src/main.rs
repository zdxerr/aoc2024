use std::error::Error;
use std::time::Instant;
use std::{env, fs};
const NL: u8 = b'\n';

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("no input path");
    let map = fs::read(&input_path)?;
    println!("Read {input_path}.");

    println!("{}", std::str::from_utf8(&map)?);
    let row_len = map
        .iter()
        .enumerate()
        .find_map(
            |(index, element)| {
                if element == &NL {
                    Some(index)
                } else {
                    None
                }
            },
        )
        .unwrap()
        .checked_add(1)
        .unwrap();
    let t0 = Instant::now();

    let mut explored: Vec<bool> = vec![false; map.len()];

    let solution: u64 = (0..map.len())
        .map(|index| explore(&map, row_len, index, &mut explored))
        .sum();

    println!(
        "Solution Day 1: {} / Duration: {:.6?}",
        solution,
        t0.elapsed()
    );
    Ok(())
}

fn explore(map: &Vec<u8>, row_len: usize, index: usize, explored: &mut Vec<bool>) -> u64 {
    let oplant = map[index];
    if oplant == NL || explored[index] {
        return 0;
    }
    let mut queue: Vec<usize> = vec![index];

    let mut area: u64 = 0;
    let mut perimeter: u64 = 0;

    while !queue.is_empty() {
        let index = queue.pop().unwrap();
        if explored[index] {
            continue;
        }
        explored[index] = true;
        area += 1;

        let mut enqueue = |index: usize| {
            let plant = map.get(index).unwrap_or(&NL);
            if oplant == *plant {
                if !explored[index] {
                    queue.push(index);
                }
            } else {
                perimeter += 1;
            }
        };

        let north_index = index.checked_sub(row_len).unwrap_or(usize::MAX);
        enqueue(north_index);

        let east_index = index.checked_add(1).unwrap_or(usize::MAX);
        enqueue(east_index);

        let south_index = index.checked_add(row_len).unwrap_or(usize::MAX);
        enqueue(south_index);

        let west_index = index.checked_sub(1).unwrap_or(usize::MAX);
        enqueue(west_index);
    }
    area * perimeter
}
