use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;
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
}

#[derive(Debug)]
struct State {
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
    let mut visited: HashMap<usize, u64> = HashMap::new();

    queue.push_front(State {
        pos: map
            .iter()
            .position(|v| *v == S)
            .expect("missing start position in input data"),
        direction: Direction::E,
        score: 0,
    });

    while let Some(state) = queue.pop_front() {
        let last_score = visited.entry(state.pos).or_insert(state.score);

        // let mut new_map = map.clone();
        // new_map[state.pos] = b'@';

        if *last_score < state.score {
            continue;
        }

        visited.insert(state.pos, state.score);

        if let Some(pos) = next(&map, xlen, state.pos, state.direction.left()) {
            // new_map[pos] = b'o';
            queue.push_back(State {
                pos,
                direction: state.direction.left(),
                score: state.score + 1001,
            });
        }
        if let Some(pos) = next(&map, xlen, state.pos, state.direction) {
            // new_map[pos] = b'o';
            queue.push_back(State {
                pos,
                direction: state.direction,
                score: state.score + 1,
            });
        }
        if let Some(pos) = next(&map, xlen, state.pos, state.direction.right()) {
            // new_map[pos] = b'o';
            queue.push_back(State {
                pos,
                direction: state.direction.right(),
                score: state.score + 1001,
            });
        }
        // println!("{}", str::from_utf8(&new_map)?);
        // let mut input: String = String::new();
        // stdin().read_line(&mut input).unwrap();
    }
    let solution = visited
        .get(
            &map.iter()
                .position(|v| *v == E)
                .expect("missing end position in input data"),
        )
        .expect("end position not reached");
    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

fn next(map: &[u8], xlen: usize, pos: usize, direction: Direction) -> Option<usize> {
    let next_pos = match direction {
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
