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

    let mut state = State::ParseTree;

    let mut counter = 0;

    for v in input {
        let index = match (v, &state) {
            (b',', State::ParseTree) => {
                print!(", ",);
                current_node.borrow_mut().towel = true;
                current_node = Rc::clone(&root);
                continue;
            }
            (b'\n', State::ParseTree) => {
                println!("#");
                current_node.borrow_mut().towel = true;
                current_node = Rc::clone(&root);
                state = State::ParsePattern;
                continue;
            }
            (b'\n', State::InvalidPattern) => {
                current_node = Rc::clone(&root);
                state = State::ParsePattern;
                continue;
            }
            (b'\n', State::ParsePattern) => {
                println!("_ {} {}", counter, current_node.borrow().towel);
                if current_node.borrow().towel {
                    counter += 1;
                }
                current_node = Rc::clone(&root);
                // break;
                continue;
            }
            (b' ', _) | (_, State::InvalidPattern) => continue,
            (b'w', _) => 0,
            (b'u', _) => 1,
            (b'b', _) => 2,
            (b'r', _) => 3,
            (b'g', _) => 4,
            _ => panic!("unexpected value: {}", str::from_utf8(&[v]).unwrap()),
        };
        print!("{} ", str::from_utf8(&[v]).unwrap());
        if state == State::ParseTree {
            current_node = {
                let mut b = current_node.borrow_mut();
                let next_node;
                if let Some(next) = b.next[index].take() {
                    print!("REUSE ");
                    next_node = Rc::clone(&next);
                    b.next[index] = Some(next);
                } else {
                    print!("NEW ");
                    next_node = Rc::new(RefCell::new(Trie::new()));
                    b.next[index] = Some(Rc::clone(&next_node));
                }
                next_node
            };
        } else {
            print!("{} {} ", str::from_utf8(&[v]).unwrap(), index);

            current_node = {
                let b = current_node.borrow();

                if let Some(next) = &b.next[index] {
                    println!("MATCH {} {}", b.towel, next.borrow().towel);
                    Rc::clone(next)
                } else {
                    println!("NO MATCH {}", b.towel);
                    if b.towel {
                        let b = root.borrow();
                        if let Some(next) = &b.next[index] {
                            println!("MATCH {} {}", b.towel, next.borrow().towel);
                            Rc::clone(next)
                        } else {
                            println!("INVALID PATTERN");
                            state = State::InvalidPattern;
                            Rc::clone(&root)
                        }
                    } else {
                        println!("INVALID PATTERN");
                        state = State::InvalidPattern;
                        Rc::clone(&root)
                    }
                }
            };
        }
    }
    // r, wr, b, g, bwu, rb, gb, br
    // brwrr

    // println!("{root:#?}");
    println!("Solution: {} / Duration: {:.6?}", counter, t0.elapsed());
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

    // fn get_next(&mut self, )
}
