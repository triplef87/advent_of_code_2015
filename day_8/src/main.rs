use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(row) = line {
                print!("{}, ", row);
                let code_len = row.len();
                let value_len = process_row_2(row);
                println!("code: {}, value: {}", code_len, value_len);
                total = total + value_len - code_len;
            }
        }
    }
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(input: String) -> usize {
    let mut counter = 0;
    let mut state: usize = 0;
    let size = input.len() - 1;

    for (index, ch) in input.chars().enumerate() {
        if index == 0 { continue; }
        if index == size { break; }
        match ch {
            '\\' => {
                match state {
                    0 => {
                        state = 1;
                    },
                    1 => {
                        state = 0;
                        counter = counter + 1;
                    },
                    _ => {

                    }
                }
            },
            '"' => {
                match state {
                    0 => {
                        counter = counter + 1;
                    },
                    1 => {
                        state = 0;
                        counter = counter + 1;
                    },
                    _ => {

                    }
                }
            },
            'x' => {
                match state {
                    0 => {
                        counter = counter + 1;
                    },
                    1 => {
                        state = 2;
                    },
                    _ => {}
                }
            },
            _ => {
                match state {
                    0 => {
                        counter = counter + 1;
                    },
                    2 => {
                        state = state + 1
                    },
                    3 => {
                        state = 0;
                        counter = counter + 1;
                    },
                    _ => {}
                }
            }
        }
    }
    counter + state
}

fn process_row_2(input: String) -> usize {
    let mut s = String::from("\"");

    for c in input.chars() {
        match c {
            '\\' => {
                s.push_str("\\\\");
            },
            '\"' => {
                s.push_str("\\\"");
            },
            _ => {
                s.push(c);
            }
        } 
    }
    s.push('"');
    s.len()
}