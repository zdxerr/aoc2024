use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let mut trie = Trie::new();
    let mut current_node = &mut trie;

    let mut state = 0;

    for v in input {
        if state == 0 {
            match v {
                b'\n' => {
                    state = 1;
                    continue;
                }
                b',' => {
                    current_node = &mut trie;
                    continue;
                }
                b' ' => continue,
                _ => {}
            }

            let index = match v {
                b'w' => 0,
                b'u' => 1,
                b'b' => 2,
                b'r' => 3,
                b'g' => 4,
                _ => panic!("unknown color {v:?}"),
            };
            let mut next = Trie::new();
            current_node.next[index] = Some(Box::new(next));
            current_node = &mut next;

            // trie = current_node;

            // println!("");
        } else {
            print!("{v}")
        }
    }
    println!();
    println!("{trie:?}");
    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}

#[derive(Debug)]
struct Trie {
    next: [Option<Box<Trie>>; 6],
}

impl Trie {
    fn new() -> Self {
        Trie {
            next: [const { None }; 6],
        }
    }
}
