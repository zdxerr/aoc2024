use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::Instant;
use std::{env, fs, str};

#[derive(Eq, PartialEq)]
enum State {
    ParseTree,
    ParsePattern,
    InvalidPattern,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let root = Rc::new(RefCell::new(Trie::new()));
    let mut current_node = Rc::clone(&root);

    let mut current_nodes0: Vec<(u64, Rc<RefCell<Trie>>)> = vec![(1, Rc::clone(&root))];
    let mut current_nodes1: Vec<(u64, Rc<RefCell<Trie>>)> = vec![];

    let mut state = State::ParseTree;

    let mut solution1 = 0;
    let mut solution2 = 0;

    for v in input {
        let index = match (v, &state) {
            (b',', State::ParseTree) => {
                current_node.borrow_mut().towel = true;
                current_node = Rc::clone(&root);
                continue;
            }
            (b'\n', State::ParseTree) => {
                current_node.borrow_mut().towel = true;
                current_node = Rc::clone(&root);
                state = State::ParsePattern;
                continue;
            }
            (b'\n', State::InvalidPattern) => {
                current_node = Rc::clone(&root);
                current_nodes0.clear();
                current_nodes1.clear();
                current_nodes0.push((1, Rc::clone(&root)));
                state = State::ParsePattern;
                continue;
            }
            (_, State::InvalidPattern) => continue,
            (b'\n', State::ParsePattern) => {
                let ways = current_nodes0
                    .iter()
                    .filter(|(_, v)| v.borrow().towel)
                    .fold(0, |acc, (c, _)| acc + c);
                if ways > 0 {
                    solution1 += 1;
                }
                solution2 += ways;
                current_node = Rc::clone(&root);
                current_nodes0.clear();
                current_nodes1.clear();
                current_nodes0.push((1, Rc::clone(&root)));
                continue;
            }
            (b' ', _) => continue,
            (b'w', _) => 0,
            (b'u', _) => 1,
            (b'b', _) => 2,
            (b'r', _) => 3,
            (b'g', _) => 4,
            _ => panic!("unexpected value: {}", str::from_utf8(&[v]).unwrap()),
        };
        // print!("{} ", str::from_utf8(&[v]).unwrap());

        if state == State::ParseTree {
            current_node = {
                let mut b = current_node.borrow_mut();
                let next_node;
                if let Some(next) = b.next[index].take() {
                    next_node = Rc::clone(&next);
                    b.next[index] = Some(next);
                } else {
                    next_node = Rc::new(RefCell::new(Trie::new()));
                    b.next[index] = Some(Rc::clone(&next_node));
                }
                next_node
            };
        } else {
            (current_nodes0, current_nodes1) = (current_nodes1, current_nodes0);
            let mut towels = 0;
            while let Some(current_node) = current_nodes1.pop() {
                let b = current_node.1.borrow();

                if let Some(next) = &b.next[index] {
                    current_nodes0.push((current_node.0, Rc::clone(next)));
                }
                if b.towel {
                    towels += current_node.0;
                }
            }

            if towels > 0 {
                let b = root.borrow();
                if let Some(next) = &b.next[index] {
                    current_nodes0.push((towels, Rc::clone(next)));
                }
            }

            if current_nodes0.is_empty() {
                state = State::InvalidPattern;
            }
        }
    }
    println!(
        "Solution: {} {} / Duration: {:.6?}",
        solution1,
        solution2,
        t0.elapsed()
    );
    Ok(())
}

#[derive(Debug)]
struct Trie {
    towel: bool,
    next: [Option<Rc<RefCell<Trie>>>; 5],
}

impl Trie {
    fn new() -> Self {
        Trie {
            towel: false,
            next: [const { None }; 5],
        }
    }
}
