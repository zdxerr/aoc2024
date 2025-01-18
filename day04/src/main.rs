use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::vec;
fn contains_word(line: &str) -> usize {
    line.matches("XMAS").count() + line.matches("SAMX").count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents =
        fs::read_to_string(file_path).expect(&format!("Unable to open file: {file_path}"));
    let mut sum = 0;
    println!("Loaded: {file_path}");
    let mut columns = Vec::new();
    for line in contents.lines() {
        sum += contains_word(line);
        columns.resize_with(line.len(), || "".to_owned());
        for (index, char) in line.chars().enumerate() {
            let col = format!("{}{}", &columns.get(index).unwrap(), &char);
            columns[index] = col.to_owned();
        }
    }
    for column in columns {
        sum += contains_word(&column);
    }
    let mut diagonals: BTreeMap<i32, String> = BTreeMap::new();
    let mut diagonals1: BTreeMap<i32, String> = BTreeMap::new();
    // data.insert("purpose", 42);
    // data.insert("nice", 69);
    for (line_number, line) in contents.lines().enumerate() {
        for (column_number, char) in line.chars().enumerate() {
            let line_offset = column_number as i32 - line_number as i32;
            diagonals
                .entry(line_offset)
                .and_modify(|diag| diag.push(char))
                .or_insert(char.to_string());
            diagonals1
                .entry(column_number as i32 + line_number as i32)
                .and_modify(|diag| diag.push(char))
                .or_insert(char.to_string());
        }
    }
    for (key, diag) in &diagonals {
        println!("# {key} {diag}");
        sum += contains_word(diag);
    }
    for (key, diag) in &diagonals1 {
        println!("~ {key} {diag}");
        sum += contains_word(diag);
    }
    dbg!(sum);
    let mut matrix: Vec<Vec<char>> = vec![];
    for (row_number, row) in contents.lines().enumerate() {
        matrix.insert(row_number, vec![]);
        for (column_number, char) in row.chars().enumerate() {
            matrix[row_number].insert(column_number, char);
        }
    }
    sum = 0;
    for (row_number, row) in matrix.iter().enumerate() {
        for (column_number, _char) in row.iter().enumerate() {
            if (0 < row_number
                && row_number < matrix.len() - 1
                && 0 < column_number
                && column_number < row.len() - 1)
                && matrix[row_number][column_number] == 'A'
                && ((matrix[row_number - 1][column_number - 1] == 'M' // 1
                    && matrix[row_number + 1][column_number + 1] == 'S'
                    && matrix[row_number - 1][column_number + 1] == 'M'
                    && matrix[row_number + 1][column_number - 1] == 'S')
                    || (matrix[row_number - 1][column_number - 1] == 'S' // 2
                    && matrix[row_number + 1][column_number + 1] == 'M'
                    && matrix[row_number - 1][column_number + 1] == 'S'
                    && matrix[row_number + 1][column_number - 1] == 'M')
                    || (matrix[row_number - 1][column_number - 1] == 'M' // 3
                    && matrix[row_number + 1][column_number + 1] == 'S'
                    && matrix[row_number - 1][column_number + 1] == 'S'
                    && matrix[row_number + 1][column_number - 1] == 'M')
                    || (matrix[row_number - 1][column_number - 1] == 'S' // 4
                    && matrix[row_number + 1][column_number + 1] == 'M'
                    && matrix[row_number - 1][column_number + 1] == 'M'
                    && matrix[row_number + 1][column_number - 1] == 'S'))
            {
                println!("{row_number} {column_number}");
                sum += 1;
            }
        }
    }
    dbg!(sum);
}
