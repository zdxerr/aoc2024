use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;
use std::ops::IndexMut;
use std::time::Instant;
use std::{env, fs, str};

const NL: u8 = b'\n';
const S: u8 = b'S';
const E: u8 = b'E';
const WALL: u8 = b'#';

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn left(self: &Direction) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
    fn right(self: &Direction) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
    fn next(self: &Direction, map: &[u8], xlen: usize, pos: usize) -> Option<usize> {
        let next_pos = match self {
            Direction::N => pos.checked_sub(xlen)?,
            Direction::E => pos.checked_add(1)?,
            Direction::S => pos.checked_add(xlen)?,
            Direction::W => pos.checked_sub(1)?,
        };
        if *map.get(next_pos)? == WALL {
            None
        } else {
            Some(next_pos)
        }
    }
}

#[derive(Debug)]
struct State {
    pre: Option<usize>,
    pos: usize,
    direction: Direction,
    score: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let map = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let xlen = map
        .iter()
        .position(|v| *v == NL)
        .expect("missing new line in input data")
        + 1;
    // println!("{}", str::from_utf8(&map)?);

    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashMap<usize, (u64, Vec<usize>)> = HashMap::new();

    queue.push_front(State {
        pre: None,
        pos: map
            .iter()
            .position(|v| *v == S)
            .expect("missing start position in input data"),
        direction: Direction::E,
        score: 0,
    });

    while let Some(state) = queue.pop_front() {
        if let Some((last_score, pres)) = visited.get_mut(&state.pos) {
            if *last_score < state.score {
                continue;
            } else if *last_score > state.score {
                visited.insert(state.pos, (state.score, state.pre.into_iter().collect()));
            } else {
                pres.extend(state.pre);
            }
        } else {
            visited.insert(state.pos, (state.score, state.pre.into_iter().collect()));
        }

        if let Some(pos) = state.direction.left().next(&map, xlen, state.pos) {
            queue.push_back(State {
                pre: Some(state.pos),
                pos,
                direction: state.direction.left(),
                score: state.score + 1001,
            });
        }
        if let Some(pos) = state.direction.next(&map, xlen, state.pos) {
            queue.push_back(State {
                pre: Some(state.pos),
                pos,
                direction: state.direction,
                score: state.score + 1,
            });
        }
        if let Some(pos) = state.direction.right().next(&map, xlen, state.pos) {
            queue.push_back(State {
                pre: Some(state.pos),
                pos,
                direction: state.direction.right(),
                score: state.score + 1001,
            });
        }
        // println!("{}", str::from_utf8(&new_map)?);
        // let mut input: String = String::new();
        // stdin().read_line(&mut input).unwrap();
    }
    let mut queue: Vec<usize> = Vec::new();
    let mut unique_on_shortest_paths: HashSet<usize> = HashSet::new();
    queue.push(
        map.iter()
            .position(|v| *v == E)
            .expect("missing end position in input data"),
    );
    let mut new_map: Vec<u8> = map.into_iter().collect();
    while let Some(pos) = queue.pop() {
        if !unique_on_shortest_paths.insert(pos) {
            continue;
        }
        new_map[pos] = b'o';
        let (_, pres) = visited.get(&pos).expect("position not visited");
        queue.extend(pres);
    }
    println!("{}", str::from_utf8(&new_map)?);
    println!(
        "Solution: {} / Duration: {:.6?}",
        unique_on_shortest_paths.len(),
        t0.elapsed()
    );
    Ok(())
}
