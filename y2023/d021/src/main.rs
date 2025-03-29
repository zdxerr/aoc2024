use std::io::{BufRead, BufReader, Error};
use std::time::Instant;
use std::{env, fs};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn main() -> Result<(), Error> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let buff_reader = BufReader::new(fs::File::open(input_path)?);

    let mut solution: u32 = 0;

    'outer: for line in buff_reader.lines() {
        let line = line.expect("unable to read line");
        let (game, rest) = line.split_once(':').unwrap();
        let (_, game) = game.split_once(' ').unwrap();

        for draw in rest.split(';') {
            for number_and_color in draw.split(",") {
                let (number, color) = number_and_color.trim().split_once(' ').unwrap();
                // dbg!(number);
                let number: u32 = number.parse().unwrap();
                match color {
                    "red" if number > RED => continue 'outer,
                    "green" if number > GREEN => continue 'outer,
                    "blue" if number > BLUE => continue 'outer,
                    _ => {}
                }
            }
        }
        solution += game.parse::<u32>().unwrap();
    }
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
