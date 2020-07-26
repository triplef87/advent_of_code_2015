use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut counter: usize = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut arr: Vec<(usize, usize, usize)> = Vec::new();
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                process_row(row, &mut map, &mut counter, &mut arr);
            }
        }
    }

    let len = map.len();
    let matrix = get_matrix(arr, len);
    let mut permutation: Vec<usize> = (0..len).collect();

    let val = travel(matrix, &mut permutation, len);

    println!("{}", val);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P :AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(input: String, map: &mut HashMap<String, usize>, counter: &mut usize, arr: &mut Vec<(usize, usize, usize)>) {
    let data_arr: Vec<&str> = input.split(' ').collect();
    if !map.contains_key(data_arr[0]) {
        map.insert(String::from(data_arr[0]), counter.clone());
        *counter = *counter + 1;
    }

    if !map.contains_key(data_arr[2]) {
        map.insert(String::from(data_arr[2]), counter.clone());
        *counter = *counter + 1;
    }

    let x = *map.get(data_arr[0]).unwrap_or(&0);
    let y = *map.get(data_arr[2]).unwrap_or(&0);

    let val: usize = data_arr[4].parse().unwrap();

    arr.push((x, y, val))
}

fn get_matrix(arr: Vec<(usize, usize, usize)>, len: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0 as usize; len]; len];

    for (x, y, val) in arr {
        matrix[x][y] = val;
        matrix[y][x] = val;
    }

    for idx in 0..len {
        matrix[idx][idx] = 0;
    }

    matrix
}

fn travel(matrix: Vec<Vec<usize>>, permutation: &mut Vec<usize>, len: usize) -> usize {
    let mut total = 0;
    
    for idx in 0..len - 1 {
        let begin = permutation[idx];
        let end = permutation[idx + 1];
        total = total + matrix[begin][end];
    }

    let mut check = true;

    while check {
        check = next_permutation(permutation, len);
        
        let mut min = 0;
        for idx in 0..len - 1 {
            let begin = permutation[idx];
            let end = permutation[idx + 1];
            min = min + matrix[begin][end];
        }

        println!("{}", min);
        
        // if min < total {
        //     total = min;
        // }
        
        // part 2
        if min > total {
            total = min;
        }
    
    }

    total
}

fn next_permutation(permutation: &mut Vec<usize>, len: usize) -> bool {
    for idx in (1..len).rev() {
        if permutation[idx] > permutation[idx - 1] {
            let mut tmp = len - 1;
            
            while permutation[idx - 1] >= permutation[tmp] {
                tmp = tmp - 1;
            }

            permutation.swap(idx - 1, tmp);

            let mut left = idx;
            let mut right = len -1;

            while left < right {
                permutation.swap(left, right);
                left = left + 1;
                right = right - 1;
            }

            return true;
        }
    }

    false
}