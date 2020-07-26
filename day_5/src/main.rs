use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(row) = line {             
                if check_repeat_str(&row) && check_repeat_chat(&row) {
                    count = count + 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Problem 1
// fn check_vowels(input: &str) -> bool {
//     let mut count: u8 = 0;
//     for element in input.chars() {
//         match element {
//             'a' => count = count + 1,
//             'e' => count = count + 1,
//             'i' => count = count + 1,
//             'o' => count = count + 1,
//             'u' => count = count + 1,
//             _ => {}
//         }

//         if count == 3 {
//             return true;
//         }
//     }

//     false
// }

// fn check_double(input: &str) -> bool {
//     let mut indices = input.char_indices();
//     let mut index: usize = 0;
//     let (_, mut origin) = indices.next().unwrap();
//     let len = input.len() - 2;
    
//     while index <= len {
//         let (pos, next) =  indices.next().unwrap();
//         if origin == next {
//             return true;
//         }
//         origin = next;
//         index = pos;
//     }
    
//     false
// }

// fn check_invalid(input: &str) -> bool {
//     if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy") {
//         return false;
//     }

//     true
// }

// Problem 2
fn check_repeat_str(input: &str) -> bool {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut index: usize = 0;
    let len = input.len() - 1;

    while index < len {
        let slice = input.get(index..(index + 2)).unwrap_or("  ");
        if map.contains_key(slice) {
            return true;
        }
        map.insert(slice.to_string(), index);
        index  = index + 2;
    }

    index = 1;
    while index  < len {
        let slice = input.get(index..(index + 2)).unwrap_or("  ");
        if map.contains_key(slice) {
            let exist_index = map.get(slice).unwrap_or(&0);
            let index_diff = *exist_index as isize - index as isize;
            if !(index_diff == 1 || index_diff == -1) {
                return true;
            }
        }
        map.insert(slice.to_string(), index);
        index  = index + 2;
    }

    false
}

fn check_repeat_chat(input: &str) -> bool {
    let len = input.len() - 1;
    let mut str_iter = input.char_indices();

    let (_, mut left) = str_iter.next().unwrap_or((0, ' '));
    let (mut index, mut right) = str_iter.next().unwrap_or((0, ' '));

    while index < len {
        let (_, incoming) = str_iter.next().unwrap_or((0, ' '));
        if left == incoming {
            return true;
        }

        left = right;
        right = incoming;
        index = index + 1;
    }
    
    false
}