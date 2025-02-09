use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

const NL: u8 = b'\n';
const S: u8 = b'S';
const E: u8 = b'E';
const WALL: u8 = b'#';

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    index: usize,
    direction: Direction,
    score: u64,
}

impl State {
    fn next(self: &State, map: &[u8], xlen: usize) -> Option<State> {
        let next_pos = match self.direction {
            Direction::N => self.index.checked_sub(xlen)?,
            Direction::E => self.index.checked_add(1)?,
            Direction::S => self.index.checked_add(xlen)?,
            Direction::W => self.index.checked_sub(1)?,
        };
        if *map.get(next_pos)? == WALL {
            None
        } else {
            Some(State {
                index: next_pos,
                direction: self.direction,
                score: self.score + 1,
            })
        }
    }
    fn prev(
        self: &State,
        map: &[u8],
        xlen: usize,
        visited: Option<&HashMap<(usize, Direction), u64>>,
    ) -> Option<State> {
        let prev_pos = match self.direction {
            Direction::S => self.index.checked_sub(xlen)?,
            Direction::W => self.index.checked_add(1)?,
            Direction::N => self.index.checked_add(xlen)?,
            Direction::E => self.index.checked_sub(1)?,
        };
        if *map.get(prev_pos)? == WALL {
            None
        } else {
            Some(State {
                index: prev_pos,
                direction: self.direction,
                score: if visited.is_none() {
                    self.score - 1
                } else {
                    *visited?.get(&(prev_pos, self.direction))?
                },
            })
        }
    }
    fn left(self: &State, visited: Option<&HashMap<(usize, Direction), u64>>) -> Option<State> {
        let direction = match self.direction {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        };
        Some(State {
            index: self.index,
            direction,
            score: if visited.is_none() {
                self.score + 1000
            } else {
                *visited?.get(&(self.index, direction))?
            },
        })
    }
    fn right(self: &State, visited: Option<&HashMap<(usize, Direction), u64>>) -> Option<State> {
        let direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
        Some(State {
            index: self.index,
            direction,
            score: if visited.is_none() {
                self.score + 1000
            } else {
                *visited?.get(&(self.index, direction))?
            },
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<(usize, Direction), u64> = HashMap::new();

    queue.push(State {
        index: map
            .iter()
            .position(|v| *v == S)
            .expect("missing start position in input data"),
        direction: Direction::E,
        score: 0,
    });

    while let Some(state) = queue.pop() {
        let visit_score = visited
            .entry((state.index, state.direction))
            .or_insert(u64::MAX);

        if state.score >= *visit_score {
            continue;
        }
        *visit_score = state.score;

        queue.extend(state.next(&map, xlen));
        queue.extend(state.left(None));
        queue.extend(state.right(None));
    }
    let mut queue: Vec<State> = Vec::new();
    let mut shortest_paths: HashSet<usize> = HashSet::new();
    let end_index = map
        .iter()
        .position(|v| *v == E)
        .expect("missing end position in input data");

    queue.push(State {
        index: end_index,
        direction: Direction::E,
        score: *visited.get(&(end_index, Direction::E)).unwrap_or(&u64::MAX),
    });

    while let Some(state) = queue.pop() {
        shortest_paths.insert(state.index);
        if map[state.index] == S {
            continue;
        }

        queue.extend(
            [
                state.prev(&map, xlen, Some(&visited)),
                state.left(Some(&visited)),
                state.right(Some(&visited)),
            ]
            .iter()
            .flatten()
            .filter(|prev_state| prev_state.score < state.score),
        );
    }
    println!(
        "Solution: {} / Duration: {:.6?}",
        shortest_paths.len(),
        t0.elapsed()
    );
    Ok(())
}
