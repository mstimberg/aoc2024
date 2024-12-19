use std::collections::HashMap;
use std::fs;

// simple recursion (too slow for large input, but good enough for the towels)
fn possibilities(towels: &Vec<&str>, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            count += possibilities(towels, &pattern[towel.len()..]);
        }
    }
    count
}

fn fill_possibilities<'a>(
    towels: &Vec<&str>,
    pattern: &'a str,
    memo: &mut HashMap<&'a str, usize>,
) {
    // println!("Pattern: {}, Current memo: {:?}", pattern, memo);
    if memo.contains_key(pattern) || pattern.is_empty() {
        return;
    }
    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            fill_possibilities(towels, &pattern[towel.len()..], memo);
            count += memo[&pattern[towel.len()..]];
        } else {
            memo.insert(&pattern, 0);
        }
    }
    memo.insert(pattern, count);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let towels = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    let patterns = lines.skip(1).collect::<Vec<_>>();
    let mut count = 0;
    for pattern in patterns {
        println!("pattern: {} ", pattern);
        let filtered_towels = towels
            .iter()
            .filter(|&x| pattern.contains(x))
            .map(|&x| x)
            .collect::<Vec<_>>();
        let mut memo = HashMap::new();
        for filtered_towel in &filtered_towels {
            memo.insert(
                *filtered_towel,
                possibilities(&filtered_towels, filtered_towel),
            );
        }
        fill_possibilities(&filtered_towels, &pattern, &mut memo);
        let p = memo[&pattern];
        println!("  possibilities: {}", p);
        count += p;
    }
    println!("Total different ways: {}", count);
}
