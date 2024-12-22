use std::fs;

fn next_number(number: i64) -> i64 {
    let mut new_number;
    new_number = (number ^ (number * 64)) % 16777216;
    new_number = (new_number ^ (new_number / 32)) % 16777216;
    new_number = (new_number ^ (new_number * 2048)) % 16777216;
    new_number
}
fn main() {
    let content = fs::read_to_string("input.txt").expect("Should not happen");
    let secrets = content.lines().map(|l| l.parse::<i64>().unwrap());
    let mut total = 0;
    for secret in secrets {
        let mut secret = secret;
        for _ in 0..2000 {
            secret = next_number(secret);
            // println!("{}", secret);
        }
        println!("{secret}");
        total += secret;
    }
    println!("Total: {}", total);
}
