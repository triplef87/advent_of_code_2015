use std::{fs::File, io, path::Path};
use io::BufRead;
#[derive(Debug)]
struct Sue {
    num: usize,
    children: isize,
    cats: isize,
    samoyeds: isize,
    pomeranians: isize,
    akitas: isize,
    vizslas: isize,
    goldfish: isize,
    trees: isize,
    cars: isize,
    perfumes: isize
}

impl Sue {
    fn new(num: usize) -> Sue {
        Sue {
            num,
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        }
    }
}

fn main() {
    let mut sue_arr: Vec<Sue> = Vec::new(); 

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                proccess_row(&row, &mut sue_arr);
            }
        }
    }

    for sue in &sue_arr {
        if check_sue(sue) {
            println!("{}", sue.num);
        }
    }
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn proccess_row(row: &String, sue_arr: &mut Vec<Sue>) {
    let slice: Vec<&str> = row.split(|c| c == ' ' || c== ',' || c == ':').collect();

    let num: usize = slice[1].parse().unwrap();
    let mut sue = Sue::new(num);

    let num: isize = slice[5].parse().unwrap();
    sue_fill_ele(&mut sue, slice[3], num);

    let num: isize = slice[9].parse().unwrap();
    sue_fill_ele(&mut sue, slice[7], num);

    let num: isize = slice[13].parse().unwrap();
    sue_fill_ele(&mut sue, slice[11], num);

    sue_arr.push(sue);
}

fn sue_fill_ele(sue: &mut Sue, s: &str, num: isize) {
    match s {
        "children" => { sue.children = num; },
        "cats" => { sue.cats = num; },
        "samoyeds" => { sue.samoyeds = num; },
        "pomeranians" => { sue.pomeranians = num; },
        "akitas" => { sue.akitas = num; },
        "vizslas" => { sue.vizslas = num; },
        "goldfish" => { sue.goldfish = num; },
        "trees" => { sue.trees = num; },
        "cars" => { sue.cars = num; },
        "perfumes" => { sue.perfumes = num; },
        _ => {}
    }
}

fn check_sue(sue: &Sue) -> bool {
    if sue.children != 3 && sue.children != -1 { return false; }
    if sue.cats <= 7 && sue.cats != -1 { return false; }
    if sue.samoyeds != 2 && sue.samoyeds != -1 { return false; }
    if sue.pomeranians >= 3 && sue.pomeranians != -1 { return false; }
    if sue.akitas != 0 && sue.akitas != -1 { return false; }
    if sue.vizslas != 0 && sue.vizslas != -1 { return false; }
    if sue.goldfish >= 5 && sue.goldfish != -1 { return false; }
    if sue.trees <= 3 && sue.trees != -1 { return false; }
    if sue.cars != 2 && sue.cars != -1 { return false; }
    if sue.perfumes != 1 && sue.perfumes != -1 { return false; }
    
    true
}
