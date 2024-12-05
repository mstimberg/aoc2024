use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn verify_pages(rules: &Vec<(&str, &str)>, pages: &Vec<&str>) -> bool {
    let page_set = HashSet::from_iter(pages.iter().cloned());
    for rule in rules {
        let rule_set: HashSet<&str> = HashSet::from_iter([rule.0, rule.1].iter().cloned());
        if rule_set.is_subset(&page_set) {
            if !(pages.iter().position(|&p| p == rule.0).unwrap()
                < pages.iter().position(|&p| p == rule.1).unwrap())
            {
                return false;
            }
        }
    }
    return true;
}

fn fix_pages<'a>(rules: &Vec<(&str, &str)>, pages: &Vec<&'a str>) -> Vec<&'a str> {
    let page_set = HashSet::from_iter(pages.iter().cloned());
    let mut fixed_pages = pages.clone();
    // A bit of brute-force...
    while !verify_pages(rules, &fixed_pages) {
        for rule in rules {
            let rule_set: HashSet<&str> = HashSet::from_iter([rule.0, rule.1].iter().cloned());
            if rule_set.is_subset(&page_set) {
                let pos1 = fixed_pages.iter().position(|&p| p == rule.0).unwrap();
                let pos2 = fixed_pages.iter().position(|&p| p == rule.1).unwrap();
                if pos1 > pos2 {
                    fixed_pages.swap(pos1, pos2);
                    break;
                }
            }
        }
    }
    fixed_pages
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = contents.lines().collect();
    // Split into two vectors, along the empty line
    let (first, second) = lines.split_at(lines.iter().position(|&x| x.is_empty()).unwrap());
    let rules: Vec<_> = first
        .iter()
        .map(|line| line.split('|').next_tuple::<(&str, &str)>().unwrap())
        .collect();
    let all_pages: Vec<Vec<&str>> = second[1..] // skip the empty line
        .into_iter()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .into_iter()
        .collect();
    let sum_correct: i32 = all_pages
        .iter()
        .filter(|pages| verify_pages(&rules, pages))
        .map(|p| p[p.len() / 2].parse::<i32>().unwrap())
        .sum();
    println!("Sum of correct pages: {}", sum_correct);
    let sum_fixed_incorrect: i32 = all_pages
        .iter()
        .filter(|pages| !verify_pages(&rules, pages))
        .map(|pages| {
            fix_pages(&rules, pages)[pages.len() / 2]
                .parse::<i32>()
                .unwrap()
        })
        .sum();
    println!("Sum of fixed incorrect pages: {}", sum_fixed_incorrect);
}
