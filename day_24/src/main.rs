use std::{fs::File, io, path::Path};
use io::BufRead;

fn main() {
    let mut packages: Vec<usize> = Vec::new();
    let mut sum = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                match row.parse::<usize>() {
                    Ok(val) => { 
                        packages.push(val);
                        sum = sum + val;
                    },
                    Err(_) => {}
                }
            }
        }
    }

    sum = sum / 4;
    let len = packages.len() / 4;
    let mut min = usize::MAX;
    let mut map: Vec<Vec<Vec<usize>>> = vec![Vec::new(); 28];


    for pack_1 in 4..=len {
        if map[pack_1 - 1].len() == 0 { map[pack_1 - 1] = gen_map(pack_1, sum, &packages); }
        
        let mut pack_2 = pack_1;
        let len_2 = (packages.len() - pack_1) / 4;
        while pack_2 <= len_2 {
            let mut left = pack_2;
            let mut right = packages.len() - pack_1 - pack_2 - left;

            if map[pack_2 - 1].len() == 0 { map[pack_2 - 1] = gen_map(pack_2, sum, &packages); }

            while left <= right {
                if map[left - 1].len() == 0 { map[left - 1] = gen_map(left, sum, &packages); }
                if map[right - 1].len() == 0 { map[right - 1] = gen_map(right, sum, &packages); }

                let len_pack_1 = map[pack_1 - 1].len();
                let len_pack_2 = map[pack_2 - 1].len();
                let len_left = map[left - 1].len();
                let len_right = map[right - 1].len();

                println!("{}: {}, {}: {}, {}: {}, {}: {}", pack_1, len_pack_1, pack_2, len_pack_2,
                        left, len_left, right, len_right);

                for p1 in &map[pack_1 - 1] {
                    for p2 in &map[pack_2 - 1] {
                        let mut check = false;
                        for i in 0..packages.len() {
                            if p1[i] == 1 && p2[i] == 1 {
                                check = true;
                                break;
                            }
                        }
                        if check { continue; }

                        let mut conti = false;
                        for p3 in &map[left - 1] {
                            let mut check = false;
                            for i in 0..packages.len() {
                                if p3[i] == 1 && (p1[i] == 1 || p2[i] == 1) {
                                    check = true;
                                    break;
                                }
                            }
                            if check { continue; }

                            for p4 in &map[right - 1] {
                                let mut check = false;
                                for i in 0..packages.len() {
                                    if p4[i] == 1 && (p1[i] == 1 || p2[i] == 1 || p3[i] == 1) {
                                        check = true;
                                        break;
                                    }
                                }
                                if check { continue; }
                                let mut load = 1;
                                
                                for i in 0..packages.len() {
                                    if p1[i] == 1 { load = load * packages[i] }
                                }
                                if load < min {
                                    min = load;
                                }
                                println!("{:?}, {}", p1, load);
                                conti = true;
                                break;
                            }
                            if conti { break; }                            
                        }
                        if conti { break; }
                    }
                }

                left = left + 1;
                right = right- 1;
            }
            pack_2 = pack_2 + 1;
        }
    
        if min != usize::MAX {
            break;
        }
    }

    println!("{}", min);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn gen_map(p_len: usize, sum: usize, packages: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut stack: Vec<Vec<usize>> = Vec::new();
    
    let mut permu = vec![1; p_len];
    permu.append(&mut vec![2; packages.len() - p_len]);

    loop {
        let mut val = 0;
        for i in 0..permu.len() {
            if permu[i] == 1 { val = val + packages[i] }
        }

        if val == sum { stack.push(permu.clone()) }

        if next_permutation(&mut permu, packages.len()) { continue; }
        
        break;
    }

    stack
}

fn next_permutation(permu: &mut Vec<usize>, len: usize) -> bool {
    for i in (1..len).rev() {
        if permu[i - 1] < permu[i] {
            let mut tmp = len - 1;
            while permu[tmp] <= permu[i - 1] {
                tmp = tmp - 1;
            }

            permu.swap(tmp, i - 1);

            let mut left = i;
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
