use std::cmp::{max, min};
use std::fs;

use ndarray::{s, Array2};

fn chest_coordinates(map: &Array2<char>) -> Vec<usize> {
    map.indexed_iter()
        .filter(|(_, &x)| x == '[')
        .map(|(idx, _)| idx.0 * 100 + idx.1)
        .collect()
}

fn chest_can_move(map: &Array2<char>, chest: (usize, usize), direction: char) -> bool {
    let (mut y, mut x) = (chest.0 as i32, chest.1 as i32);
    let row_diff = match direction {
        '^' => -1,
        'v' => 1,
        _ => panic!("Invalid direction"),
    };
    if map[((y + row_diff) as usize, x as usize)] == '.'
        && map[((y + row_diff) as usize, (x + 1) as usize)] == '.'
    {
        return true;
    } else if map[((y + row_diff) as usize, x as usize)] == '#'
        || map[((y + row_diff) as usize, (x + 1) as usize)] == '#'
    {
        // we are blocked
        return false;
    } else if map[((y + row_diff) as usize, x as usize)] == '[' {
        // we have a single chest to push
        return chest_can_move(
            map,
            ((chest.0 as i32 + row_diff) as usize, chest.1),
            direction,
        );
    } else if map[((y + row_diff) as usize, x as usize)] == ']'
        && map[((y + row_diff) as usize, (x + 1) as usize)] == '.'
    {
        // we have a single chest to push on the left
        return chest_can_move(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 - 1) as usize),
            direction,
        );
    } else if map[((y + row_diff) as usize, x as usize)] == '.'
        && map[((y + row_diff) as usize, (x + 1) as usize)] == '['
    {
        // we have a single chest to push on the right
        return chest_can_move(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 + 1) as usize),
            direction,
        );
    } else {
        // We have two chestes to push
        return chest_can_move(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 - 1) as usize),
            direction,
        ) && chest_can_move(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 + 1) as usize),
            direction,
        );
    }
}

fn push_chest_up_down(map: &mut Array2<char>, chest: (usize, usize), direction: char) -> bool {
    let (mut y, mut x) = (chest.0 as i32, chest.1 as i32);
    let row_diff = match direction {
        '^' => -1,
        'v' => 1,
        _ => panic!("Invalid direction"),
    };
    if map[((y + row_diff) as usize, x as usize)] == '.'
        && map[((y + row_diff) as usize, (x + 1) as usize)] == '.'
    {
        // we can move the chest already
        map[(chest.0, chest.1)] = '.';
        map[(chest.0, chest.1 + 1)] = '.';
        map[((chest.0 as i32 + row_diff) as usize, chest.1)] = '[';
        map[((chest.0 as i32 + row_diff) as usize, chest.1 + 1)] = ']';
        return true;
    } else if map[((y + row_diff) as usize, x as usize)] == '#'
        || map[((y + row_diff) as usize, (x + 1) as usize)] == '#'
    {
        // we are blocked
        return false;
    } else if map[((y + row_diff) as usize, x as usize)] == '[' {
        // we have a single chest to push
        if push_chest_up_down(
            map,
            ((chest.0 as i32 + row_diff) as usize, chest.1),
            direction,
        ) {
            map[(chest.0, chest.1)] = '.';
            map[(chest.0, chest.1 + 1)] = '.';
            map[((chest.0 as i32 + row_diff) as usize, chest.1)] = '[';
            map[((chest.0 as i32 + row_diff) as usize, chest.1 + 1)] = ']';
            return true;
        } else {
            return false;
        }
    } else if map[((y + row_diff) as usize, x as usize)] == ']'
        && map[((y + row_diff) as usize, (x + 1) as usize)] == '.'
    {
        // we have a single chest to push on the left
        if push_chest_up_down(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 - 1) as usize),
            direction,
        ) {
            map[(chest.0, chest.1)] = '.';
            map[(chest.0, chest.1 + 1)] = '.';
            map[((chest.0 as i32 + row_diff) as usize, chest.1)] = '[';
            map[((chest.0 as i32 + row_diff) as usize, chest.1 + 1)] = ']';
            return true;
        } else {
            return false;
        }
    } else if map[((y + row_diff) as usize, x as usize)] == '.'
        && map[((y + row_diff) as usize, (x + 1) as usize)] == '['
    {
        // we have a single chest to push on the right
        if push_chest_up_down(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 + 1) as usize),
            direction,
        ) {
            map[(chest.0, chest.1)] = '.';
            map[(chest.0, chest.1 + 1)] = '.';
            map[((chest.0 as i32 + row_diff) as usize, chest.1)] = '[';
            map[((chest.0 as i32 + row_diff) as usize, chest.1 + 1)] = ']';
            return true;
        } else {
            return false;
        }
    } else {
        // We have two chestes to push
        if push_chest_up_down(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 - 1) as usize),
            direction,
        ) && push_chest_up_down(
            map,
            ((chest.0 as i32 + row_diff) as usize, (chest.1 + 1) as usize),
            direction,
        ) {
            map[(chest.0, chest.1)] = '.';
            map[(chest.0, chest.1 + 1)] = '.';
            map[((chest.0 as i32 + row_diff) as usize, chest.1)] = '[';
            map[((chest.0 as i32 + row_diff) as usize, chest.1 + 1)] = ']';
            return true;
        } else {
            return false;
        }
    }
}

