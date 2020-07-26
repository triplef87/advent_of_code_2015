use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count: u64 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(row) = line {
                count = count + get_ribbon(row) as u64;
            }
        }
    }
    println!("Total is: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_size(row: String) -> u32 {
    let box_size: Vec<u32> = row.split("x").map(|element| {
        element.parse::<u32>().unwrap()
    }).collect();
    let area_1 = box_size[0] * box_size[1];
    let area_2 = box_size[0] * box_size[2];
    let area_3 = box_size[1] * box_size[2];
    let mut min_area: u32;

    if area_1 <= area_2 {
        min_area = area_1;
    } else {
        min_area = area_2;
    }

    if min_area > area_3 {
        min_area = area_3;
    }

    2 * (area_1 + area_2 + area_3) + min_area
}

fn get_ribbon(row: String) -> u32 {
    let box_size: Vec<u32> = row.split("x").map(|element| {
        element.parse::<u32>().unwrap()
    }).collect();

    let total = box_size[0] * box_size[1] * box_size[2];
    let dis_1 = box_size[0] + box_size[1];
    let dis_2 = box_size[0] + box_size[2];
    let dis_3 = box_size[1] + box_size[2];
    let mut min_dis: u32;

    if dis_1 <= dis_2 {
        min_dis = dis_1;
    } else {
        min_dis = dis_2;
    }

    if min_dis > dis_3 {
        min_dis = dis_3;
    }

    2 * min_dis + total
}