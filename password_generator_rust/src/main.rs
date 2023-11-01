use std::char::from_u32;
use rand::{Rng, thread_rng};
use rand;

fn main() {
    let password_length = 15;
    let mut result = String::new();

    for _ in 0..password_length {
        let number = thread_rng().gen_range(48..122);
        let ch = from_u32(number).unwrap();
        result.push(ch);
    }
    
    println!("{}", result);
}
