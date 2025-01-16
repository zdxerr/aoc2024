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
                positions
                    .entry(*value)
                    .or_insert_with(|| Vec::<usize>::new())
                    .push(index);
                positions
            },
        );

    let mut anodes: HashMap<u8, HashSet<usize>> = HashMap::new();
    let ylen = map.len() / (xlen + 1);
    for (value, value_positions) in positions {
        for (index, pos_a) in value_positions.iter().enumerate() {
            for pos_b in &value_positions[index + 1..] {
                // println!("{value} {pos_a} {pos_b}");

                let (ax, ay) = coords(pos_a, &xlen);
                let (bx, by) = coords(pos_b, &xlen);

                let (nax, nbx) = if ax < bx {
                    let dx = bx - ax;
                    (
                        ax.checked_sub(dx).unwrap_or(usize::MAX),
                        bx.checked_add(dx).unwrap_or(usize::MAX),
                    )
                } else {
                    let dx = ax - bx;
                    (
                        ax.checked_add(dx).unwrap_or(usize::MAX),
                        bx.checked_sub(dx).unwrap_or(usize::MAX),
                    )
                };
                let (nay, nby) = if ay < by {
                    let dy = by - ay;
                    (
                        ay.checked_sub(dy).unwrap_or(usize::MAX),
                        by.checked_add(dy).unwrap_or(usize::MAX),
                    )
                } else {
                    let dy = ay - by;
                    (
                        ay.checked_add(dy).unwrap_or(usize::MAX),
                        by.checked_sub(dy).unwrap_or(usize::MAX),
                    )
                };
                // println!("{ax}|{ay}  /  {bx}|{by}  ~  {nax}|{nay}  >  {nbx}|{nby}");
                let positions = anodes.entry(value).or_default();
                if (nax < xlen) & (nay < ylen) {
                    let nposa = nay * (xlen + 1) + nax;
                    positions.insert(nposa);
                }
                if (nbx < xlen) & (nby < ylen) {
                    let nposb = nby * (xlen + 1) + nbx;
                    positions.insert(nposb);
                }
                // dbg!(&anodes);

                // println!("{}", str::from_utf8(&map).expect("ERROR"));
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
    println!(
        "Solution: {} / Duration: {:.6?}",
        unique_anodes.len(),
        t0.elapsed()
    );
}
