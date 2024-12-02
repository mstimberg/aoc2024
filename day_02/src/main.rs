use std::fs;

fn sequence_count(levels: &Vec<i32>) -> (usize, usize) {
    let diffs = levels.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();
    let count_increasing = diffs.iter().filter(|&x| *x >= 1 && *x <= 3).count();
    let count_decreasing = diffs.iter().filter(|&x| *x >= -3 && *x <= -1).count();
    (count_increasing, count_decreasing)
}

fn count_safe(reports: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for report in reports {
        let (count_increasing, count_decreasing) = sequence_count(report);
        if count_increasing == report.len() - 1 || count_decreasing == report.len() - 1 {
            sum += 1;
        } else if count_increasing >= report.len() - 3 || count_decreasing >= report.len() - 3 {
            // brute force...
            for i in 0..report.len() {
                let mut smaller_report = report.clone();
                smaller_report.remove(i);
                let (ci, cd) = sequence_count(&smaller_report);
                if ci == smaller_report.len() - 1 || cd == smaller_report.len() - 1 {
                    sum += 1;
                    break;
                }
            }
        }
    }
    sum
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let reports: Vec<_> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let safe = count_safe(&reports);
    println!("safe: {}", safe);
}
