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

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

fn explore(map: &Vec<u8>, row_len: usize, index: usize, explored: &mut Vec<bool>) -> u64 {
    let plant = map[index];
    if plant == NL || explored[index] {
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

        let directions: [usize; 8] = [
            index.checked_sub(row_len).unwrap_or(usize::MAX),
            index
                .checked_sub(row_len)
                .unwrap_or(usize::MAX)
                .checked_add(1)
                .unwrap_or(usize::MAX),
            index.checked_add(1).unwrap_or(usize::MAX),
            index
                .checked_add(row_len)
                .unwrap_or(usize::MAX)
                .checked_add(1)
                .unwrap_or(usize::MAX),
            index.checked_add(row_len).unwrap_or(usize::MAX),
            index
                .checked_add(row_len)
                .unwrap_or(usize::MAX)
                .checked_sub(1)
                .unwrap_or(usize::MAX),
            index.checked_sub(1).unwrap_or(usize::MAX),
            index
                .checked_sub(row_len)
                .unwrap_or(usize::MAX)
                .checked_sub(1)
                .unwrap_or(usize::MAX),
        ];
        let values: [&u8; 8] = directions.map(|index| map.get(index).unwrap_or(&NL));

        // hint: the number of corners is equal to the number of edges
        let corners: [bool; 8] = [
            // convex
            *values[0] != plant && *values[2] != plant,
            *values[2] != plant && *values[4] != plant,
            *values[4] != plant && *values[6] != plant,
            *values[6] != plant && *values[0] != plant,
            // concave
            *values[0] == plant && *values[1] != plant && *values[2] == plant,
            *values[2] == plant && *values[3] != plant && *values[4] == plant,
            *values[4] == plant && *values[5] != plant && *values[6] == plant,
            *values[6] == plant && *values[7] != plant && *values[0] == plant,
        ];

        perimeter += corners.iter().filter(|edge| **edge).count() as u64;

        if *values[0] == plant {
            queue.push(directions[0]);
        }
        if *values[2] == plant {
            queue.push(directions[2]);
        }
        if *values[4] == plant {
            queue.push(directions[4]);
        }
        if *values[6] == plant {
            queue.push(directions[6]);
        }
    }
    area * perimeter
}