fn move_robot_up_down(
    map: &mut Array2<char>,
    robot: (usize, usize),
    direction: char,
) -> (usize, usize) {
    let (y, x) = (robot.0 as i32, robot.1 as i32);
    let row_diff = match direction {
        '^' => -1,
        'v' => 1,
        _ => panic!("Invalid direction"),
    };
    if map[((y + row_diff) as usize, x as usize)] == '#' {
        return robot;
    }
    if map[((y + row_diff) as usize, x as usize)] == '[' {
        if chest_can_move(map, ((y + row_diff) as usize, x as usize), direction) {
            push_chest_up_down(map, ((y + row_diff) as usize, x as usize), direction);
        } else {
            return robot;
        }
    } else if map[((y + row_diff) as usize, x as usize)] == ']' {
        if chest_can_move(map, ((y + row_diff) as usize, x as usize - 1), direction) {
            push_chest_up_down(map, ((y + row_diff) as usize, (x - 1) as usize), direction);
        } else {
            return robot;
        }
    }
    map[(robot.0, robot.1)] = '.';
    let new_robot = ((robot.0 as i32 + row_diff) as usize, robot.1);
    map[new_robot] = '@';
    new_robot
}

fn move_robot_left_right(
    map: &mut Array2<char>,
    robot: (usize, usize),
    direction: char,
) -> (usize, usize) {
    let (mut y, mut x) = (robot.0 as i32, robot.1 as i32);
    let (move_y, move_x) = match direction {
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Invalid direction"),
    };
    // First check whether there are any spaces between the position and the wall
    let mut has_space = false;
    while map[((y + move_y) as usize, (x + move_x) as usize)] != '#' {
        y += move_y;
        x += move_x;
        if map[(y as usize, x as usize)] == '.' {
            has_space = true;
            break;
        }
    }
    if !has_space {
        return robot;
    }
    x = robot.1 as i32;
    y = robot.0 as i32;

    // Move chest(es) if there is one
    if map[((y + move_y) as usize, (x + move_x) as usize)] == '['
        || map[((y + move_y) as usize, (x + move_x) as usize)] == ']'
    {
        let mut left_part = map[((y + move_y) as usize, (x + move_x) as usize)] == '[';
        y += move_y;
        x += move_x;
        map[(y as usize, x as usize)] = '.';
        while map[((y + move_y) as usize, (x + move_x) as usize)] != '#' {
            y += move_y;
            x += move_x;
            if map[(y as usize, x as usize)] == '[' || map[(y as usize, x as usize)] == ']' {
                if left_part {
                    map[(y as usize, x as usize)] = '[';
                } else {
                    map[(y as usize, x as usize)] = ']';
                }
                left_part = !left_part;
            }
            if map[(y as usize, x as usize)] == '.' {
                if left_part {
                    map[(y as usize, x as usize)] = '[';
                } else {
                    map[(y as usize, x as usize)] = ']';
                }
                break; // later chestes won't be moved
            }
        }
    }
    map[(robot.0, robot.1)] = '.';
    let new_robot = (
        (robot.0 as i32 + move_y) as usize,
        (robot.1 as i32 + move_x) as usize,
    );
    map[new_robot] = '@';
    new_robot
}

fn move_robot(map: &mut Array2<char>, robot: (usize, usize), direction: char) -> (usize, usize) {
    if direction == '<' || direction == '>' {
        return move_robot_left_right(map, robot, direction);
    } else {
        return move_robot_up_down(map, robot, direction);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut map = Vec::new();
    let mut moves = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut first_part = true;
    for line in contents.lines() {
        if line.is_empty() || line[0..1] != *"#" {
            first_part = false;
        }
        if first_part {
            width = line.len() * 2;
            for c in line.chars() {
                match c {
                    '#' => {
                        map.push('#');
                        map.push('#');
                    }
                    '@' => {
                        map.push('@');
                        map.push('.');
                    }
                    '.' => {
                        map.push('.');
                        map.push('.');
                    }
                    'O' => {
                        map.push('[');
                        map.push(']');
                    }
                    _ => panic!("Invalid character"),
                }
            }
            height += 1;
        } else {
            moves.extend(line.chars());
        }
    }
    let robot = map.iter().position(|&x| x == '@').unwrap();
    let mut map = Array2::from_shape_vec([height, width], map).unwrap();
    let (mut robot_y, mut robot_x) = (robot / width, robot % width);
    println!("Width: {}, Height: {}", width, height);
    println!("Map:\n {:?}", map);
    println!("Robot: {}, {}", robot_y, robot_x);
    println!("Moves: {:?}", moves);

    for one_move in moves {
        (robot_y, robot_x) = move_robot(&mut map, (robot_y, robot_x), one_move);
        // println!("Robot: {}, {}", robot_y, robot_x);
        // println!("Map:\n {:?}", map);
    }
    println!("Final Map:\n {:?}", map);
    println!("chestes: {:?}", chest_coordinates(&map));
    println!(
        "chest sum: {}",
        chest_coordinates(&map).iter().sum::<usize>()
    );
}
