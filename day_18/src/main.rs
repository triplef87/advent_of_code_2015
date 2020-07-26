use std::{fs::File, io, path::Path};
use io::BufRead;

fn main() {
    let mut light_arr: Vec<Vec<bool>> = Vec::new();
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                let mut tmp:Vec<bool> = Vec::new();
                for c in row.chars() {
                    match c {
                        '#' => tmp.push(true),
                        '.' => tmp.push(false),
                        _ => {}
                    }
                }
                light_arr.push(tmp);
            }
        }
    }

    for _ in 0..100 {
        let mut tmp = light_arr.clone();
        for x in 0..100 {
            for y in 0..100 {
                let mut counter = 0;

                let x_start = if x == 0 { 0 } else { x - 1 };
                let x_end = if x == 99 { x } else { x + 1 };
                let y_start = if y == 0 { 0 } else { y - 1 };
                let y_end = if y == 99 { y } else { y + 1 };

                for x_neighber in x_start..=x_end {
                    for y_neighber in y_start..=y_end {
                        if light_arr[x_neighber][y_neighber] && (x_neighber != x || y_neighber != y) {
                            counter = counter + 1;
                        }
                    }
                }

                if light_arr[x][y] && !(counter == 2 || counter == 3) {
                    tmp[x][y] = false;
                }

                if !light_arr[x][y] && counter == 3 {
                    tmp[x][y] = true;
                }

                if (x == 0 || x == 99) && (y == 0 || y == 99) {
                    tmp[x][y] = true;
                }
            }
        }
        light_arr = tmp;
        println!("-------");
        for row in &light_arr  {
            for val in row {
                if *val {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    let mut counter = 0;
    for row in light_arr {
        for val in row {
            if val {
                counter = counter + 1;
            }
        }
    }

    println!("{}", counter);
    
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}
