use cached::proc_macro::cached;
use lazy_static::lazy_static;
use std::{collections::HashMap, fs, vec};

fn translate(code: &str) -> Vec<(usize, usize)> {
    let mut translated = Vec::new();
    for c in code.chars() {
        match c {
            '7' => translated.push((0, 0)),
            '8' => translated.push((0, 1)),
            '9' => translated.push((0, 2)),
            '4' => translated.push((1, 0)),
            '5' => translated.push((1, 1)),
            '6' => translated.push((1, 2)),
            '1' => translated.push((2, 0)),
            '2' => translated.push((2, 1)),
            '3' => translated.push((2, 2)),
            '0' => translated.push((3, 1)),
            'A' => translated.push((3, 2)),
            _ => panic!("Invalid code"),
        }
    }
    translated
}

fn keypad_pos(key: char) -> (usize, usize) {
    match key {
        '^' => (0, 1),
        'A' => (0, 2),
        'v' => (1, 1),
        '<' => (1, 0),
        '>' => (1, 2),
        _ => panic!("Invalid key"),
    }
}

lazy_static! {
    // This seemed to be easier to hard-code, but in the end it was not...
    static ref MOVE_TRANSLATION: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("A", vec!["A"]);
        m.insert("<A", vec!["v<<A", ">>^A"]);
        m.insert("vA", vec!["<vA", "^>A"]);
        m.insert(">A", vec!["vA", "^A"]);
        m.insert("^A", vec!["<A", ">A"]);
        m.insert("<<A", vec!["v<<A", "A", ">>^A"]);
        m.insert(">>A", vec!["vA", "A", "^A"]);
        m.insert("<vA", vec!["v<<A", ">A", "^>A"]);
        m.insert("<^A", vec!["v<<A", ">^A", ">A"]);
        m.insert(">vA", vec!["vA", "<A", "^>A"]);
        m.insert(">^A", vec!["vA", "<^A", ">A"]);
        m.insert("v<A", vec!["<vA", "<A", ">>^A"]);
        m.insert("v>A", vec!["<vA", ">A", "^A"]);
        m.insert("^<A", vec!["<A", "v<A", ">>^A"]);
        m.insert("^>A", vec!["<A", "v>A", "^A"]);
        m.insert("^^A", vec!["<A", "A", ">A"]);
        m.insert("^^>A", vec!["<A", "A", "v>A", "^A"]);
        m.insert("^^<A", vec!["<A", "A", "v<A", ">>^A"]);
        m.insert("<^^A", vec!["v<<A", ">^A", "A", ">A"]);
        m.insert("<<^A", vec!["v<<A", "A", ">^A", ">A"]);
        m.insert("vvA", vec!["<vA", "A", "^>A"]);
        m.insert(">>^A", vec!["vA", "A", "<^A", ">A"]);
        m.insert(">^^A", vec!["vA", "<^A", "A", ">A"]);
        m.insert(">>vA", vec!["vA", "A", "<A", "^>A"]);
        m.insert("v<<A", vec!["<vA", "<A", "A", ">>^A"]);
        m.insert("vv>A", vec!["<vA", "A", ">A", "^A"]);
        m.insert("^<<A", vec!["<A", "v<A", "A", ">>^A"]);
        m.insert(">vvA", vec!["vA", "<A", "A", "^>A"]);
        m.insert("^>>A", vec!["<A", "v>A", "A", "^A"]);
        m.insert("^^^A", vec!["<A", "A", "A", ">A"]);
        m.insert("vvvA", vec!["<vA", "A", "A", "^>A"]);
        m.insert("vvv<A", vec!["<vA", "A", "A", "<A", ">>^A"]);
        m.insert("vvv>A", vec!["<vA", "A", "A", ">A", "^A"]);
        m.insert("^^^<A", vec!["<A", "A", "A", "v<A", ">>^A"]);
        m.insert("^^^>A", vec!["<A", "A", "A", "v>A", "^A"]);
        m.insert("<vvvA", vec!["v<<A", ">A", "A", "A", "^>A"]);
        m.insert(">vvvA", vec!["vA", "<A", "A", "A", "^>A"]);
        m.insert("<^^^A", vec!["v<<A", ">^A", "A", "A", ">A"]);
        m.insert(">^^^A", vec!["vA", "<^A", "A", "A", ">A"]);
        m.insert("^^<<A", vec!["<A", "A", "v<A", "A", ">>^A"]);
        m.insert("<<^^A", vec!["v<<A", "A", ">^A", "A", ">A"]);
        m
    };
}

#[cached]
fn shortest_sequence(moves: String, robots: usize) -> usize {
    if robots == 0 {
        return moves.len();
    }
    let mut presses = 0;
    let translations = MOVE_TRANSLATION.get(moves.as_str()).unwrap();
    for translation in translations {
        presses += shortest_sequence(translation.to_string(), robots - 1);
    }
    presses
}

