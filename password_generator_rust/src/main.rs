use rand::Rng;

fn main() {
    let password_length = 15;
    let mut result = String::new();
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    for _ in 0..password_length {
        let random_index = rand::thread_rng().gen_range(0..charset.len());
        let random_char = charset.chars().nth(random_index).unwrap();
        result.push(random_char);
    }

    println!("{}", result);
}
