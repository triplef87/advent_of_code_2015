use std::{fs::File, io, path::Path};
use io::BufRead;

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    capcity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: usize
}

impl Ingredient {
    fn add(&mut self, ingredient: &Ingredient) {
        self.capcity = self.capcity + ingredient.capcity;
        self.durability = self.durability + ingredient.durability;
        self.flavor = self.flavor + ingredient.flavor;
        self.texture = self.texture + ingredient.texture;
        self.calories = self.calories + ingredient.calories;
    }

    fn get_score(&self) -> u128 {
        let capcity: u128 = if self.capcity > 0 { self.capcity as u128 } else { 0 };
        let durability: u128 = if self.durability > 0 { self.durability as u128 } else { 0 };
        let flavor: u128 = if self.flavor > 0 { self.flavor as u128 } else { 0 };
        let texture: u128 = if self.texture > 0 { self.texture as u128 } else { 0 };
        
        capcity * durability * flavor * texture
    }
}

fn main() {
    let mut ingredients: Vec<Ingredient> = Vec::new();
    let mut cookie = Ingredient{ capcity: 0, durability: 0, flavor: 0, texture: 0, calories: 0};

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(row) = line {
                procees_row(&row, &mut ingredients);
            }
        }
    };
    
    // Part 1
    // for ingredient in &ingredients {
    //     cookie.add(ingredient);
    // }

    // for _ in 0..96 {
    //     let mut max = cookie.clone();
    //     for ingredient in &ingredients {
    //         let mut temp = cookie.clone();
    //         temp.add(ingredient);
    //         if temp.get_score() > max.get_score() {
    //             max = temp;
    //         }
    //     }
    //     cookie = max;
    // }

    let matrix = gen_matrix();

    println!("{}", matrix.len());

    for row in matrix {
        let mut temp = Ingredient{ capcity: 0, durability: 0, flavor: 0, texture: 0, calories: 0};
        
        for (idx, num) in row.iter().enumerate() {
            for _ in 0..*num {
                temp.add(&ingredients[idx]);
            }
        }

        if temp.get_score() > cookie.get_score() {
            cookie = temp;
        }
    }

    println!("{:?} has score: {}",cookie, cookie.get_score());
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn procees_row(row: &String, ingredients: &mut Vec<Ingredient>) {
    let trim: String = row.chars().filter(|c| { 
        match c {
            '0'..='9' | ',' | '-' => true,
            _ => false
        }
    }).map(|c| {
        if c == ',' {
            ' '
        } else {
            c
        }
    }).collect();

    let slice: Vec<&str> = trim.split(' ').collect();
    
    let capcity:isize = slice[0].parse().unwrap();
    let durability: isize = slice[1].parse().unwrap();
    let flavor: isize = slice[2].parse().unwrap();
    let texture: isize = slice[3].parse().unwrap();
    let calories: usize = slice[4].parse().unwrap();

    ingredients.push(Ingredient{ capcity, durability, flavor, texture, calories});
}

fn gen_matrix() -> Vec<Vec<usize>> {
    let mut matrix = Vec::new();

    let mut high = 61;
    let mut low = 4;

    loop {
        for x in 1..low {
            for y in 1..high {
                if high + low == 100 {
                    let row = vec![low - x, x, high - y, y];
                    matrix.push(row);
                }
            }
        }

        if high < 3 {
            break;
        }

        high = high - 3;
        low = low + 8;
    }    

    matrix
}