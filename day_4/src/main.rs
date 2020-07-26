extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let base = "iwrupvqb";
    let mut counter = 0;

    loop {
        let hash = format!("{}{}", base, counter);
        let mut encode = Md5::new();
    
        encode.input_str(&hash);
        let result = encode.result_str();

        // if result.starts_with("00000") {
        //     println!("{}", result);
        //     break;
        // }
        if result.starts_with("000000") {
            println!("{}", result);
            break;
        }
        counter = counter + 1;
    }
    println!("Number is {}", counter);
}
