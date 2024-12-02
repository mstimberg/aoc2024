use itertools::Itertools;
use itertools::sorted;
use std::fs;

fn distance(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    sorted(l1).zip(sorted(l2)).map(|(x, y)| (x - y).abs()).sum()      
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let tuples: Vec<_> = contents.lines().map(|line| {
            line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).next_tuple::<(i32, i32)>().unwrap()
    }).collect();
    let (vec1, vec2): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();
    println!("{}", distance(vec1, vec2));
}
