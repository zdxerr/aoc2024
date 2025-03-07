use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let mut solution = 0;
    let _: HashMap<(u8, u8), HashSet<(u8, u8)>> =
        input
            .trim_ascii()
            .split(|v| *v == b'\n')
            .fold(HashMap::new(), |mut map, line| {
                let a = (line[0], line[1]);
                let b = (line[3], line[4]);

                map.entry(a).or_default().insert(b);
                map.entry(b).or_default().insert(a);

                let seta = map.get(&a).unwrap();
                let setb = map.get(&b).unwrap();

                for c in seta.intersection(setb) {
                    if a.0 == b't' || b.0 == b't' || c.0 == b't' {
                        // println!("TRIPLET {a:?}-{b:?}-{c:?}");
                        solution += 1;
                    }
                }

                map
            });

    println!("Solution: {:#?} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}
