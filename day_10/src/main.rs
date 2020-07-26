fn main() {
    let mut string = String::from("1321131112");
    
    for _ in 0..50 {
        string = count_and_say(string);
    }
    println!("{}", string.len());
}

fn count_and_say(string: String) -> String {
    let mut tmp = String::from("");
    let mut buf = ' ';
    let mut counter: usize = 1;

    for c in string.chars() {
        if buf == ' ' {
            buf = c;
        } else if buf == c {
            counter = counter + 1;
        } else {
            tmp.push_str(&counter.to_string());
            tmp.push(buf);
            buf = c;
            counter = 1;
        }
    }

    tmp.push_str(&counter.to_string());
    tmp.push(buf);

    tmp
}
