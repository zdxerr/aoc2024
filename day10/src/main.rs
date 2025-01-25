use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const NL: u8 = b'\n';
const X0: u8 = b'0';
const X9: u8 = b'9';

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("no input path");
    let map = fs::read(&input_path)?;
    println!("Read {input_path}.");

    // println!("{}", std::str::from_utf8(&map)?);
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
    let results0: Vec<u64> = map
        .iter()
        .enumerate()
        .filter(|(_, value)| *value == &X0)
        .map(|(index, _)| explore(&map, &row_len, index, false))
        .collect();
    let solution0: u64 = results0.iter().sum();
    println!(
        "Solution Day 1: {} / Duration: {:.6?}",
        solution0,
        t0.elapsed()
    );

    let t1 = Instant::now();
    let results1: Vec<u64> = map
        .iter()
        .enumerate()
        .filter(|(_, value)| *value == &X0)
        .map(|(index, _)| explore(&map, &row_len, index, true))
        .collect();
    let solution1: u64 = results1.iter().sum();
    println!(
        "Solution Day 2: {} / Duration: {:.6?}",
        solution1,
        t1.elapsed()
    );

    Ok(())
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn next_index_value(
    map: &Vec<u8>,
    row_len: &usize,
    index: usize,
    direction: Direction,
) -> Option<usize> {
    let new_index = match direction {
        Direction::North => index.checked_sub(*row_len).unwrap_or(usize::MAX),
        Direction::East => index.checked_add(1).unwrap_or(usize::MAX),
        Direction::South => index.checked_add(*row_len).unwrap_or(usize::MAX),
        Direction::West => index.checked_sub(1).unwrap_or(usize::MAX),
    };
    let (expected_new_value, new_value) = (
        map.get(index).unwrap_or(&NL).checked_add(1).unwrap(),
        map.get(new_index).unwrap_or(&NL),
    );
    if new_value == &expected_new_value {
        Some(new_index)
    } else {
        None
    }
}

fn explore(map: &Vec<u8>, row_len: &usize, index: usize, find_unique_paths: bool) -> u64 {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    queue.push_back(index);
    let mut counter: u64 = 0;

    while !queue.is_empty() {
        let current_index = queue.pop_front().unwrap();

        if !find_unique_paths && !visited.insert(current_index) {
            continue;
        }

        if map.get(current_index).unwrap() == &X9 {
            counter += 1;
            continue;
        }

        queue.extend(next_index_value(
            map,
            row_len,
            current_index,
            Direction::North,
        ));
        queue.extend(next_index_value(
            map,
            row_len,
            current_index,
            Direction::East,
        ));
        queue.extend(next_index_value(
            map,
            row_len,
            current_index,
            Direction::South,
        ));
        queue.extend(next_index_value(
            map,
            row_len,
            current_index,
            Direction::West,
        ));
    }

    counter
}
