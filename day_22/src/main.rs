use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn next_number(number: i64) -> (i64, i8, i8) {
    let mut new_number;
    new_number = (number ^ (number * 64)) % 16777216;
    new_number = (new_number ^ (new_number / 32)) % 16777216;
    new_number = (new_number ^ (new_number * 2048)) % 16777216;
    let last_digit = (new_number % 10) as i8;
    let change = last_digit - ((number % 10) as i8);
    (new_number, last_digit, change as i8)
}

fn signal_stop(stop_signal: &(i8, i8, i8, i8), all_prices: &Vec<Vec<i8>>, all_changes: &Vec<Vec<i8>>) -> i64{
    let mut total = 0;
    for (prices, changes) in all_prices.iter().zip(all_changes) {
        for (i, c) in changes.windows(4).enumerate() {
            if c[0] == stop_signal.0 && c[1] == stop_signal.1 && c[2] == stop_signal.2 && c[3] == stop_signal.3 {
                total += prices[i + 3] as i64;
                break;
            }
        }
    }
    total
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should not happen");
    let secrets = content.lines().map(|l| l.parse::<i64>().unwrap());
    let mut total = 0;
    let mut all_prices = Vec::new();
    let mut all_changes = Vec::new();
    for secret in secrets {
        let mut secret = secret;
        let mut prices = Vec::new();
        let mut changes = Vec::new();
        for _ in 0..2000 {
            let result = next_number(secret);
            secret = result.0;
            prices.push(result.1);
            changes.push(result.2);
            // println!("{}", secret);
        }
        all_prices.push(prices);
        all_changes.push(changes);
    }
    // really not elegant brute-force… slow but working
    let mut stop_signals = HashSet::<(i8, i8, i8, i8)>::new();
    for changes in all_changes.clone() {
        for stop_signal in changes.windows(4) {
            stop_signals.insert((stop_signal[0], stop_signal[1], stop_signal[2], stop_signal[3]));
        }
    }
    let mut max_bananas = 0;
    for (i, stop_signal) in stop_signals.iter().enumerate() {
        println!("{}/{}", i+1, stop_signals.len());
        let bananas = signal_stop(stop_signal, &all_prices, &all_changes);
        if bananas > max_bananas {
            max_bananas = bananas;
        }
    }
    // println!("stop signals: {:?}", stop_signals);
    println!("Max bananas: {}", max_bananas);
}
