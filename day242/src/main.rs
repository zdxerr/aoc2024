use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs, str};

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");

    let mut outputs: Vec<&str> = vec![];

    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let (_, gates) = input.split_once("\n\n").unwrap();

    let mut inports: HashMap<[u8; 3], &[u8]> = HashMap::new();
    let gates: HashMap<[u8; 3], ([u8; 3], &[u8], [u8; 3])> = gates
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            [
                parts.next().unwrap().as_bytes(),
                parts.next().unwrap().as_bytes(),
                parts.next().unwrap().as_bytes(),
                parts.next().unwrap().as_bytes(),
                parts.next().unwrap().as_bytes(),
            ]
        })
        .map(|[a, op, b, _, c]| {
            inports.insert(a.try_into().unwrap(), op);
            inports.insert(b.try_into().unwrap(), op);
            (
                c.try_into().unwrap(),
                (a.try_into().unwrap(), op, b.try_into().unwrap()),
            )
        })
        .collect();

    for (c, (a, op, b)) in &gates {
        match (a, gates.get(a), op, b, gates.get(b), c, inports.get(c)) {
            (
                [b'x', b'0', b'0'],
                _,
                [b'X', b'O', b'R'],
                [b'y', b'0', b'0'],
                _,
                [b'z', b'0', b'0'],
                _,
            )
            | (
                [b'y', b'0', b'0'],
                _,
                [b'X', b'O', b'R'],
                [b'x', b'0', b'0'],
                _,
                [b'z', b'0', b'0'],
                _,
            ) => {}
            (
                [b'x', b'0', b'0'],
                _,
                [b'A', b'N', b'D'],
                [b'y', b'0', b'0'],
                _,
                _,
                Some([b'X', b'O', b'R']),
            )
            | (
                [b'y', b'0', b'0'],
                _,
                [b'A', b'N', b'D'],
                [b'x', b'0', b'0'],
                _,
                _,
                Some([b'X', b'O', b'R']),
            ) => {}
            (
                [b'x', x1, x0],
                _,
                [b'X', b'O', b'R'],
                [b'y', y1, y0],
                _,
                _,
                Some([b'X', b'O', b'R']) | Some([b'A', b'N', b'D']),
            ) if x1 == y1 && x0 == y0 => {}
            (
                [b'y', y1, y0],
                _,
                [b'X', b'O', b'R'],
                [b'x', x1, x0],
                _,
                _,
                Some([b'X', b'O', b'R']) | Some([b'A', b'N', b'D']),
            ) if x1 == y1 && x0 == y0 => {}
            (_, _, [b'X', b'O', b'R'], _, _, [b'z', _, _], _) => {}
            ([b'x', _, _], _, [b'A', b'N', b'D'], [b'y', _, _], _, _, Some([b'O', b'R'])) => {}
            ([b'y', _, _], _, [b'A', b'N', b'D'], [b'x', _, _], _, _, Some([b'O', b'R'])) => {}
            (
                _,
                Some((_, [b'O', b'R'], _)),
                [b'A', b'N', b'D'],
                _,
                Some((_, [b'X', b'O', b'R'], _)),
                _,
                Some([b'O', b'R']),
            ) => {}
            ([b'y', _, _], _, [b'A', b'N', b'D'], [b'x', _, _], _, _, Some([b'X', b'O', b'R'])) => {
            }
            (_, _, [b'A', b'N', b'D'], _, _, _, Some([b'O', b'R'])) => {}
            (_, _, [b'O', b'R'], _, _, _, Some([b'X', b'O', b'R']) | Some([b'A', b'N', b'D'])) => {}
            (
                _,
                Some((_, [b'A', b'N', b'D'], _)),
                [b'O', b'R'],
                _,
                Some((_, [b'A', b'N', b'D'], _)),
                [b'z', b'4', b'5'],
                _,
            ) => {}
            _ => outputs.push(str::from_utf8(c).unwrap()),
        }
    }

    outputs.sort_unstable();

    for output in &outputs {
        let gate = gates.get(output.as_bytes()).unwrap();
        let inport =
            str::from_utf8(inports.get(output.as_bytes()).unwrap_or(&[b'-'].as_slice())).unwrap();

        let a = gates
            .get(&gate.0)
            .unwrap_or(&([b'-'; 3], &[b'-'], [b'-'; 3]));
        let b = gates
            .get(&gate.2)
            .unwrap_or(&([b'-'; 3], &[b'-'], [b'-'; 3]));
        println!(
            "{output} ({inport}) = {} ({}) {} {} ({})",
            str::from_utf8(&gate.0).unwrap(),
            str::from_utf8(&a.1).unwrap(),
            str::from_utf8(&gate.1).unwrap(),
            str::from_utf8(&gate.2).unwrap(),
            str::from_utf8(&b.1).unwrap(),
        )
    }

    println!(
        "Solution: ({}) {} / Duration: {:.6?}",
        outputs.len(),
        outputs.join(","),
        t0.elapsed()
    );

    Ok(())
}
