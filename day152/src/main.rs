use core::panic;
use std::error::Error;
use std::io::stdin;
use std::time::Instant;
use std::{env, fs, str};
const FREE: u8 = b'.';
const ROBOT: u8 = b'@';
const BOX: u8 = b'O';
const BOXL: u8 = b'[';
const BOXR: u8 = b']';
const WALL: u8 = b'#';
const NL: u8 = b'\n';

const UP: u8 = b'^';
const RIGHT: u8 = b'>';
const DOWN: u8 = b'v';
const LEFT: u8 = b'<';

struct State {
    map: Vec<u8>,
    xlen: usize,
    pos: usize,
}

impl State {
    fn new(map: &[u8]) -> State {
        let map: Vec<u8> = map
            .trim_ascii()
            .iter()
            .flat_map(|v| match *v {
                BOX => vec![BOXL, BOXR],
                ROBOT => vec![ROBOT, FREE],
                NL => vec![NL],
                _ => vec![*v, *v],
            })
            .collect();
        let xlen = map.iter().position(|&v| v == NL).unwrap() + 1;
        let pos = map.iter().position(|&v| v == ROBOT).unwrap();
        State { map, xlen, pos }
    }

    fn x(self: &State) -> usize {
        self.pos % self.xlen
    }

    fn y(self: &State) -> usize {
        self.pos / self.xlen
    }

    fn display(self: &State) -> Result<(), Box<dyn Error>> {
        println!("{}", str::from_utf8(&self.map)?);
        println!("x={}, y={}, xlen={}", self.x(), self.y(), self.xlen);
        Ok(())
    }

    fn next(self: &State, pos: usize, direction: u8) -> Option<usize> {
        let next_pos = match direction {
            UP => pos.checked_sub(self.xlen)?,
            RIGHT => pos.checked_add(1)?,
            DOWN => pos.checked_add(self.xlen)?,
            LEFT => pos.checked_sub(1)?,
            _ => return None,
        };
        if next_pos < self.map.len() {
            Some(next_pos)
        } else {
            // println!("NEXT {}", next_pos);
            None
        }
    }

    fn moveable(self: &mut State, pos: usize, direction: u8) -> bool {
        if let Some(next_pos) = self.next(pos, direction) {
            // println!("{} | {} | {}", pos, next_pos, self.map[next_pos] as char);
            match self.map[next_pos] {
                FREE => true,
                BOXL => match direction {
                    RIGHT | LEFT => self.moveable(next_pos, direction),
                    UP | DOWN => {
                        self.moveable(next_pos, direction) && self.moveable(next_pos + 1, direction)
                    }
                    _ => panic!(),
                },
                BOXR => match direction {
                    RIGHT | LEFT => self.moveable(next_pos, direction),
                    UP | DOWN => {
                        self.moveable(next_pos, direction) && self.moveable(next_pos - 1, direction)
                    }
                    _ => panic!(),
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn shift(self: &mut State, pos: usize, direction: u8) -> bool {
        if let Some(next_pos) = self.next(pos, direction) {
            match self.map[next_pos] {
                FREE => {
                    self.map[next_pos] = self.map[pos];
                    self.map[pos] = FREE;
                    if pos == self.pos {
                        self.pos = next_pos;
                    }
                    true
                }
                BOXL => match direction {
                    RIGHT | LEFT => {
                        self.shift(next_pos, direction);
                        self.shift(pos, direction);
                        true
                    }
                    UP | DOWN => {
                        self.shift(next_pos, direction);
                        self.shift(next_pos + 1, direction);
                        self.shift(pos, direction);
                        true
                    }
                    _ => panic!(),
                },
                BOXR => match direction {
                    RIGHT | LEFT => {
                        self.shift(next_pos, direction);
                        self.shift(pos, direction);
                        true
                    }
                    UP | DOWN => {
                        self.shift(next_pos, direction);
                        self.shift(next_pos - 1, direction);
                        self.shift(pos, direction);
                        true
                    }
                    _ => panic!(),
                },
                _ => false,
            }
        } else {
            false
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let sequence_index = input
        .iter()
        .position(|value| match *value {
            UP | RIGHT | DOWN | LEFT => true,
            _ => false,
        })
        .unwrap();
    let (map, sequence) = input.split_at(sequence_index);
    let mut state = State::new(map);

    for &direction in sequence {
        // state.display()?;

        let moveable = state.moveable(state.pos, direction);
        // println!("{} {}", direction as char, moveable);
        // let mut input: String = String::new();
        // stdin().read_line(&mut input).unwrap();

        if moveable {
            state.shift(state.pos, direction);
        }
    }
    let solution = state
        .map
        .iter()
        .enumerate()
        .filter_map(|(index, value)| if *value == BOXL { Some(index) } else { None })
        .fold(0, |acc, index| {
            acc + (index / state.xlen) * 100 + (index % state.xlen)
        });
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
