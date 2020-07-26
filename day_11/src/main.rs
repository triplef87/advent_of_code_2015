fn main() {
    let mut pwd = String::from("hxbxxyzz");

    let mut check = false;

    while !check {
        pwd = get_next(pwd);
        check = condition_1(pwd.clone());
        check = condition_2(pwd.clone(), check);
        check = condition_3(pwd.clone(), check);
    }

    println!("{}", pwd);
}

fn get_next(old: String) -> String {
    let mut new = String::from("");
    let mut plus = true;
    
    for c in old.chars().rev() {
        if plus {
            if c == 'z' {
                new.push('a');
            } else {
                plus = false;
                
                let c_next = c as u8 + 1;
                new.push(c_next as char);
            }
        } else {
            plus = false;
            new.push(c);
        }
    }

    new.chars().rev().collect()
}

fn condition_1(pwd: String) -> bool {
    let len = pwd.len();
    let mut pwd_array = pwd.chars();
    let mut pos = 2;
    let mut counter = 0;

    let mut left = pwd_array.nth(0).unwrap() as i32;
    let mut right = pwd_array.nth(0).unwrap() as i32;

    while pos < len {
        if right - left == 1 {          
            counter = counter + 1;
        } else {
            counter = 0;
        }

        if counter == 2 {
            return true;
        }

        left = right;
        right = pwd_array.nth(0).unwrap() as i32;
        pos = pos + 1; 
    }

    false
}

fn condition_2(pwd: String, conti: bool) -> bool {
    if !conti {
        return false;
    }

    for c in pwd.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }
    }

    true
}

fn condition_3(pwd: String, conti: bool) -> bool {
    if !conti {
        return false;
    }

    let mut counter = 0;
    let mut buf = ' ';
    let mut prev = ' ';

    for c in pwd.chars() {
        if c == buf && buf != prev {
            counter = counter + 1;
            prev = buf;
        } else {
            buf = c;
        }

        if counter == 2 {
            return true;
        }
    }

    false
}