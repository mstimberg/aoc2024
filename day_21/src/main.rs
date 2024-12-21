use std::fs;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum KeyPress {
    Up,
    Down,
    Left,
    Right,
    A,
}

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

fn keypad_pos(key: KeyPress) -> (usize, usize) {
    match key {
        KeyPress::Up => (0, 1),
        KeyPress::A => (0, 2),
        KeyPress::Down => (1, 1),
        KeyPress::Left => (1, 0),
        KeyPress::Right => (1, 2),
    }
}

fn move_and_press(pos: &mut (usize, usize), target: (usize, usize)) -> Vec<KeyPress> {
    // println!("Moving from {:?} to {:?}", pos, target);
    let mut key_presses = Vec::new();
    while *pos != target {
        if pos.1 < target.1 {
            pos.1 += 1;
            key_presses.push(KeyPress::Right);
        } else if pos.0 > target.0 {
            pos.0 -= 1;
            key_presses.push(KeyPress::Up);
        } else if pos.0 < target.0 {
            pos.0 += 1;
            key_presses.push(KeyPress::Down);
        } else if pos.1 > target.1 {
            pos.1 -= 1;
            key_presses.push(KeyPress::Left);
        }
        assert!((pos.0, pos.1) != (0, 0));
    }
    key_presses.push(KeyPress::A);
    key_presses
}

fn all_moves(
    start: (usize, usize),
    target: (usize, usize),
    forbidden: (usize, usize),
) -> Vec<Vec<KeyPress>> {
    let mut moves = Vec::new();
    if start == target {
        moves.push(vec![]);
        // No moves needed
    } else if start.0 == target.0 {
        if start.1 < target.1 {
            moves.push(vec![KeyPress::Right; target.1 - start.1]);
        } else if start.1 > target.1 {
            moves.push(vec![KeyPress::Left; start.1 - target.1]);
        }
    } else if start.1 == target.1 {
        if start.0 < target.0 {
            moves.push(vec![KeyPress::Down; target.0 - start.0]);
        } else if start.0 > target.0 {
            moves.push(vec![KeyPress::Up; start.0 - target.0]);
        }
    } else {
        // Try the two options
        if start.0 <= target.0 {
            let mut moves1 = vec![KeyPress::Down; target.0 - start.0];
            let mut moves2 = Vec::new();
            if start.1 < target.1 {
                moves1.extend(vec![KeyPress::Right; target.1 - start.1]);
                moves2.extend(vec![KeyPress::Right; target.1 - start.1]);
            } else if start.1 > target.1 {
                moves1.extend(vec![KeyPress::Left; start.1 - target.1]);
                moves2.extend(vec![KeyPress::Left; start.1 - target.1]);
            }
            moves2.extend(vec![KeyPress::Down; target.0 - start.0]);
            if !(target.0 == forbidden.0 && start.1 == forbidden.1) {
                moves.push(moves1);
            }
            if !(start.0 == forbidden.0 && target.1 == forbidden.1) {
                moves.push(moves2);
            }
        } else {
            let mut moves1 = vec![KeyPress::Up; start.0 - target.0];
            let mut moves2 = Vec::new();
            if start.1 < target.1 {
                moves1.extend(vec![KeyPress::Right; target.1 - start.1]);
                moves2.extend(vec![KeyPress::Right; target.1 - start.1]);
            } else if start.1 > target.1 {
                moves1.extend(vec![KeyPress::Left; start.1 - target.1]);
                moves2.extend(vec![KeyPress::Left; start.1 - target.1]);
            }
            moves2.extend(vec![KeyPress::Up; start.0 - target.0]);
            moves.push(moves1);
            if !(start.0 == forbidden.0 && target.1 == forbidden.1) {
                moves.push(moves2);
            }
        }
    }
    for m in moves.iter_mut() {
        m.push(KeyPress::A);
    }
    moves
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read the file");
    let codes = content.lines().collect::<Vec<_>>();
    let positions = codes.iter().map(|x| translate(x)).collect::<Vec<_>>();
    let mut total_complexity = 0;
    for (code, position) in codes.into_iter().zip(positions) {
        println!("Code        : {:?}", code);
        println!("Positions   : {:?}", position);
        let mut total_presses = 0;
        for (i, digit) in code.chars().enumerate() {
            let mut start;
            if i == 0 {
                start = (3, 2);
            } else {
                start = position[i - 1];
            }
            let moves = all_moves(start, position[i], (3, 0));
            println!("  First robot for {digit} : {:?}", moves);
            let mut moves2 = Vec::new();
            for (i, m) in moves.iter().enumerate() {
                let mut moves21: Vec<KeyPress> = Vec::new();
                let mut start = (0, 2);
                for key in m {
                    let target = keypad_pos(*key);
                    moves21.extend(move_and_press(&mut start, target));
                }
                moves2.push(moves21);
            }
            // let flat_moves = flatten(moves2);
            println!("    Second robot for {digit}: {:?}", moves2);
            let mut moves3 = Vec::new();
            for (i, m) in moves2.iter().enumerate() {
                let mut moves31: Vec<KeyPress> = Vec::new();
                let mut start = (0, 2);
                for key in m {
                    let target = keypad_pos(*key);
                    moves31.extend(move_and_press(&mut start, target));
                }
                moves3.push(moves31);
            }
            println!("    Me for {digit}          : {:?}", moves3);
            let shortest = moves3.iter().min_by_key(|x| x.len()).unwrap();
            println!("    Me for {digit}          : {:?}", shortest);
            total_presses += shortest.len();
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
