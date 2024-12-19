use std::fs;

fn is_possible(towels: &Vec<&str>, pattern: &str, from_start: bool) -> bool {
    // Check patterns from start and end to avoid getting stuck in long backtracking
    println!(
        "Checking pattern: {} from {}",
        pattern,
        if from_start { "start" } else { "end" }
    );
    if pattern.is_empty() {
        return true;
    }
    // switch between start and end
    for towel in towels {
        if from_start && pattern.starts_with(towel) {
            if is_possible(towels, &pattern[towel.len()..], false) {
                return true;
            }
        } else if !from_start && pattern.ends_with(towel) {
            if is_possible(towels, &pattern[..pattern.len() - towel.len()], true) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let towels = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    let patterns = lines.skip(1).collect::<Vec<_>>();
    let mut count = 0;
    for pattern in patterns {
        println!("new pattern: {} ", pattern);
        if is_possible(&towels, pattern, true) {
            println!("is possible");
            count += 1;
        } else {
            println!("is not possible");
        }
    }
    println!("{} patterns are possible", count);
}