fn all_moves(
    start: (usize, usize),
    target: (usize, usize),
    forbidden: (usize, usize),
) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();
    assert!(start != forbidden);
    if start == target {
        moves.push("".to_string());
        // No moves needed
    } else if start.0 == target.0 && target.0 != forbidden.0 {
        if start.1 < target.1 {
            moves.push(">".to_string().repeat(target.1 - start.1));
        } else if start.1 > target.1 {
            moves.push("<".to_string().repeat(start.1 - target.1));
        }
    } else if start.1 == target.1 && target.1 != forbidden.1 {
        if start.0 < target.0 {
            moves.push("v".to_string().repeat(target.0 - start.0));
        } else if start.0 > target.0 {
            moves.push("^".to_string().repeat(start.0 - target.0));
        }
    } else {
        // Try the two options
        if start.0 <= target.0 {
            let mut moves1 = "v".to_string().repeat(target.0 - start.0);
            let mut moves2 = String::new();
            if start.1 < target.1 {
                moves1.push_str(&">".to_string().repeat(target.1 - start.1));
                moves2.push_str(&">".to_string().repeat(target.1 - start.1));
            } else if start.1 > target.1 {
                moves1.push_str(&"<".to_string().repeat(start.1 - target.1));
                moves2.push_str(&"<".to_string().repeat(start.1 - target.1));
            }
            moves2.push_str(&"v".to_string().repeat(target.0 - start.0));
            if !(target.0 == forbidden.0 && start.1 == forbidden.1) {
                moves.push(moves1);
            }
            if !(start.0 == forbidden.0 && target.1 == forbidden.1) {
                moves.push(moves2);
            }
        } else {
            let mut moves1 = "^".to_string().repeat(start.0 - target.0);
            let mut moves2 = String::new();
            if start.1 < target.1 {
                moves1.push_str(&">".to_string().repeat(target.1 - start.1));
                moves2.push_str(&">".to_string().repeat(target.1 - start.1));
            } else if start.1 > target.1 {
                moves1.push_str(&"<".to_string().repeat(start.1 - target.1));
                moves2.push_str(&"<".to_string().repeat(start.1 - target.1));
            }
            moves2.push_str(&"^".to_string().repeat(start.0 - target.0));
            if !(target.0 == forbidden.0 && start.1 == forbidden.1) {
                moves.push(moves1);
            }
            if !(start.0 == forbidden.0 && target.1 == forbidden.1) {
                moves.push(moves2);
            }
        }
    }
    for m in moves.iter_mut() {
        m.push_str("A");
    }
    moves
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read the file");
    let codes = content.lines().collect::<Vec<_>>();
    let positions = codes.iter().map(|x| translate(x)).collect::<Vec<_>>();
    // Verify the translations
    for (target, moves) in MOVE_TRANSLATION.iter() {
        let mut pos = keypad_pos('A');
        let mut result = String::new();
        for m in moves.iter() {
            for c in m.chars() {
                if c == 'A' {
                    break;
                }
                pos = match c {
                    '^' => (pos.0 - 1, pos.1),
                    'v' => (pos.0 + 1, pos.1),
                    '<' => (pos.0, pos.1 - 1),
                    '>' => (pos.0, pos.1 + 1),
                    _ => panic!("Invalid move"),
                };
                if pos == (0, 0) {
                    panic!(
                        "Invalid move to forbidden position: {:?} (moves: {:?}), target: {:?}",
                        m, moves, target
                    );
                }
            }
            let pressed = match pos {
                (0, 1) => "^",
                (0, 2) => "A",
                (1, 1) => "v",
                (1, 0) => "<",
                (1, 2) => ">",
                _ => panic!("Invalid position {:?} in moves for {}", pos, target),
            };
            result.push_str(pressed);
        }
        if result != *target {
            panic!("Translation failed for {} -> {}", target, result);
        }
    }

    let mut total_complexity = 0;
    for (code, position) in codes.into_iter().zip(positions) {
        println!("Code        : {:?}", code);
        println!("Positions   : {:?}", position);
        let mut total_presses = 0;
        // let mut memo = HashMap::new();
        for (i, digit) in code.chars().enumerate() {
            let start = if i == 0 { (3, 2) } else { position[i - 1] };
            let moves = all_moves(start, position[i], (3, 0));
            let moves_as_str = moves.iter().map(|x| x.as_str()).collect::<Vec<_>>();
            let mut possible_moves = Vec::new();
            for m in moves_as_str {
                let presses = shortest_sequence(m.to_string(), 25);
                possible_moves.push(presses);
            }
            println!();
            let shortest = possible_moves.iter().min().unwrap();
            println!("    Shortest presses for {digit} : {}", shortest);
            total_presses += shortest;
        }
        println!("Total presses: {}", total_presses);
        let numerical = code[0..3].parse::<usize>().unwrap();
        println!("Numerical part of code: {}", numerical);
        println!("Complexity of the code: {}", total_presses * numerical);
        total_complexity += total_presses * numerical;
        println!();
    }
    println!("Total complexity: {}", total_complexity);
}
