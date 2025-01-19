use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    let mut disc_layout: Vec<u8> = input
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).expect("unable to parse digit") as u8)
        .collect();
    println!("Read {input_path}.");

    let mut disc_index: usize = 0;
    let mut checksum = 0;

    let (mut idx0, mut idx1): (usize, usize) = (0, disc_layout.len() - 1);
    let mut offest = 0;
    for idx1 in (1..disc_layout.len()).rev() {
        dbg!(idx1);
        let idx1_is_file = idx1 % 2 == 0;
        let idx1_file_index = idx1 / 2;

        for idx0 in 1..idx1 + 1 {
            dbg!(idx0);
            let idx0_is_free = idx0 % 2 == 1;
            if idx0_is_free && disc_layout[idx0] >= disc_layout[idx1] {
                // move file
                //
                disc_layout.insert(idx0);
                disc_layout.insert(idx0, 0);

                break;
            }
        }
    }
    println!();
    println!("Solution: {} / Duration: {:.6?}", checksum, t0.elapsed());
    Ok(())
}
