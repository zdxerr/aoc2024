use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents =
        fs::read_to_string(file_path).expect(&format!("Unable to open file: {file_path}"));
    let mut sum = 0;
    let mut enabled = true;
    println!("Loaded: {file_path}");
    let re = Regex::new(r"(mul)\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)|(do)\(\)|(don't)\(\)").unwrap();
    for cap in re.captures_iter(&contents) {
        // dbg!(&cap);
        println!("{}", &cap[0]);
        match cap
            .get(1)
            .unwrap_or_else(|| cap.get(4).unwrap_or_else(|| cap.get(5).unwrap()))
            .as_str()
        {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                if enabled {
                    sum += cap[2].parse::<i32>().unwrap_or(0) * cap[3].parse::<i32>().unwrap_or(0);
                }
            }
            _ => println!("Unexpected: {}", &cap[1]),
        };
    }
    dbg!(sum);
}
