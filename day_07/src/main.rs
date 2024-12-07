use itertools::Itertools;
use std::fs;

fn solve(result: i64, numbers: &Vec<i32>) -> i64 {
    // brute force…
    for op_number in 0..2_i32.pow(numbers.len() as u32 - 1) {
        let mut total = numbers[0] as i64;
        for (i, &number) in numbers.iter().skip(1).enumerate() {
            if (op_number >> i) & 1 == 1 {
                total += number as i64;
            } else {
                total *= number as i64;
            }
        }
        if total == result {
            return total;
        }
    }
    return 0; // no solution
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let calculations = contents
        .lines()
        .map(|line| line.split(':').next_tuple::<(&str, &str)>().unwrap())
        .map(|(a, b)| {
            (
                a.parse::<i64>().unwrap(),
                b.trim()
                    .split(' ')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect::<Vec<(i64, Vec<i32>)>>();
    let total = calculations
        .into_iter()
        .map(|(result, numbers)| solve(result, &numbers))
        .sum::<i64>();
    println!("Total: {}", total);
}
