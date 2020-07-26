use std::{fs::File, io, path::Path};
use io::BufRead;

fn main() {
    let mut instructions: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                instructions.push(row);
            }
        }
    }

    let mut index = 0;
    let mut values = (1, 0);
    
    while index < instructions.len() {
        println!("{:?}", values);
        let instruction: Vec<&str> = instructions.get(index).unwrap().split(" ").collect();

        match instruction[0] {
            "hlf" => {
                values.0 = values.0 / 2;
                index = index + 1;
            },
            "tpl" => {
                values.0 = values.0 * 3;
                index = index + 1;
            },
            "inc" => {
                if instruction[1] == "a" {
                    values.0 = values.0 + 1;
                } else {
                    values.1 = values.1 + 1;
                }
                index = index + 1;
            },
            "jmp" => {
                let offset: i32 = instruction[1].parse().unwrap();
                if offset.is_negative() {
                    index = index - offset.wrapping_abs() as usize;
                } else {
                    index = index + offset as usize;
                }
            },
            "jie" => {
                if values.0 % 2 == 0 {
                    let offset: i32 = instruction[2].parse().unwrap();
                    if offset.is_negative() {
                        index = index - offset.wrapping_abs() as usize;
                    } else {
                        index = index + offset as usize;
                    }
                } else {
                    index = index + 1;
                }
            },
            "jio" => {
                if values.0 == 1 {
                    let offset: i32 = instruction[2].parse().unwrap();
                    if offset.is_negative() {
                        index = index - offset.wrapping_abs() as usize;
                    } else {
                        index = index + offset as usize;
                    }
                } else {
                    index = index + 1;
                }
            },
            _ => {

            }
        }
    }

    println!("{:?}", values);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}
