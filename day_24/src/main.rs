use core::panic;
use itertools::{sorted, Itertools};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Logical {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    i1: String,
    i2: String,
    op: Logical,
    output: String,
}

fn evaluate(gates: &Vec<Gate>, wires: &mut HashMap<String, bool>, gate: Gate) {
    if !wires.clone().contains_key(&gate.i1) {
        for g in gates.iter() {
            if g.output == gate.i1 {
                evaluate(gates, wires, g.clone());
                break;
            }
        }
    }
    if !wires.clone().contains_key(&gate.i2) {
        for g in gates.iter() {
            if g.output == gate.i2 {
                evaluate(gates, wires, g.clone());
                break;
            }
        }
    }
    let i1 = wires.get(&gate.i1).unwrap();
    let i2 = wires.get(&gate.i2).unwrap();
    let result = match gate.op {
        Logical::AND => i1 & i2,
        Logical::OR => i1 | i2,
        Logical::XOR => i1 ^ i2,
    };
    wires.insert(gate.output.clone(), result);
}

fn has_loop<'a>(
    gates: &'a Vec<Gate>,
    output: &'a str,
    visited: &mut HashSet<&'a str>,
    verbose: bool,
) -> bool {
    if visited.contains(output) {
        return true;
    }
    for gate in gates {
        if gate.output == output {
            visited.insert(output);
            if has_loop(gates, &gate.i1, visited, verbose)
                || has_loop(gates, &gate.i2, visited, verbose)
            {
                return true;
            }
        }
    }
    return false;
}

