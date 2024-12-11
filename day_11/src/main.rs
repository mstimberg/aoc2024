use std::collections::HashMap;
use std::{fs, usize};

#[derive(Debug, Clone)]
enum DictEntry {
    Reference(usize, i64),
    Number(i64),
}

fn n_blink_number(
    blinks: usize,
    orig_n: i64,
    dictionary: &mut HashMap<(usize, i64), Vec<DictEntry>>,
) -> Vec<DictEntry> {
    // println!("Calculating for {} blinks and number {}", blinks, orig_n);
    if dictionary.contains_key(&(blinks, orig_n)) {
        return dictionary[&(blinks, orig_n)].clone();
    }
    // we have to calculate things ourselves
    let mut new_numbers = Vec::new();
    if orig_n == 0 {
        new_numbers.push(DictEntry::Number(1));
    } else if (orig_n.ilog10() + 1) % 2 == 0 {
        let tens = 10_i64.pow((orig_n.ilog10() / 2 + 1) as u32);
        new_numbers.push(DictEntry::Number(orig_n / tens as i64));
        new_numbers.push(DictEntry::Number(orig_n % tens as i64));
    } else {
        new_numbers.push(DictEntry::Number(orig_n * 2024));
    }
    dictionary.insert((1, orig_n), new_numbers.clone());
    if blinks > 1 {
        let new_numbers = n_blink(blinks - 1, new_numbers, dictionary);
        dictionary.insert((blinks, orig_n), new_numbers.clone());
    }
    dictionary[&(blinks, orig_n)].clone()
}

fn n_blink(
    blinks: usize,
    orig_numbers: Vec<DictEntry>,
    dictionary: &mut HashMap<(usize, i64), Vec<DictEntry>>,
) -> Vec<DictEntry> {
    // println!("Calculating for {} blinks and numbers {:?}", blinks, orig_numbers);
    let mut new_numbers = Vec::new();
    for n in orig_numbers {
        match n {
            DictEntry::Number(n) => {
                if dictionary.contains_key(&(blinks, n)) {
                    new_numbers.push(DictEntry::Reference(blinks, n));
                } else {
                    new_numbers.extend(n_blink_number(blinks, n, dictionary));
                }
            }
            DictEntry::Reference(b, n) => {
                new_numbers.extend(n_blink_number(blinks - 1, n, dictionary));
            }
        }
    }
    new_numbers
}

fn total_count(
    numbers: Vec<DictEntry>,
    memo: &mut HashMap<(usize, i64), u64>,
    dictionary: &HashMap<(usize, i64), Vec<DictEntry>>,
) -> u64 {
    let mut count = 0;
    for number in numbers {
        match number {
            DictEntry::Number(n) => {
                count += 1;
            }
            DictEntry::Reference(b, n) => {
                if memo.contains_key(&(b, n)) {
                    count += memo[&(b, n)];
                } else {
                    let entries = dictionary[&(b, n)].clone();
                    let new_count = total_count(entries, memo, dictionary);
                    memo.insert((b, n), new_count);
                    count += new_count;
                }
            }
        }
    }
    count
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let numbers = contents
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("Initial arrangement:\n {:?}\n", numbers);
    let blinks = 75;
    let mut memoizers = HashMap::new();
    let result = n_blink(
        blinks,
        numbers.iter().map(|n| DictEntry::Number(*n)).collect(),
        &mut memoizers,
    );
    println!("After {} blinks:\n {:?}", blinks, result.len());
    let mut count_memo: HashMap<(usize, i64), u64> = HashMap::new();
    let count = total_count(result, &mut count_memo, &memoizers);
    println!("Flattened:\n {:?}", count);
    // println!("Memo:\n {:?}", memoizers);
}
