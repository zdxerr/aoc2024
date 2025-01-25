use std::error::Error;
use std::iter;
use std::mem;
use std::time::Instant;
use std::{env, fs};

#[derive(Debug)]
enum Block {
    File(u8, u64),
    Empty(u8),
}

fn display(layout: &[Block]) {
    for block in layout {
        print!(
            "{}",
            match block {
                Block::File(length, number) => format!("{number}").repeat(*length as usize),
                Block::Empty(length) => ".".repeat(*length as usize),
            }
        );
    }
    println!();
}

fn checksum(layout: &[Block]) -> u64 {
    layout
        .iter()
        .flat_map(|block| match block {
            Block::File(length, number) => iter::repeat_n(*number, *length as usize),
            Block::Empty(length) => iter::repeat_n(0, *length as usize),
        })
        .enumerate()
        .fold(0, |acc, (disc_index, number)| {
            acc + disc_index as u64 * u64::from(number)
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    let mut disc_layout: Vec<Block> = input
        .trim()
        .chars()
        .enumerate()
        .map(|(index, digit)| {
            let length = digit.to_digit(10).expect("unable to parse digit") as u8;
            match index % 2 == 0 {
                true => Block::File(length, (index / 2) as u64),
                false => Block::Empty(length),
            }
        })
        .collect();
    println!("Read {input_path}.");

    display(&disc_layout);

    let mut idx1 = disc_layout.len() - 1;

    while idx1 > 0 {
        if let Block::Empty(_) = disc_layout[idx1] {
            idx1 -= 1;
            continue;
        }

        match disc_layout[idx1] {
            Block::Empty(_) => {
                idx1 -= 1;
                continue;
            }
            Block::File(length1, _) => {
                for idx0 in 1..idx1 {
                    match disc_layout[idx0] {
                        Block::File(_, _) => continue,
                        Block::Empty(length0) if length0 >= length1 => {
                            // println!("{idx1} -> {idx0}");
                            let file1 = mem::replace(&mut disc_layout[idx1], Block::Empty(length1));
                            disc_layout[idx0] = file1;
                            let empty_space = length0 - length1;
                            if empty_space > 0 {
                                disc_layout.insert(idx0 + 1, Block::Empty(empty_space));
                            } else {
                                idx1 -= 1;
                            }
                            // display(&disc_layout);
                            break;
                        }
                        Block::Empty(_) => {}
                    }
                }
            }
        }
        idx1 -= 1;
    }

    println!(
        "Solution: {} / Duration: {:.6?}",
        checksum(&disc_layout),
        t0.elapsed()
    );
    Ok(())
}
