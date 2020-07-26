use std::{fs::File, io, path::Path, collections::HashSet};
use rand::{thread_rng, seq::SliceRandom};
use io::BufRead;

fn main() {
    let molecule = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr"; 
    let mut map: Vec<(String, String)> = Vec::new();
    let mut set:HashSet<String> = HashSet::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                process_row(row, &mut map);
            }
        }
    }

    // let molecule = "ee";

    // part 1
    // let mut tmp = molecule.to_string();
    // for (key, val) in &map {
    //     let len = key.len();
    //     let mut index = 0;

    //     while index_of(&tmp, key, &mut index) {
    //         let (prev, next) = tmp.split_at(index);

    //         let mut new = next.replacen(key, val, 1);
    //         new.insert_str(0, prev);

    //         if !set.contains(&new) {
    //             set.insert(new.clone());
    //         }

    //         index = index + len;
    //         tmp = molecule.to_string();
    //     }
    // }

    map.shuffle(&mut thread_rng());

    // part 2
    let mut goal = molecule.clone().to_string();
    let mut counter = 0;
    while goal != "e" {
        let mut check = true;
        for (val, key) in &map {
            let mut index = 0;

            while index_of(&goal, key, &mut index) {
                check = false;
                println!("{}", goal);
                let (prev, next) = goal.split_at(index);

                let mut new = next.replacen(key, val, 1);
                new.insert_str(0, prev);

                index = index + val.len();
                goal = new;
                counter = counter + 1;
            }
        }

        if check {
            counter = 0;
            goal = String::from(molecule);
            map.shuffle(&mut thread_rng());
        }
    }

    println!("{}", counter);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(row: String, map: &mut Vec<(String, String)>) {
    let slice: Vec<&str> = row.split(' ').collect();

    map.push((String::from(slice[0]), String::from(slice[2])));
}

fn index_of(input: &str, pat: &str, index: &mut usize) -> bool {
    let mut check = false;

    let len = pat.len();
    let mut buf = String::from("");

    let (_, sub) = input.split_at(*index);

    for (i, c) in sub.chars().enumerate() {
        buf.push(c);
        
        if buf.len() > len {
            buf.remove(0);
        }

        if buf == pat {
            *index = *index + i + 1 - len;
            check = true;
            break;
        }
    }

    check
}
