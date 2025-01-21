use std::error::Error;
use std::io::Empty;
use std::time::Instant;
use std::{env, fs};

fn display(layout: &Vec<File>) {
    layout.iter().for_each(|file| {
        print!(
            "{}",
            match file {
                File::Number(length, number) => format!("{number}").repeat(*length as usize),
                File::Empty(length) => ".".repeat(*length as usize),
            }
        );
    });
    println!();
}

#[derive(Debug)]
enum File {
    Number(u8, u8),
    Empty(u8),
}
// fn unpack()

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    let mut disc_layout: Vec<File> = input
        .trim()
        .chars()
        .enumerate()
        .map(|(index, digit)| {
            let length = digit.to_digit(10).expect("unable to parse digit") as u8;
            match index % 2 == 0 {
                true => File::Number(length, (index / 2) as u8),
                false => File::Empty(length),
            }
        })
        .collect();
    println!("Read {input_path}.");

    display(&disc_layout);

    let mut disc_index: usize = 0;
    let mut checksum = 0;

    let mut new_disc_layout: Vec<u8> = Vec::new();

    let (mut idx0, idx1) = (0, disc_layout.len() - 1);

    loop {
        let file1 = &disc_layout[idx1];
        if let File::Empty(_) = file1 {
            continue;
        }
        for idx0 in 1..disc_layout.len() - 1 {
            let file0 = &disc_layout[idx0];
            match file0 {
                File::Number(_, _) => continue,
                File::Empty(length) => disc_layout.insert(idx0, file1),
            }
            if let File::Number(_, _) = file0 {
                continue;
            }
            if file0.0 >= file1.0 {
                disc_layout.insert(idx0, file1);
            }
        }

        idx1 -= 1;

        if idx1 < 1 {
            break;
        }
    }

    // for file in disc_layout.iter().rev() {
    //     dbg!(file);

    //     for idx0 in 0..disc_layout.len() - 1 {}
    // }

    // let (mut idx0, mut idx1): (usize, usize) = (0, disc_layout.len() - 1);
    // let mut offest = 0;
    // for idx1 in (1..disc_layout.len()).rev() {
    //     dbg!(idx1);
    //     let idx1_is_file = idx1 % 2 == 0;
    //     let idx1_file_index = idx1 / 2;

    //     for idx0 in 1..idx1 + 1 {
    //         dbg!(idx0);
    //         let idx0_is_free = idx0 % 2 == 1;
    //         if idx0_is_free && disc_layout[idx0] >= disc_layout[idx1] {
    //             // move file
    //             //
    //             disc_layout.insert(idx0);
    //             disc_layout.insert(idx0, 0);

    //             break;
    //         }
    //     }
    // }
    // println!();
    println!("Solution: {} / Duration: {:.6?}", checksum, t0.elapsed());
    Ok(())
}
