use std::env;
use std::fs;

use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // dbg!(contents);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    let mut delta = Vec::new();
    let mut counter_list2 = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();
        dbg!(a, b);
        list1.push(a);
        list2.push(b);
        match counter_list2.get(&b) {
            Some(&number) => counter_list2.insert(b, number + 1),
            _ => counter_list2.insert(b, 1),
        };
    }
    // dbg!(list1.sort());
    // dbg!(list1);
    // dbg!(list2.sort());
    // dbg!(list2);
    let mut sum2 = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        delta.push((a - b).abs());

        match counter_list2.get(&a) {
            Some(&bcount) => sum2 += a * bcount,
            _ => sum2 += 0,
        };
    }
    // dbg!(delta);
    let sum: i32 = delta.iter().sum();
    dbg!(sum);
    dbg!(counter_list2);
    dbg!(sum2);
}
