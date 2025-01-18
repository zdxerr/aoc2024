use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{env, fs, str};
const NL: u8 = b'\n';
const FREE: u8 = b'.';
const IGNORED_SYMBOLS: &'static [u8] = &[NL, FREE];

fn coords(index: &usize, width: &usize) -> (usize, usize) {
    (index % (width + 1), index / (width + 1))
}

fn main() {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");

    let map = fs::read(&input_path).unwrap_or_else(|_| panic!("unable to read: {input_path:?}"));
    println!("Read {input_path}.");

    // parse map for xlen and positions of all antenas
    let mut xlen: usize = usize::MAX;

    let positions = map
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if xlen == usize::MAX && value == &NL {
                xlen = index;
            }
            (index, value)
        })
        .filter(|(_, value)| !IGNORED_SYMBOLS.contains(value))
        .fold(
            HashMap::<u8, Vec<usize>>::new(),
            |mut positions, (index, value)| {
                positions.entry(*value).or_default().push(index);
                positions
            },
        );

    let mut anodes: HashMap<u8, HashSet<usize>> = HashMap::new();
    let ylen = map.len() / (xlen + 1);
    for (value, value_positions) in positions {
        let positions = anodes.entry(value).or_default();
        let mut insert = |x: usize, y: usize| {
            if (x < xlen) & (y < ylen) {
                positions.insert(y * (xlen + 1) + x);
                positions.insert(y * (xlen + 1) + x);
            }
        };
        for (index, pos_a) in value_positions.iter().enumerate() {
            for pos_b in &value_positions[index + 1..] {
                let (ax, ay) = coords(pos_a, &xlen);
                let (bx, by) = coords(pos_b, &xlen);

                let dx = bx.abs_diff(ax);
                let dy = by.abs_diff(ay);
                let signx = bx >= ax; // avoid overflows by saving the sing

                let (mut nx, mut ny): (usize, usize) = (ax, ay);
                while (nx < xlen) & (ny < ylen) {
                    insert(nx, ny);
                    if signx {
                        nx = nx.checked_sub(dx).unwrap_or(usize::MAX);
                        ny = ny.checked_sub(dy).unwrap_or(usize::MAX);
                    } else {
                        nx = nx.checked_add(dx).unwrap_or(usize::MAX);
                        ny = ny.checked_sub(dy).unwrap_or(usize::MAX);
                    }
                }

                let (mut nx, mut ny): (usize, usize) = (bx, by);
                while (nx < xlen) & (ny < ylen) {
                    insert(nx, ny);
                    if signx {
                        nx = nx.checked_add(dx).unwrap_or(usize::MAX);
                        ny = ny.checked_add(dy).unwrap_or(usize::MAX);
                    } else {
                        nx = nx.checked_sub(dx).unwrap_or(usize::MAX);
                        ny = ny.checked_add(dy).unwrap_or(usize::MAX);
                    }
                }
            }
        }
    }
    let unique_anodes: HashSet<&usize> =
        anodes
            .values()
            .fold(HashSet::<&usize>::new(), |mut acc, set| {
                acc.extend(set);
                acc
            });
    let rendered_map: Vec<u8> = map
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            if unique_anodes.contains(&index) && value == FREE {
                b'#'
            } else {
                value
            }
        })
        .collect();
    println!("{}", str::from_utf8(&rendered_map).expect("ERROR"));
    println!(
        "Solution: {} / Duration: {:.6?}",
        unique_anodes.len(),
        t0.elapsed()
    );
}
