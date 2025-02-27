use std::error::Error;
use std::time::Instant;
use std::{env, fs};

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

    dbg!(numcoords, dcoords);

    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
