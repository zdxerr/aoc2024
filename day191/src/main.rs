use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::rc::{Rc, Weak};
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let trie = Rc::new(RefCell::new(Trie::new()));
    let mut current_node = Rc::clone(&trie);

    let mut state = 0;

    for v in input {
        if state == 0 {
            match v {
                b',' => {
                    current_node.borrow_mut().towel = true;
                    current_node = Rc::clone(&trie);
                    continue;
                }
                b'\n' => {
                    state = 1;
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
            let next = Rc::new(RefCell::new(Trie::new()));
            current_node.borrow_mut().next[index].replace(Rc::clone(&next));
            current_node = Rc::clone(&next);
        } else {
            dbg!(v.to_string());
            match v {
                b'\n' if state == 2 => break,
                b'\n' if state == 1 => {
                    state = 2;
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
            print!("{v} {index}");

            if let Some(c) = &current_node.borrow().next[index] {
                println!("X {c:?}");
            } else {
                println!("FAILED, skip");
                break;
            }
        }
    }
    println!("{trie:?}");
    println!("Solution: {} / Duration: {:.6?}", 0, t0.elapsed());
    Ok(())
}

#[derive(Debug)]
struct Trie {
    towel: bool,
    next: [Option<Rc<RefCell<Trie>>>; 6],
}

impl Trie {
    fn new() -> Self {
        Trie {
            towel: false,
            next: [const { None }; 6],
        }
    }
}
