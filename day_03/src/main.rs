use itertools::Itertools;
use regex::Regex;
use std::fs;

fn extract_values(text: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    re.find_iter(text)
        .map(|x| {
            x.as_str()[4..x.as_str().len() - 1]
                .split(",")
                .map(|i| i.parse::<i32>().unwrap())
                .next_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let extracted = extract_values(&contents);
    println!(
        "sum of multiplications: {}",
        extracted.iter().map(|(x, y)| x * y).sum::<i32>()
    );
}
