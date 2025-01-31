use std::error::Error;
use std::time::Instant;
use std::{env, fs, str};

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
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");
    // let mut map = vec![];
    // let mut sequence = vec![];
    //
    let xlen = input.iter().position(|value| *value == NL).unwrap();
    let pos = input.iter().position(|value| *value == ROBOT).unwrap();
    let sequence_index = input
        .iter()
        .position(|value| match *value {
            UP | RIGHT | DOWN | LEFT => true,
            _ => false,
        })
        .unwrap();

    let map = &input[0..sequence_index - 2];
    let sequence = &input[sequence_index..];
    dbg!(xlen, pos);
    println!("{}", str::from_utf8(map)?);
    println!("____________________________");
    println!("{}", str::from_utf8(sequence)?);

    for direction in sequence {
        match *direction {
            UP => {}
            RIGHT => {}
            DOWN => {}
            LEFT => {}
            _ => continue,
        }
    }
    let solution = 0;
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

fn move_(map: &mut Vec<u8>, from: usize, to: usize) {}
