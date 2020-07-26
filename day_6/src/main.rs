use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut light_array = [[Light {bright: 0}; 1000]; 1000];
    let mut total: i32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let ins = parse_row(&row);
                total = total + iter_array( &mut light_array, ins.ins_type, ins.begin, ins.end);
                println!("Instruction is {}, from {:?} to {:?}", ins.ins_type, ins.begin, ins.end);
            }
        }
    }
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_row(row: &str) -> Instruct {
    let split: Vec<&str> = row.split(" ").collect();
    let mut index = 0;
    let mut ins = Instruct {
        ins_type: 0,
        begin: (0, 0),
        end: (0, 0),
    };
    
    if split[0] == "toggle" {
        ins.ins_type = 2;
    } else {
        if split[1] == "off" {
            ins.ins_type = 1;
        }
        index = 1;
    }

    let split_num: Vec<&str> = split[index + 1].split(",").collect();
    ins.begin = (split_num[0].parse().unwrap(), split_num[1].parse().unwrap());

    let split_num: Vec<&str> = split[index + 3].split(",").collect();
    ins.end = (split_num[0].parse().unwrap(), split_num[1].parse().unwrap());

    ins
}

struct Instruct {
    ins_type: u8,
    begin: (usize, usize),
    end: (usize, usize)
}

fn iter_array(arr: &mut [[Light; 1000]; 1000], ins: u8, begin: (usize, usize), end: (usize, usize)) -> i32 {
    let mut x = begin.0;
    let mut y = begin.1;
    let mut count = 0;

    while x <= end.0 {
        while y <= end.1 {
            match ins {
                0 => {
                    count = count + arr[x][y].turn_on();
                },
                1 => {
                    count = count + arr[x][y].turn_off();
                },
                2 => {
                    count = count + arr[x][y].toggle();
                },
                _ => {}
            }
            y = y + 1;
        }
        y = begin.1;
        x = x + 1;
    }

    count
}

// Problem 1
// #[derive(Copy, Clone)]
// struct Light {
//     on: bool
// }

// impl Light {
//     fn turn_off(&mut self) -> i32 {
//         if self.on {
//             self.on = false;
//             return -1;
//         }

//         return 0;
//     }

//     fn turn_on(&mut self) -> i32 {
//         if !self.on {
//             self.on = true;
//             return 1;
//         }

//         return 0;
//     }

//     fn toggle(&mut self) -> i32 {
//         let mut ret:i32 = 0;
//         if self.on {
//             ret = ret - 1;
//         } else {
//             ret = ret + 1;
//         }
//         self.on = !self.on;

//         ret
//     }
// }

// Problem 2
#[derive(Copy, Clone)]
struct Light {
    bright: usize
}

impl Light {
    fn turn_off(&mut self) -> i32 {
        if self.bright  == 0 {
            return 0;
        }
        self.bright = self.bright - 1;

        -1
    }

    fn turn_on(&mut self) -> i32 {
        self.bright = self.bright + 1;

        1
    }

    fn toggle(&mut self) -> i32 {
        self.bright = self.bright + 2;

        2
    }
}
