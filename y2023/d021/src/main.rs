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

    for line in buff_reader.lines() {
        let line = line.expect("unable to read line");
        let (game, rest) = line.split_once(':').unwrap();
        let (_, game) = game.split_once(' ').unwrap();
        game.parse::<u32>();

        for draw in rest.split(';') {
            for color_and_number in draw.split(",") {
                let (color, number) = color_and_number.split_once(' ').unwrap();

                match color {
                    "red" => {}
                    "green" => {}
                    "blue" => {}
                    _ => {}
                }
                number.parse::<u32>();
                println!("{game} {color}");
            }
        }
    }
    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}
