use core::panic;
use regex::Regex;
use std::collections::VecDeque;
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

    let program: Vec<u8> = {
        let (_, [_, _, _, p]) = expr
            .captures(input.as_str())
            .ok_or("failed to parse input")?
            .extract();
        p.split(',').map(|s| s.parse().unwrap()).collect()
    };

    let mut inputs: VecDeque<Vec<u8>> = VecDeque::new();
    inputs.push_front(vec![]);

    while let Some(mut input) = inputs.pop_front() {
        let index = input.len();
        let program_value = program
            .get(program.len().wrapping_sub(index.wrapping_add(1)))
            .unwrap();

        input.resize(index.wrapping_add(1), 0);

        for value in 0..=7 {
            input[index] = value;
            let a = input_to_u64(&input);
            let output = compute(a, 0, 0, &program)?;
            if output
                .get(output.len().wrapping_sub(index.wrapping_add(1)))
                .unwrap()
                == program_value
            {
                inputs.push_back(input.clone());
                if input.len() == program.len() {
                    println!("Solution: {} / Duration: {:.6?}", a, t0.elapsed());
                    return Ok(());
                }
            }
        }
    }
    eprintln!("No solution found / Duration: {:.6?}", t0.elapsed());
    Ok(())
}

fn input_to_u64(input: &[u8]) -> u64 {
    input
        .iter()
        .fold(0_u64, |acc, value| (acc << 3) | u64::from(*value))
}

fn compute(a: u64, b: u64, c: u64, program: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let (mut a, mut b, mut c) = (a, b, c);
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
            0..=3 => u64::from(*literal_operand),
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
                    .checked_div(2_u64.pow(combo_operand.try_into()?))
                    .ok_or("error during adv operation")?;
            }
            // bxl
            1 => {
                b ^= u64::from(*literal_operand);
            }
            // bst
            2 => {
                b = combo_operand % 8_u64;
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
                    .checked_div(2_u64.pow(combo_operand.try_into()?))
                    .ok_or("error during adv operation")?;
            }
            // cdv
            7 => {
                c = a
                    .checked_div(2_u64.pow(combo_operand.try_into()?))
                    .ok_or("error during adv operation")?;
            }
            _ => {
                panic!("invalid program opcode: {:?}", opcode);
            }
        }
    }
    Ok(output)
}
