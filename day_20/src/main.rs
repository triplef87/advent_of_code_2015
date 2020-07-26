fn main() {
    let goal = 33100000;
    let max = goal / 10;
    let mut houses = vec![0; max + 1];

    for x in 1..max+1 {
        let mut y = x;
        let mut counter = 1;
        while y <= max {
            houses[y] = houses[y] + x * 11;

            y = y + x;
            
            if counter == 50 {
                break;
            }

            counter = counter + 1;
        }
    }

    for idx in 1..max+1 {
        if houses[idx] > goal {
            println!("{}", idx);
            break;
        }
    }
}
