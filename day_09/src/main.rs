use std::fs;
use std::iter::repeat;

fn expand(input: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let digits = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    // let's assume there's an uneven number of digits (seems to be the case in the examples)
    result.extend(repeat(0).take(digits[0] as usize).collect::<Vec<_>>());
    let mut i = 1;
    while 2 * i - 1 < digits.len() {
        result.extend(
            repeat(-1)
                .take(digits[2 * i - 1] as usize)
                .collect::<Vec<_>>(),
        );
        result.extend(
            repeat(i as i32)
                .take(digits[2 * i] as usize)
                .collect::<Vec<_>>(),
        );
        i += 1;
    }
    result
}

fn compressed(input: &Vec<i32>) -> Vec<i32> {
    let mut result = input.clone();
    // This involves a lot of unncessary searching but it's good enough for now
    let mut current_id = *result.iter().max().unwrap();
    while current_id >= 0 {
        println!("{}", current_id);
        let length = result.iter().filter(|&x| *x == current_id).count();
        let start = result.iter().position(|x| *x == current_id).unwrap();
        let mut space_counter = 0;
        for i in 0..start {
            if result[i] == -1 {
                space_counter += 1;
            } else {
                space_counter = 0;
            }
            if space_counter == length {
                result[i - length + 1..i + 1].fill(current_id);
                result[start..start + length].fill(-1);
                break;
            }
        }
        current_id -= 1;
    }
    result
}

fn checksum(input: &Vec<i32>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter(|(_, id)| **id != -1)
        .map(|(i, id)| i as i64 * *id as i64)
        .sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let expanded = expand(&contents);
    // println!("Expanded: {:?}", expanded);
    let compressed = compressed(&expanded);
    // println!("Compressed: {:?}", compressed);
    let result = checksum(&compressed);
    println!("Checksum: {}", result);
}
