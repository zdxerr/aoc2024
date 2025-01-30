use regex::Regex;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const X: usize = 101;
const Y: usize = 103;

#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn step(self: &mut Robot) {
        self.x = ((X + self.x).wrapping_add_signed(self.vx)) % X;
        self.y = ((Y + self.y).wrapping_add_signed(self.vy)) % Y;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(r"p=(\d+),(\d+)\s*v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<Robot> = expr
        .captures_iter(input.as_str())
        .map(|c| Robot {
            x: c.get(1).unwrap().as_str().parse().unwrap(),
            y: c.get(2).unwrap().as_str().parse().unwrap(),
            vx: c.get(3).unwrap().as_str().parse().unwrap(),
            vy: c.get(4).unwrap().as_str().parse().unwrap(),
        })
        .collect();

    let mut solution: Option<u64> = Option::None;
    for seconds in 0..u64::MAX {
        if strung(&robots) {
            render(&robots);
            solution = Option::Some(seconds);
            break;
        }
        robots.iter_mut().for_each(Robot::step);
    }
    println!("Solution: {:?} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

fn render(robots: &Vec<Robot>) {
    let mut map: [u8; X * Y] = [0; X * Y];
    for robot in robots {
        map[robot.x + X * robot.y] += 1;
    }

    println!(
        "{}",
        map.iter()
            .map(u8::to_string)
            .map(|number| match number.as_str() {
                "0" => String::from(" "),
                _ => number,
            })
            .enumerate()
            .map(|(index, number)| {
                if index % X == 0 {
                    String::from("\n") + number.as_str()
                } else {
                    number
                }
            })
            .reduce(|acc, number| acc + number.as_str())
            .unwrap()
    );
}

fn strung(robots: &Vec<Robot>) -> bool {
    let mut map: [u8; X * Y] = [0; X * Y];
    for robot in robots {
        map[robot.x + X * robot.y] += 1;
    }

    let mut count: usize = 0;
    for number in map {
        if number == 0 {
            count = 0;
        } else {
            count += 1;
            if count > 20 {
                return true;
            }
        }
    }
    false
}
