use regex::{Match, Regex};
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

    fn get_ports(prefix: char, map: &HashMap<String, Expression>) -> Vec<&String> {
        let mut s: Vec<&String> = map.keys().filter(|k| k.starts_with(prefix)).collect();
        s.sort();
        s.reverse();
        s
    }

    let xs = get_ports('x', &map);
    let ys = get_ports('y', &map);
    let zs = get_ports('z', &map);

    for x in &xs {
        println!("{x}: {:?}", map[*x]);
    }
    for x in &ys {
        println!("{x}: {:?}", map[*x]);
    }
    for x in &zs {
        println!("{x}: {:?}", map[*x]);
    }

    let (x, y) = (
        to_u64(&xs.into_iter().map(|k| map.get(k).unwrap().into()).collect()),
        to_u64(&ys.into_iter().map(|k| map.get(k).unwrap().into()).collect()),
    );
    let z = x + y;

    println!("X: {} {:#020b}", x, x);
    println!("Y: {} {:#020b}", y, y);
    println!("Z: {} {:#020b}", z, z);

    let solution = 0;

    println!("Solution: {} / Duration: {:.6?}", solution, t0.elapsed());

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

fn to_u64(values: &Vec<bool>) -> u64 {
    values.iter().fold(0, |acc, &v| (acc << 1) | u64::from(v))
}

// fn from_vec()
