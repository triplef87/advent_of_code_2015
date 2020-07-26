extern crate lib;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use lib::*;

fn main() {
    let mut map: HashMap<String, Wire> = HashMap::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(row) = line {
                process_row(&row, &mut map);
            }
        }
    }
    
    // for (key, val) in &map {
    //     println!("{} has {:?}", key, val);
    // }

    println!("*******");
    process_signal(&mut map);
    println!("*******");

    for (key, val) in &map {
        println!("{} has {:?}", key, val);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(row: &str, map: &mut HashMap<String, Wire>) {
    let split: Vec<&str> = row.split(" ").collect();

    match split.len() {
        3 => {
            let source = split[0].parse::<u16>();
            match source {
                Ok(num) => {
                    insert_map(map, String::from(split[0]), String::from(split[2]), num, true)
                },
                Err(_) => {
                    insert_map(map, String::from(split[0]), String::from(split[2]), 0, false)
                }
            }
            
            if map.contains_key(split[2]) {
                if let Some(wire_src) = map.get_mut(split[2]) {
                    wire_src.src_a = String::from(split[0]);
                }
            } else {
                let mut wire_src = Wire::new();
                wire_src.src_a = String::from(split[0]);
                map.insert(String::from(split[2]), wire_src);
            }
        },
        4 => {
            insert_map(map, String::from(split[1]), String::from(split[3]), 0, false);
            if map.contains_key(split[3]) {
                if let Some(wire_src) = map.get_mut(split[3]) {
                    wire_src.command = Gate::Not;
                    wire_src.src_a = String::from(split[1]);
                }
            } else {
                let mut wire_src = Wire::new();
                wire_src.command = Gate::Not;
                wire_src.src_a = String::from(split[1]);
                map.insert(String::from(split[3]), wire_src);
            }
        },
        5 => {
            let source = split[0].parse::<u16>();
            match source {
                Ok(num) => {
                    insert_map(map, String::from(split[0]), String::from(split[4]), num, true)
                },
                Err(_) => {
                    insert_map(map, String::from(split[0]), String::from(split[4]), 0, false)
                }
            }

            let source = split[2].parse::<u16>();
            match source {
                Ok(num) => {
                    insert_map(map, String::from(split[2]), String::from(split[4]), num, true)
                },
                Err(_) => {
                    insert_map(map, String::from(split[2]), String::from(split[4]), 0, false)
                }
            }

            if map.contains_key(split[4]) {
                if let Some(wire_src) = map.get_mut(split[4]) {
                    match split[1] {
                        "AND" => wire_src.command = Gate::And,
                        "OR" => wire_src.command = Gate::Or,
                        "LSHIFT" => wire_src.command = Gate::Lshift,
                        "RSHIFT" => wire_src.command = Gate::Rshift,
                        _ => {}
                    }
                    wire_src.src_a = String::from(split[0]);
                    wire_src.src_b = String::from(split[2]);
                }
            } else {
                let mut wire_src = Wire::new();
                match split[1] {
                    "AND" => wire_src.command = Gate::And,
                    "OR" => wire_src.command = Gate::Or,
                    "LSHIFT" => wire_src.command = Gate::Lshift,
                    "RSHIFT" => wire_src.command = Gate::Rshift,
                    _ => {}
                }
                wire_src.src_a = String::from(split[0]);
                wire_src.src_b = String::from(split[2]);
                map.insert(String::from(split[4]), wire_src);
            }
        },
        _ => {}
    }
}

fn insert_map(map: &mut HashMap<String, Wire>, src: String, dest: String, val: u16, signal: bool) {
    if map.contains_key(&src) {
        if let Some(wire_src) = map.get_mut(&src) {
            wire_src.dest.push(dest);
        }
    } else {
        let mut wire_src = Wire::new();
        if signal {
            wire_src.state = WireType::Signal;
            wire_src.val = val;
        }
        wire_src.dest.push(dest);
        map.insert(src, wire_src);
    }
}

fn process_signal(map: &mut HashMap<String, Wire>) {
    let mut signal_stack = Vec::new();
    
    for (key ,val) in map.into_iter() {
        match val.state {
            WireType::Signal => {
                signal_stack.push(String::from(key));
            },
            _ => {}
        }
    }

    while !signal_stack.is_empty() {
        let cur_key = signal_stack.pop().unwrap();
        let mut src_wire = Wire::new();
        if let Some(cur_wire) = map.get_mut(&cur_key) {
            src_wire = cur_wire.get_wire();
        }

        println!("cur_key: {}, has {:?}", cur_key, src_wire);

        for key in src_wire.dest {
            if let Some(dest_wire) = map.get_mut(&key) {
                print!("{}, has ", key);
                match dest_wire.command {
                    Gate::And => {
                        if dest_wire.src_a == cur_key {
                            dest_wire.a_val = (src_wire.val, true)
                        }

                        if dest_wire.src_b == cur_key {
                            dest_wire.b_val = (src_wire.val, true)
                        }

                        if dest_wire.a_val.1 && dest_wire.b_val.1 {
                            let a = dest_wire.a_val.0;
                            let b = dest_wire.b_val.0;
                            dest_wire.state = WireType::Signal;
                            dest_wire.val = a & b;
                            signal_stack.push(key);
                        }
                    },
                    Gate::Or => {
                        if dest_wire.src_a == cur_key {
                            dest_wire.a_val = (src_wire.val, true)
                        }

                        if dest_wire.src_b == cur_key {
                            dest_wire.b_val = (src_wire.val, true)
                        }

                        if dest_wire.a_val.1 && dest_wire.b_val.1 {
                            let a = dest_wire.a_val.0;
                            let b = dest_wire.b_val.0;
                            dest_wire.state = WireType::Signal;
                            dest_wire.val = a | b;
                            signal_stack.push(key);
                        }                        
                    },                    
                    Gate::Lshift => {
                        if dest_wire.src_a == cur_key {
                            dest_wire.a_val = (src_wire.val, true)
                        }

                        if dest_wire.src_b == cur_key {
                            dest_wire.b_val = (src_wire.val, true)
                        }

                        if dest_wire.a_val.1 && dest_wire.b_val.1 {
                            let a = dest_wire.a_val.0;
                            let b = dest_wire.b_val.0;
                            dest_wire.state = WireType::Signal;
                            dest_wire.val = a << b;
                            signal_stack.push(key);
                        }   
                    },
                    Gate::Rshift => {
                        if dest_wire.src_a == cur_key {
                            dest_wire.a_val = (src_wire.val, true)
                        }

                        if dest_wire.src_b == cur_key {
                            dest_wire.b_val = (src_wire.val, true)
                        }

                        if dest_wire.a_val.1 && dest_wire.b_val.1 {
                            let a = dest_wire.a_val.0;
                            let b = dest_wire.b_val.0;
                            dest_wire.state = WireType::Signal;
                            dest_wire.val = a >> b;
                            signal_stack.push(key);
                        }   
                    },
                    Gate::Not => {
                        dest_wire.state = WireType::Signal;
                        dest_wire.val = !src_wire.val;
                        signal_stack.push(key);
                    },
                    Gate::Give => {
                        dest_wire.state = WireType::Signal;
                        dest_wire.val = src_wire.val;
                        signal_stack.push(key);
                    }
                }
                println!("{:?}", dest_wire);
            }
        }
    }
}