fn find_gates(gates: &Vec<Gate>, output: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    for gate in gates {
        if gate.output == output {
            result.insert(gate.output.clone());
            result.extend(find_gates(gates, &gate.i1));
            result.extend(find_gates(gates, &gate.i2));
        }
    }
    result
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let mut wires: HashMap<String, bool> = HashMap::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split(":").collect();
        wires.insert(parts[0].to_string(), parts[1].trim() == "1");
    }
    let mut gates: Vec<Gate> = Vec::new();
    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.split(" ").collect();
        let gate = Gate {
            i1: parts[0].to_string(),
            i2: parts[2].to_string(),
            op: match parts[1] {
                "AND" => Logical::AND,
                "OR" => Logical::OR,
                "XOR" => Logical::XOR,
                _ => panic!("Invalid operator"),
            },
            output: parts[4].to_string(),
        };
        gates.push(gate);
    }

    // Store original values for second part
    let original_wires = wires.clone();
    let original_gates = gates.clone();
    for gate in original_gates.clone() {
        if gate.output.starts_with("z") {
            evaluate(&gates, &mut wires, gate);
        }
    }

    let mut z_values: Vec<bool> = Vec::new();
    for z_idx in (0..99).rev() {
        let z_key = format!("z{:02}", z_idx);
        if wires.contains_key(&z_key) {
            z_values.push(*wires.get(&z_key).unwrap());
        }
    }
    println!("Z values: {:?}", z_values);
    let total_z = z_values.iter().fold(0, |acc, x| acc * 2 + (*x as i64));
    println!("Total Z: {}", total_z);
    let mut do_not_touch = HashSet::new();
    let mut new_gates = original_gates.clone();
    let mut swapped = Vec::new();
    for bit in 0..45 {
        // first bit is correct
        let mut tried = 0;
        let mut gates = new_gates.clone();
        'outer: loop {
            let mut all_correct = true;
            'inner: for carry in [0, 1].iter() {
                for x in [0, 1].iter() {
                    for y in [0, 1].iter() {
                        // println!("Checking for bit {}, x={}, y={}, carry={}, swap number {}", bit, x, y, carry, tried);
                        // Set initial values
                        let mut wires = original_wires.clone();

                        for before in 0..46 {
                            wires.insert(format!("x{before:02}"), false);
                            wires.insert(format!("x{before:02}"), false);
                        }
                        if carry == &1 && bit > 0 {
                            wires.insert(format!("x{b:02}", b = bit - 1).to_string(), true);
                            wires.insert(format!("y{b:02}", b = bit - 1).to_string(), true);
                        }
                        wires.insert(format!("x{bit:02}").to_string(), *x == 1);
                        wires.insert(format!("y{bit:02}").to_string(), *y == 1);
                        let mut visited = HashSet::new();
                        // println!("  Checking for loop");
                        let loop_graph =
                            has_loop(&gates, &format!("z{bit:02}"), &mut visited, false);
                        let dependencies = if !loop_graph {
                            find_gates(&gates, &format!("z{bit:02}"))
                        } else {
                            HashSet::new()
                        };
                        let mut incorrect_elements = false;
                        for dep in dependencies {
                            for gate in gates.clone() {
                                if gate.output == dep {
                                    if (gate.i1.starts_with("x")
                                        && &gate.i1[1..].parse::<i32>().unwrap() > &bit)
                                        || (gate.i2.starts_with("x")
                                            && &gate.i2[1..].parse::<i32>().unwrap() > &bit)
                                        || (gate.i1.starts_with("y")
                                            && &gate.i1[1..].parse::<i32>().unwrap() > &bit)
                                        || (gate.i2.starts_with("y")
                                            && &gate.i2[1..].parse::<i32>().unwrap() > &bit)
                                    {
                                        // println!("  Incorrect element: {:?}", gate);
                                        incorrect_elements = true;
                                    }
                                    break;
                                }
                            }
                        }
                        if !loop_graph && !incorrect_elements {
                            // println!("  No loop detected");
                            for gate in gates.clone() {
                                if gate.output == format!("z{bit:02}") {
                                    evaluate(&gates, &mut wires, gate);
                                    break;
                                }
                            }
                        }
                        let z = wires.get(&format!("z{bit:02}").to_string());
                        if incorrect_elements
                            || loop_graph
                            || (bit != 0 && *z.unwrap() != ((x ^ y) != *carry)
                                || (bit == 0 && *z.unwrap() != ((x ^ y) != 0)))
                        {
                            // if !loop_graph {
                            //     println!("Incorrect for bit {}, x={}, y={}, carry={}, trying swap number {}", bit, x, y, carry, tried);
                            // } else {
                            //     println!("Loop detected");
                            // }
                            all_correct = false;
                            // try all combinations of gates, excluding those used in previous bits
                            let mut to_swap = Vec::new();
                            let mut next_combination = new_gates
                                .iter()
                                .filter(|&g| !do_not_touch.contains(&g.output))
                                .combinations(2)
                                .skip(tried);
                            if let Some(combi) = next_combination.next() {
                                gates = new_gates.clone();
                                for gate in combi {
                                    to_swap.push(gate.clone());
                                }
                                assert!(to_swap.len() == 2);
                                let temp_output = to_swap[0].output.clone();
                                gates.iter_mut().find(|g| **g == to_swap[0]).unwrap().output =
                                    to_swap[1].output.clone();
                                gates.iter_mut().find(|g| **g == to_swap[1]).unwrap().output =
                                    temp_output;
                                println!("Swapped gates: {:?}", to_swap);
                                tried += 1;
                                break 'inner;
                            } else {
                                panic!(
                                    "No more combinations for bit {}, x={}, y={}, carry={}",
                                    bit, x, y, carry
                                );
                            }
                        }
                    }
                }
            }
            if all_correct {
                println!("All correct for bit {} (needed swap: {})", bit, tried > 0);
                if tried > 0 {
                    let mut next_combination = new_gates
                        .iter()
                        .filter(|&g| !do_not_touch.contains(&g.output))
                        .combinations(2)
                        .skip(tried - 1); // had been increased by one
                    let combi = next_combination.next().unwrap();
                    println!("Swapped: {:?}", combi);
                    swapped.push(combi[0].output.clone());
                    swapped.push(combi[1].output.clone());
                    // do not swap correct gates for earlier bits
                    do_not_touch.extend(find_gates(&gates, &format!("z{bit:02}")));
                    // Also add the swapped gates to the do not touch list
                    do_not_touch.insert(combi[0].output.clone());
                    do_not_touch.insert(combi[1].output.clone());
                }
                new_gates = gates.clone();
                println!("  Do not touch: {}/{}", do_not_touch.len(), gates.len());
                // for do_not in sorted(do_not_touch.iter()) {
                //     println!("    {}", do_not);
                // }
                break 'outer;
            }
        }
    }
    swapped.sort();
    let swapped_string = swapped.iter().join(",");
    println!("Swapped: {}", swapped_string);
}
