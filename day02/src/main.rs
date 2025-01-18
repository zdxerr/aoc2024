use std::env;
use std::fs;

fn valid_deltas(levels: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = levels.windows(2).map(|level| level[1] - level[0]).collect();
    deltas.iter().all(|delta| (0 < *delta && *delta < 4))
        || deltas.iter().all(|delta| (-4 < *delta && *delta < 0))
}

fn valid(levels: &Vec<i32>) -> bool {
    if valid_deltas(&levels) {
        println!("{levels:?} VALID");
        return true;
    }
    for (index, _level) in levels.iter().enumerate() {
        let mut levelsc: Vec<_> = levels.iter().cloned().collect();
        levelsc.remove(index);
        if valid_deltas(&levelsc) {
            println!("{levels:?} {levelsc:?} VALID");
            return true;
        }
    }
    println!("{levels:?} INVALID");
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents =
        fs::read_to_string(file_path).expect(&format!("Unable to open file: {file_path}"));

    println!("Loaded: {file_path}");

    let mut counter = 0;
    for line in contents.lines() {
        if valid(
            &line
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        ) {
            counter += 1;
        }
    }

    dbg!(counter);
}
