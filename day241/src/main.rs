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

    // dbg!(&map);

    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());

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
