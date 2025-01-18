use std::collections::HashSet;
use std::env;
use std::fs;

fn valid(rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> (u32, u32) {
    print!("{update:?}");
    let mut valid = true;

    let mut update = update.clone();
    for index in 0..update.len() {
        for sindex in index + 1..update.len() {
            if rules.contains(&(update[sindex], update[index])) {
                let page = update.remove(sindex);
                update.insert(index, page);

                print!(" move {page} at {sindex} to {index}");

                valid = false;
            }
        }
    }
    // let mut repaired_update = update.clone();
    // for (index, current_page) in update.iter().enumerate() {
    //     for (succeding_index, succeding_page) in (&update[index + 1..]).iter().enumerate() {
    //         if rules.contains(&(succeding_page.clone(), current_page.clone())) {
    //             let real_succeding_index = index + 1 + succeding_index;
    //             // repaired_update.swap(index, real_succeding_index);
    //             repaired_update.remove(real_succeding_index);
    //             repaired_update.insert(index, succeding_page.clone());
    //             print!(" INVALID swap {index}>{real_succeding_index} {repaired_update:?}");
    //             valid = false;
    //             break;
    //         }
    //     }
    // }
    if valid {
        let value = update[update.len() / 2];
        println!(" VALID -> {value}");
        (value, 0)
    } else {
        let value = update[update.len() / 2];
        println!(" {update:?} -> {value}");
        return (0, value);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents =
        fs::read_to_string(file_path).expect(&format!("Unable to open file: {file_path}"));
    println!("Loaded: {file_path}");
    let mut rules: HashSet<(u32, u32)> = HashSet::new();
    let mut result: (u32, u32) = (0, 0);
    for line in contents.lines() {
        if let Some((a, b)) = line.split_once('|') {
            rules.insert((a.parse().unwrap_or(0), b.parse().unwrap_or(0)));
        } else if line != "" {
            let update: Vec<u32> = line
                .split(',')
                .map(|page_str| page_str.parse().unwrap_or(0))
                .collect();

            let uresult = valid(&rules, &update);
            result.0 += uresult.0;
            result.1 += uresult.1;
        } else {
            if line != "" {
                eprintln!("ERROR {line}");
            }
        }
    }
    println!("Result: {result:?}");
}
