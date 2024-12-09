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
    let mut result = Vec::new();
    let mut reverse_iterator = input.iter().rev();
    let n_spaces = input.iter().filter(|&c| *c == -1).count();
    for i in 0..input.len() - n_spaces {
        if input[i] != -1 {
            // copy over
            result.push(input[i]);
        } else {
            let mut rightmost_id;
            loop {
                rightmost_id = reverse_iterator.next().unwrap();
                if *rightmost_id != -1 {
                    break;
                }
            }
            result.push(*rightmost_id);
        }
    }
    result
}

fn checksum(input: &Vec<i32>) -> i64 {
    input
        .iter()
        .enumerate()
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
