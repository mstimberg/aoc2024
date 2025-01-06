use itertools::Itertools;
use num::integer::gcd;
use std::fs;

fn find_solution(x_a: i64, y_a: i64, x_b: i64, y_b: i64, x_t: i64, y_t: i64) -> Option<(i64, i64)> {
    let gcd_sub = gcd(x_a - y_a, x_b - y_b);
    if (x_t - y_t) % gcd_sub != 0 {
        return None;
    }
    let xy_a = (x_a - y_a) / gcd_sub;
    let xy_b = (x_b - y_b) / gcd_sub;
    let xy_t = (x_t - y_t) / gcd_sub;
    println!("New system: a*{xy_a} + b*{xy_b} = {xy_t}");
    // Rearranging: a = xy_t/xy_a - xy_b/xy_a*b
    // Inserting into basic equation for x and solving for b
    // b = (x_t - xy_t/xy_a*x_a)/(xy_b/xy_a*x_a + x_b)
    // intermediate results are not necessarily integers, calculate with floats
    let b = ((x_t as f64 - (xy_t as f64) / (xy_a as f64) * (x_a as f64))
        / ((-xy_b as f64) / (xy_a as f64) * (x_a as f64) + x_b as f64)).round() as i64;
    let a = ((x_t as f64 - (b as f64) * (x_b as f64)) / (x_a as f64)).round() as i64;
    if a > 0
        && b > 0
        && a*x_a + b*x_b == x_t
        && a*y_a + b*y_b == y_t
    {
        return Some((a, b));
    }
    None
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
                a_description[0].trim()[2..].parse::<i64>().unwrap(),
                a_description[1].trim()[2..].parse::<i64>().unwrap(),
            );
            let b_description = l2[10..].split(",").collect_vec();
            let (x_b, y_b) = (
                b_description[0].trim()[2..].parse::<i64>().unwrap(),
                b_description[1].trim()[2..].parse::<i64>().unwrap(),
            );
            let t_description = l3[7..].split(",").collect_vec();
            let (x_t, y_t) = (
                t_description[0].trim()[2..].parse::<i64>().unwrap(),
                t_description[1].trim()[2..].parse::<i64>().unwrap(),
            );
            (x_a, y_a, x_b, y_b, x_t, y_t)
        })
        .collect::<Vec<_>>();
    let mut total_prizes = 0;
    let offset = 10_000_000_000_000;
    for machine in machines {
        // println!("{:?}", machine);
        let (x_a, y_a, x_b, y_b, x_t, y_t) = machine;
        let solution = find_solution(x_a, y_a, x_b, y_b, x_t + offset, y_t + offset);
        match solution {
            Some((a, b)) => {
                println!("  Solution: a = {}, b = {}", a, b);
                total_prizes += 3 * a + b
            }
            None => {
                println!("  No solution.");
                continue;
            }
        }
    }
    println!("Total prizes: {}", total_prizes);
}
