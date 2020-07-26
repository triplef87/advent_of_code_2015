use std::{fs::File, io, path::Path};
use std::{collections::HashMap, io::BufRead};

fn main() {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut stack: Vec<(usize, usize, isize)> = Vec::new();
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                process_row(&row[0..row.len() - 1], &mut map, &mut stack);
            }
        }
    }

    let total = map.len() + 1;
    let matrix = gen_matrix(&stack, total);
    let mut permu:Vec<usize> = (0..total).collect();
    let mut ans = 0;

    let mut conti = true;
    while conti {
        let happiness = compute(&matrix, &permu, total - 1);
        if ans < happiness {
            ans = happiness
        }

        for val in permu.clone() {
            print!("{} ", val);
        }
        println!("\nhappiness: {}", happiness);

        conti = next_permutaion(&mut permu);
    } 
    println!("{}", ans);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(row: &str, map: &mut HashMap<String, usize>, stack: &mut Vec<(usize, usize, isize)>) {
    let slice: Vec<&str> = row.split(" ").collect();

    if !map.contains_key(slice[0]) {
        let val = map.len();
        map.insert(String::from(slice[0]), val);
    }

    if !map.contains_key(slice[10]) {
        let val = map.len();
        map.insert(String::from(slice[10]), val);
    }

    let mut val: isize = slice[3].parse().unwrap();
    if slice[2] == "lose" {
        val = -val; 
    }

    let x = *map.get(slice[0]).unwrap();
    let y = *map.get(slice[10]).unwrap();

    stack.push((x, y, val))
}

fn gen_matrix(stack: &Vec<(usize, usize, isize)>, len: usize) -> Vec<Vec<isize>> {
    let mut matrix = vec![vec![0; len]; len];

    for val in stack {
        matrix[val.0][val.1] = val.2;
    }

    for x in 0..len {
        matrix[x][len - 1] = 0;
        matrix[len - 1][x] = 0;
    }

    matrix
}

fn next_permutaion(permu: &mut Vec<usize>) -> bool {
    let len = permu.len();
    
    for x in (2..len).rev() {
        if permu[x] > permu[x - 1] {
            let mut y = len - 1;
            while permu[x - 1] > permu[y] {
                y = y - 1;
            }
            permu.swap(x - 1, y);

            let mut left = x;
            let mut right = len - 1;

            while left < right {
                permu.swap(left, right);
                left = left + 1;
                right = right - 1;
            }
            return true;
        }
    }

    false
}

fn compute(matrix: &Vec<Vec<isize>>, permu: &Vec<usize>, len: usize) -> isize {
    let mut val = 0;

    for x in 0..len {
        let now = permu[x];
        let next = permu[x + 1];

        val = val + matrix[now][next];
        val = val + matrix[next][now];
    }

    val = val + matrix[permu[len]][permu[0]];
    val = val + matrix[permu[0]][permu[len]];

    val
}