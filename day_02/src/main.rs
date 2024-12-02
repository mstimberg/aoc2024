use std::fs;

fn count_safe(reports : &Vec::<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for report in reports {
        let diffs = report.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();
        if diffs.iter().all(|&x| x >= 1 && x<= 3) || (diffs.iter().all(|&x| x >= -3 && x <= -1 )){
            sum += 1;
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
