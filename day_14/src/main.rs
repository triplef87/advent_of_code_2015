use std::{fs::File, io, path::Path};
use io::BufRead;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    // for part2
    is_fly: bool,
    counter: usize,
    dist: usize,
    point: usize
}

impl Reindeer {
    fn travel(&self, total_time: usize) -> usize {
        let turn_time = self.fly_time + self.rest_time;
        let turn_num = total_time / turn_time;
        let mut remain = total_time - turn_num * turn_time;
        
        if remain > self.fly_time {
            remain = self.fly_time
        }
        println!("{}, {}, {}", turn_time, turn_num, remain);

        let dist = self.speed * (turn_num * self.fly_time + remain);

        dist
    }

    fn step(&mut self) {
        if self.is_fly {
            self.dist = self.dist + self.speed;
        }

        self.counter = self.counter - 1;

        if self.counter == 0 {
            if self.is_fly {
                self.counter = self.rest_time;
            } else {
                self.counter = self.fly_time;
            }
            self.is_fly = !self.is_fly
        }
        // println!("{}'s state is {}, dist is {}, point is {}", self.name, self.is_fly, self.dist, self.point);
    }
}


fn main() {
    let mut reindeers: Vec<Reindeer> = Vec::new();
    let mut winner = 0;

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                process_row(&row, &mut reindeers);
            }
        }
    }

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            reindeer.step()
        }

        let mut record = (0, 0);
        for (idx, reindeer) in reindeers.iter().enumerate() {
            if reindeer.dist > record.1 {
                record = (idx, reindeer.dist)
            }
        }

        reindeers[record.0].point = reindeers[record.0].point + 1;
    }

    for reindeer in &reindeers {
        if winner < reindeer.point {
            winner = reindeer.point;
        }
    }

    println!("{}", winner);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(row: &String, reindeers: &mut Vec<Reindeer>) {
    let slice: Vec<&str> = row.split(" ").collect();

    let name = String::from(slice[0]);
    let speed: usize = slice[3].parse().unwrap();
    let fly_time: usize = slice[6].parse().unwrap();
    let rest_time: usize = slice[13].parse().unwrap();
    let is_fly= true;
    let counter = fly_time;
    let dist = 0;
    let point = 0;

    reindeers.push(Reindeer{ name, speed, fly_time, rest_time, is_fly, counter, dist, point });
}
