use core::panic;
use regex::Regex;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).ok_or("invalid arguments")?;
    let input = fs::read_to_string(&input_path)?;
    println!("Read {input_path}.");

    let expr = Regex::new(
        r"Register\s*A:\s*(-?\d+)\s*Register\s*B:\s*(-?\d+)\s*Register\s*C:\s*(-?\d+)\s*Program:\s*([\d,]+)",
    )?;

    let (mut a, mut b, mut c, program): (u32, u32, u32, Vec<u8>) = {
        let (_, [a, b, c, p]) = expr
            .captures(input.as_str())
            .ok_or("failed to parse input")?
            .extract();
        (
            a.parse()?,
            b.parse()?,
            c.parse()?,
            p.split(',').map(|s| s.parse().unwrap()).collect(),
        )
    };

    let mut output: Vec<u8> = Vec::new();
    let mut instruction_pointer: usize = 0;
    while instruction_pointer < program.len() {
        let (opcode, literal_operand) = (
            program
                .get(instruction_pointer)
                .ok_or("unable to get opcode")?,
            program
                .get(instruction_pointer + 1)
                .ok_or("unable to get operand")?,
        );
        instruction_pointer += 2;
        let combo_operand = match literal_operand {
            0..=3 => u32::from(*literal_operand),
            4 => a,
            5 => b,
            6 => c,
            _ => {
                panic!("invalid program operand: {:?}", literal_operand);
            }
        };
        match opcode {
            // adv
            0 => {
                a = a
                    .checked_div(2_u32.pow(combo_operand))
                    .ok_or("error during adv operation")?;
            }
            // bxl
            1 => {
                b ^= u32::from(*literal_operand);
            }
            // bst
            2 => {
                b = combo_operand % 8_u32;
            }
            // jnz
            3 if a != 0 => {
                instruction_pointer = *literal_operand as usize;
            }
            3 => {}
            // bxc
            4 => {
                b ^= c;
            }
            // out
            5 => {
                output.push((combo_operand % 8) as u8);
            }
            // bdv
            6 => {
                b = a
                    .checked_div(2_u32.pow(combo_operand))
                    .ok_or("error during adv operation")?;
            }
            // cdv
            7 => {
                c = a
                    .checked_div(2_u32.pow(combo_operand))
                    .ok_or("error during adv operation")?;
            }
            _ => {
                panic!("invalid program opcode: {:?}", opcode);
            }
        }
    }
    output
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(",");
    println!(
        "Solution: {} / Duration: {:.6?}",
        output
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(","),
        t0.elapsed()
    );
    Ok(())
}
