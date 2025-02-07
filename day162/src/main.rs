use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::stdin;
use std::time::Instant;
use std::{env, fs, str};

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

#[derive(Debug, Eq, PartialEq)]
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
    fn left(self: &State) -> Option<State> {
        Some(State {
            index: self.index,
            direction: match self.direction {
                Direction::N => Direction::W,
                Direction::E => Direction::N,
                Direction::S => Direction::E,
                Direction::W => Direction::S,
            },
            score: self.score + 1000,
        })
    }
    fn right(self: &State) -> Option<State> {
        Some(State {
            index: self.index,
            direction: match self.direction {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            },
            score: self.score + 1000,
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
    // println!("{}", str::from_utf8(&map)?);

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

        // dbg!(&state, &visit);
        if state.score >= *visit_score {
            continue;
        }

        *visit_score = state.score;
        queue.extend(state.next(&map, xlen));
        queue.extend(state.left());
        queue.extend(state.right());

        // println!("{}", str::from_utf8(&new_map)?);
        // let mut input: String = String::new();
        // stdin().read_line(&mut input).unwrap();
    }
    let mut queue: Vec<usize> = Vec::new();
    let mut shortest_paths: HashSet<usize> = HashSet::new();
    queue.push(
        map.iter()
            .position(|v| *v == E)
            .expect("missing end position in input data"),
    );

    let mut new_map: Vec<u8> = map.clone();
    while let Some(pos) = queue.pop() {
        let min = [
            visited.get(&(pos, Direction::N)),
            visited.get(&(pos, Direction::E)),
            visited.get(&(pos, Direction::S)),
            visited.get(&(pos, Direction::W)),
        ]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(&u64::MAX);
        dbg!(
            &queue,
            pos,
            visited.get(&(pos, Direction::N)),
            visited.get(&(pos, Direction::E)),
            visited.get(&(pos, Direction::S)),
            visited.get(&(pos, Direction::W)),
            min,
        );
        // let next_pos = match self.direction {
        //     Direction::N => self.index.checked_sub(xlen)?,
        //     Direction::E => self.index.checked_add(1)?,
        //     Direction::S => self.index.checked_add(xlen)?,
        //     Direction::W => self.index.checked_sub(1)?,
        // };

        if let Some(score) = visited.get(&(pos, Direction::N)) {
            if *score == *min {
                queue.push(pos.checked_add(xlen).unwrap());
            }
        }
        if let Some(score) = visited.get(&(pos, Direction::S)) {
            if *score == *min {
                queue.push(pos.checked_sub(xlen).unwrap());
            }
        }
        if let Some(score) = visited.get(&(pos, Direction::E)) {
            if *score == *min {
                queue.push(pos.checked_sub(1).unwrap());
            }
        }
        if let Some(score) = visited.get(&(pos, Direction::W)) {
            if *score == *min {
                queue.push(pos.checked_add(1).unwrap());
            }
        }
        new_map[pos] = b'o';
        println!("{}", str::from_utf8(&new_map)?);
    }
    println!("{}", str::from_utf8(&new_map)?);
    println!(
        "Solution: {} / Duration: {:.6?}",
        0,
        // unique_on_shortest_paths.len(),
        t0.elapsed()
    );
    Ok(())
}
