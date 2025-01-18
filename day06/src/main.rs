use core::panic;
use std::env;
use std::fs;
use std::str;

const FREE: u8 = b'.';
const UP: u8 = b'^';
const DOWN: u8 = b'v';
const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';
const NL: u8 = b'\n';
const BLOCK: u8 = b'#';
const BLOCK_ADDED: u8 = b'O';

#[derive(Clone, Debug)]
pub struct Map {
    map: Vec<u8>,
    xlen: usize,
    pos: usize,
    direction: u8,
}

pub enum PositionState {
    Free,
    Blocked,
    Off,
    VisitedInSameDirection,
    VisitedInDifferentDirection,
}

impl Map {
    pub fn new(file_path: &String) -> Self {
        let map = fs::read(file_path).expect(&format!("Unable to read file: {file_path}"));
        println!("Loaded: {file_path}");
        let max = map.len();
        let mut xlen = max;
        let mut pos = max;
        for (index, symbol) in map.iter().enumerate() {
            match *symbol {
                NL => {
                    if xlen == max {
                        xlen = index
                    }
                }
                UP | DOWN | LEFT | RIGHT => {
                    if pos == max {
                        pos = index
                    }
                }
                _ => (),
            }
            if pos != max && xlen != max {
                break;
            }
        }
        let direction = map[pos];
        Self {
            map,
            xlen,
            pos,
            direction,
        }
    }

    pub fn value_at(&self, pos: usize) -> Option<u8> {
        let value = self.map.get(pos).unwrap_or(&NL);
        match *value {
            NL => None,
            _ => Some(*value),
        }
    }

    pub fn off(&self, pos: usize) -> bool {
        match self.map.get(pos) {
            Some(&NL) | None => true,
            _ => panic!(),
        }
    }

    pub fn position_state(&self, pos: usize) -> PositionState {
        match self.map.get(pos) {
            None | Some(&NL) => PositionState::Off,
            Some(&BLOCK) | Some(&BLOCK_ADDED) => PositionState::Blocked,
            Some(&FREE) => PositionState::Free,
            Some(direction) if direction == &self.direction => {
                PositionState::VisitedInSameDirection
            }
            _ => PositionState::VisitedInDifferentDirection,
        }
    }

    pub fn display(&self) {
        let x = self.pos % (self.xlen + 1);
        let y = self.pos / (self.xlen + 1);
        println!(
            "Position: {} X: {} Y: {} Direction: {}",
            self.pos, x, y, self.direction as char
        );
        let mut map = self.map.clone();
        map[self.pos] = 'X' as u8;
        let map = str::from_utf8(&map).unwrap();
        for (index, line) in map.lines().enumerate() {
            println!("{index:03} {line}");
        }
    }

    pub fn next(&self) -> usize {
        let delta: isize = match self.direction {
            UP => -(self.xlen as isize + 1),
            DOWN => self.xlen as isize + 1,
            LEFT => -1,
            RIGHT => 1,
            value => panic!("unexpected value at current position: {value}"),
        };
        self.pos.checked_add_signed(delta).unwrap_or(usize::MAX)
    }

    pub fn turn(&mut self) {
        self.direction = match self.direction {
            UP => RIGHT,
            DOWN => LEFT,
            RIGHT => DOWN,
            LEFT => UP,
            _ => panic!(),
        };
    }

    pub fn block(&mut self, pos: usize) {
        self.map[pos] = BLOCK_ADDED;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut map = Map::new(&file_path);
    let mut counter = 1;
    let mut loop_counter = 0;
    for step in 1..9999999999999u64 {
        // map.display();
        // println!("#### {step:012}");

        let next_pos = map.next();

        dbg!(step, next_pos);

        match map.position_state(next_pos) {
            PositionState::Off => {
                println!("OFF OFF OFF");
                break;
            }
            PositionState::Blocked => {
                map.turn();
                continue;
            }
            PositionState::VisitedInSameDirection => panic!("LOOP"),
            PositionState::VisitedInDifferentDirection => {}
            PositionState::Free => {
                counter += 1;

                let mut mapb = map.clone();
                mapb.block(next_pos);
                mapb.turn();

                for stepb in 1..999999999999u64 {
                    // dbg!(stepb);
                    // mapb.display();
                    let next_posb = mapb.next();

                    // dbg!(next_posb);

                    match mapb.position_state(next_posb) {
                        PositionState::Off => {
                            // println!("OFF OFF OFF");
                            break;
                        }
                        PositionState::Blocked => {
                            mapb.turn();
                            continue;
                        }
                        PositionState::VisitedInSameDirection => {
                            loop_counter += 1;
                            println!("LOOP");
                            mapb.display();
                            break;
                        }
                        PositionState::VisitedInDifferentDirection => {
                            // println!("{step:09} / {stepb:09} {next_posb:05} VISITED");
                            // mapb.display();
                        }
                        PositionState::Free => {} // _ => {}
                    }

                    if mapb.map[next_posb] == FREE {
                        mapb.map[next_posb] = mapb.direction;
                    }

                    mapb.pos = next_posb;
                }
                map.map[next_pos] = map.direction;
            }
        }
        map.pos = next_pos;
    }
    dbg!(counter);
    dbg!(loop_counter);
}
