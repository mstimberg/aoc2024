use std::fs;

fn blink(numbers: Vec<i64>) -> Vec<i64> {
    let mut new_numbers = Vec::new();
    for n in numbers {
        if n == 0 {
            new_numbers.push(1);
        } else if (n.ilog10() + 1) % 2 == 0 {
            let tens = 10_i64.pow((n.ilog10() + 1) / 2 as u32);
            new_numbers.push(n / tens as i64);
            new_numbers.push(n % tens as i64);
        } else {
            new_numbers.push(n * 2024);
        }
    }
    new_numbers
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let numbers = contents
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("Initial arrangement:\n {:?}\n", numbers);
    let mut new_numbers = numbers.clone();
    for b in 0..25 {
        new_numbers = blink(new_numbers);
        // println!("After {} blinks:\n{:?}\n", b+1, new_numbers);
    }
    println!("Number of stones after 25 blinks: {}", new_numbers.len());
}
