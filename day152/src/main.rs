use std::error::Error;
use std::io::stdin;
use std::time::Instant;
use std::{env, fs, str};
const FREE: u8 = b'.';
const ROBOT: u8 = b'@';
const BOX: u8 = b'O';
const WALL: u8 = b'#';
const NL: u8 = b'\n';

const UP: u8 = b'^';
const RIGHT: u8 = b'>';
const DOWN: u8 = b'v';
const LEFT: u8 = b'<';

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let mut input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let xlen = input.iter().position(|value| *value == NL).unwrap() + 1;
    let mut pos = input.iter().position(|value| *value == ROBOT).unwrap();
    let sequence_index = input
        .iter()
        .position(|value| match *value {
            UP | RIGHT | DOWN | LEFT => true,
            _ => false,
        })
        .unwrap();
    let (map, sequence) = input.split_at_mut(sequence_index);
    let ylen = map.len() / xlen;

    let map: Vec<u8> = map
        .iter()
        .flat_map(|v| match *v {
            BOX => [b'[', b']'],
            ROBOT => [ROBOT, FREE],
            NL => [b' ', NL],
            _ => [*v, *v],
        })
        .collect();
    println!("{}", str::from_utf8(map.trim_ascii())?);
    // for &mut direction in sequence {
    //     // println!("{}", str::from_utf8(map.trim_ascii())?);
    //     // println!("{}", direction as char);
    //     // let mut input: String = String::new();
    //     // stdin().read_line(&mut input).unwrap();
    //     if let Some(next_pos) = next(direction, pos, xlen, ylen) {
    //         let mut end_pos = next_pos;

    //         while map[end_pos] == BOX {
    //             if let Some(end_pos2) = next(direction, end_pos, xlen, ylen) {
    //                 end_pos = end_pos2;
    //             } else {
    //                 break;
    //             }
    //         }

    //         if end_pos == next_pos {
    //             if map[next_pos] == FREE {
    //                 map[next_pos] = ROBOT;
    //                 map[pos] = FREE;
    //                 pos = next_pos;
    //             }
    //             continue;
    //         }

    //         if map[end_pos] == FREE {
    //             map[next_pos] = ROBOT;
    //             map[pos] = FREE;
    //             pos = next_pos;
    //             map[end_pos] = BOX;
    //         }
    //     }
    // }
    // let solution = map
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(index, value)| if *value == BOX { Some(index) } else { None })
    //     .fold(0, |acc, index| acc + (index / xlen) * 100 + (index % xlen));
    let solution = 0;
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

fn next(direction: u8, pos: usize, xlen: usize, ylen: usize) -> Option<usize> {
    let next_pos = match direction {
        UP => pos.checked_sub(xlen)?,
        RIGHT => pos.checked_add(1)?,
        DOWN => pos.checked_add(xlen)?,
        LEFT => pos.checked_sub(1)?,
        _ => return None,
    };
    if next_pos < xlen * ylen {
        Some(next_pos)
    } else {
        None
    }
}
