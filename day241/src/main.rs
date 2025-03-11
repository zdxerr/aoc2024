use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(
        r"(?:([a-z0-9]+)\s*:\s*([01]))|(?:([a-z0-9]+)\s+((?:AND)|(?:OR)|(?:XOR))\s+([a-z0-9]+)\s*->\s*([a-z0-9]+))",
    )
    .unwrap();
    let map: HashMap<String, Expression> = expr
        .captures_iter(input.as_str())
        .map(|cap| {
            if let (Some(port), Some(value)) = (cap.get(1), cap.get(2)) {
                (
                    port.as_str().to_string(),
                    Expression::Value(value.as_str() == "1"),
                )
            } else if let (Some(in0), Some(op), Some(in1), Some(out)) =
                (cap.get(3), cap.get(4), cap.get(5), cap.get(6))
            {
                if out.as_str().starts_with('z') {
                    println!(
                        "{:?}",
                        (
                            out.as_str().to_string(),
                            Expression::Gate {
                                a: in0.as_str().to_string(),
                                op: match op.as_str() {
                                    "AND" => Operator::AND,
                                    "OR" => Operator::OR,
                                    "XOR" => Operator::XOR,
                                    _ => panic!("unknown operator"),
                                },
                                b: in1.as_str().to_string(),
                            },
                        )
                    );
                }
                (
                    out.as_str().to_string(),
                    Expression::Gate {
                        a: in0.as_str().to_string(),
                        op: match op.as_str() {
                            "AND" => Operator::AND,
                            "OR" => Operator::OR,
                            "XOR" => Operator::XOR,
                            _ => panic!("unknown operator"),
                        },
                        b: in1.as_str().to_string(),
                    },
                )
            } else {
                panic!();
            }
        })
        .collect();
    let mut zs: Vec<&String> = map.keys().filter(|k| k.starts_with('z')).collect();
    zs.sort();
    zs.reverse();

    let solution = zs
        .into_iter()
        .map(|z| {
            println!("{}: {}", z, u64::from(solve(z, &map)));
            solve(z, &map)
        })
        .fold(0_u64, |acc, v| (acc << 1) | u64::from(v));
    // for x in map.keys().filter(|k| k.starts_with('z')) {
    //     println!("{x}: {}", solve(x, &map));
    // }

    // dbg!(&map);

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());

    Ok(())
}

#[derive(Debug)]
enum Expression {
    Value(bool),
    Gate { a: String, op: Operator, b: String },
}

#[derive(Debug)]
enum Operator {
    AND,
    OR,
    XOR,
}

fn solve(port: &String, map: &HashMap<String, Expression>) -> bool {
    match map.get(port).unwrap() {
        Expression::Gate { a, op, b } => {
            let a = solve(a, map);
            let b = solve(b, map);
            match op {
                Operator::AND => a & b,
                Operator::OR => a | b,
                Operator::XOR => a ^ b,
            }
        }
        Expression::Value(value) => *value,
    }
}
