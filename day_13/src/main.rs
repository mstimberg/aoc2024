use itertools::Itertools;
use std::fs;

fn find_solutions(x_a: u32, y_a: u32, x_b: u32, y_b: u32, x_t: u32, y_t: u32) -> Vec<(u32, u32)> {
    let mut solutions = Vec::new();
    for a in 0..100 {
        for b in 0..100 {
            if a * x_a + b * x_b == x_t && a * y_a + b * y_b == y_t {
                solutions.push((a, b));
            } else if a * x_a + b * x_b > x_t || a * y_a + b * y_b > y_t {
                break;
            }
        }
    }
    solutions
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let machines = contents
        .lines()
        .filter(|x| !x.is_empty())
        .chunks(3)
        .into_iter()
        .map(|c| {
            let (l1, l2, l3) = c.collect_tuple().unwrap();
            let a_description = l1[10..].split(",").collect_vec();
            let (x_a, y_a) = (
                a_description[0].trim()[2..].parse::<u32>().unwrap(),
                a_description[1].trim()[2..].parse::<u32>().unwrap(),
            );
            let b_description = l2[10..].split(",").collect_vec();
            let (x_b, y_b) = (
                b_description[0].trim()[2..].parse::<u32>().unwrap(),
                b_description[1].trim()[2..].parse::<u32>().unwrap(),
            );
            let t_description = l3[7..].split(",").collect_vec();
            let (x_t, y_t) = (
                t_description[0].trim()[2..].parse::<u32>().unwrap(),
                t_description[1].trim()[2..].parse::<u32>().unwrap(),
            );
            (x_a, y_a, x_b, y_b, x_t, y_t)
        })
        .collect::<Vec<_>>();
    let mut total_prizes = 0;
    for machine in machines {
        println!("{:?}", machine);
        let (x_a, y_a, x_b, y_b, x_t, y_t) = machine;
        let solutions = find_solutions(x_a, y_a, x_b, y_b, x_t, y_t);
        let prizes = solutions.iter().map(|(a, b)| a * 3 + b).collect::<Vec<_>>();
        println!("solutions: {:?}", solutions);
        println!("prices: {:?}", prizes);
        if prizes.is_empty() {
            continue;
        }
        total_prizes += prizes.iter().min().unwrap();
    }
    println!("Total prizes: {}", total_prizes);
}
