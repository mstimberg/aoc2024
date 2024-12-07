use itertools::Itertools;
use radix_fmt::radix_3;
use std::fs;

fn solve(result: i64, numbers: &Vec<i32>) -> i64 {
    // brute force…
    for op_number in 0..3_i32.pow(numbers.len() as u32 - 1) {
        let op_str = format!(
            "{:0>width$}",
            radix_3(op_number as u64).to_string(),
            width = numbers.len() - 1
        );
        let mut total = numbers[0] as i64;
        for (i, &number) in numbers.iter().skip(1).enumerate() {
            let op = op_str.as_bytes()[i];
            if op == b'0' {
                total += number as i64;
            } else if op == b'1' {
                total *= number as i64;
            } else if op == b'2' {
                // concatenate the numbers
                total *= 10_i64.pow(number.ilog10() as u32 + 1);
                total += number as i64;
            }
            if total > result {
                // no need to continue
                break;
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
