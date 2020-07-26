use std::{fs::File, io, path::Path, collections::HashSet};
use io::BufRead;

fn main() {
    let mut containers: Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                let val: usize = row.parse().unwrap();
                containers.push(val);
            } 
        }
    }

    containers.sort();
    containers.reverse();

    get_combination(&containers);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_combination(containers: &Vec<usize>) {
    let len = containers.len();
    let mut min_len = containers.len() + 1;
    let mut set: HashSet<Vec<usize>> = HashSet::new();
    let mut stack: Vec<usize> = Vec::new();
    stack.push(0);

    let mut dirty = vec![false; len];
    dirty[0] = true;

    let mut goal = 150 - containers[0];
    loop {
        let mut start = *stack.last().unwrap() + 1;

        while start < len {
            if containers[start] <= goal && !dirty[start] {
                goal = goal - containers[start];
                stack.push(start);
                dirty[start] = true;
                break;
            }
            start = start + 1;
        }

        if goal == 0 {
            // counter = counter + 1;     
            println!("****");
            println!("combination: {:?}", &stack);
            for idx in &stack {
                print!("{} ", containers[*idx]);
            }
            println!("\n****");
            
            set.insert(stack.clone());
            if min_len > stack.len() {
                min_len = stack.len();
            }

            start = len;
        }

        if start == len {
            goal = goal + containers[*stack.last().unwrap()];
            let pos = stack.pop().unwrap();
            for x in pos + 1..len {
                dirty[x] = false;
            }
        }

        if stack.len() == 0 {
            let mut pos = 0;
            while pos < len && dirty[pos] {
                pos = pos + 1;
            }

            if pos == len {
                break;
            }

            stack.push(pos);
            dirty[pos] = true;
            goal = goal - containers[pos];
        }
    }

    let mut ans = set.len(); 
    for row in &set {
        if row.len() > min_len {
            ans = ans - 1;
        }
    }
    println!("{}", ans);
}
