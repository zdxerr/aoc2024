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
    'main: while idx0 <= idx1 {
        let idx0_is_file = idx0 % 2 == 0;
        let idx0_file_index = idx0 / 2;
        let idx1_is_file = idx1 % 2 == 0;
        let idx1_file_index = idx1 / 2;
        if idx0_is_file {
            for _ in 0..disc_layout[idx0] {
                print!("{idx0_file_index}");
                checksum += disc_index * idx0_file_index;
                disc_index += 1;
            }
        } else {
            for n in 0..disc_layout[idx0] {
                if idx1_is_file {
                    if disc_layout[idx1] > 0 {
                        disc_layout[idx1] -= 1;
                        print!("{idx1_file_index}");
                        checksum += disc_index * idx1_file_index;
                        disc_index += 1;
                    } else {
                        idx1 -= 2;
                        disc_layout[idx0] -= n;
                        continue 'main;
                    }
                } else {
                    idx1 -= 1;
                }
            }
        }
        idx0 += 1;
    }
    println!();
    println!("Solution: {} / Duration: {:.6?}", checksum, t0.elapsed());
    Ok(())
}
