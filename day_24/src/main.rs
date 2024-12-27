use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Logical {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Gate {
    i1: String,
    i2: String,
    op: Logical,
    output: String,
}

fn evaluate(gates: &Vec<Gate>, wires: &mut HashMap<String, bool>, gate: &Gate) {
    if !wires.clone().contains_key(&gate.i1) {
        for g in gates {
            if g.output == gate.i1 {
                evaluate(gates, wires, g);
                break;
            }
        }
    }
    if !wires.clone().contains_key(&gate.i2) {
        for g in gates {
            if g.output == gate.i2 {
                evaluate(gates, wires, g);
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
    // println!("Wires:");
    // for (key, value) in &wires {
    //     println!("{}: {}", key, value);
    // }

    for gate in &gates {
        if gate.output.starts_with("z") {
            evaluate(&gates, &mut wires, &gate);
        }
    }
    // println!("Final wires:");
    // for (key, value) in &wires {
    //     println!("{}: {}", key, value);
    // }
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
}
