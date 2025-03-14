use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(
        r"(?:([a-z0-9]+)\s*:\s*([01]))|(?:([a-z0-9]+)\s+((?:AND)|(?:OR)|(?:XOR))\s+([a-z0-9]+)\s*->\s*([a-z0-9]+))",
    )
    .unwrap();
    let map: BTreeMap<String, Expression> = expr
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

    let mut outputs: Vec<&str> = vec![];

    for (z, expr) in map.iter().filter(|(k, _)| k.starts_with('z')) {
        let number: u32 = z[1..].parse().unwrap();
        // println!("{number} {z}: {expr:?}");
        if let Expression::Gate { a, op, b } = expr {
            if *op != Operator::XOR {
                outputs.push(z);
                continue;
            }
            if number == 0 {
                if !([a, b] == ["x00", "y00"] || [b, a] == ["x00", "y00"]) {
                    outputs.push(z);
                }
                continue;
            }

            let mut x_xor_y = false;
            let inputs = [&format!("x{number:02}"), &format!("y{number:02}")];
            match (a, map.get(a).unwrap(), b, map.get(b).unwrap()) {
                (
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: Operator::XOR,
                        b: b1,
                    },
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: Operator::OR,
                        b: b2,
                    },
                )
                | (
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: Operator::OR,
                        b: b2,
                    },
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: Operator::XOR,
                        b: b1,
                    },
                ) if [a1, b1] == inputs || [b1, a1] == inputs => {
                    println!("{z} / {out2} = {a2}  {b2}");

                    // if number == 1 {
                    //     if !(*op2 == Operator::AND
                    //         && ([a2, b2] == ["x00", "y00"] || [b2, a2] == ["x00", "y00"]))
                    //     {
                    //         outputs.push(z);
                    //     }
                    //     continue;
                    // }
                }
                (
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: Operator::XOR,
                        b: b1,
                    },
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: Operator::OR,
                        b: b2,
                    },
                )
                | (
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: Operator::OR,
                        b: b2,
                    },
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: Operator::XOR,
                        b: b1,
                    },
                ) => outputs.push(out1),
                (
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: _,
                        b: b1,
                    },
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: Operator::OR,
                        b: b2,
                    },
                )
                | (
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: Operator::OR,
                        b: b2,
                    },
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: _,
                        b: b1,
                    },
                ) => outputs.push(out1),
                (
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: Operator::XOR,
                        b: b1,
                    },
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: _,
                        b: b2,
                    },
                )
                | (
                    out2,
                    Expression::Gate {
                        a: a2,
                        op: _,
                        b: b2,
                    },
                    out1,
                    Expression::Gate {
                        a: a1,
                        op: Operator::XOR,
                        b: b1,
                    },
                ) => outputs.push(out2),
                _ => panic!(),
            }
        }

        // if let Expression::Gate { a, op, b } = expr {
        //     let expra = map.get(a).unwrap();
        //     println!("  - {a}: {expra:?}");
        //     if let Expression::Gate { a, op, b } = expra {
        //         let expra = map.get(a).unwrap();
        //         println!("    - {a}: {expra:?}");
        //         let exprb = map.get(b).unwrap();
        //         println!("    - {b}: {exprb:?}");
        //     y}
        //     let exprb = map.get(b).unwrap();
        //     println!("  - {b}: {exprb:?}");
        //     if let Expression::Gate { a, op, b } = exprb {
        //         let expra = map.get(a).unwrap();
        //         println!("    - {a}: {expra:?}");
        //         let exprb = map.get(b).unwrap();
        //         println!("    - {b}: {exprb:?}");
        //     }
        // }
    }

    outputs.sort_unstable();

    println!(
        "Solution: {} / Duration: {:.6?}",
        outputs.join(","),
        t0.elapsed()
    );

    Ok(())
}

#[derive(Debug)]
enum Expression {
    Value(bool),
    Gate { a: String, op: Operator, b: String },
}

impl Into<bool> for &Expression {
    fn into(self) -> bool {
        match self {
            Expression::Value(value) => *value,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}
