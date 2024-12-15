use std::fs;

use ndarray::Array2;

fn box_coordinates(map: &Array2<char>) -> Vec<usize> {
    map.indexed_iter().filter(|(_, &x)| x == 'O').map(|(idx, _)| idx.0*100 + idx.1).collect()
}

fn move_robot(map: &mut Array2<char>, robot: (usize, usize), direction: char) -> (usize, usize) {
    let (mut y, mut x) = (robot.0 as i32, robot.1 as i32);
    let (move_y, move_x) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
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
        }
    }
    if !has_space {
        return robot;
    }
    x = robot.1 as i32;
    y = robot.0 as i32;

    // Move box(es) if there is one
    if map[((y + move_y) as usize, (x + move_x) as usize)] == 'O' {
        y += move_y;
        x += move_x;
        map[(y as usize, x as usize)] = '.';
        while map[((y + move_y) as usize, (x + move_x) as usize)] != '#' {
            y += move_y;
            x += move_x;
            if map[(y as usize, x as usize)] == '.' {
                map[(y as usize, x as usize)] = 'O';
                break;  // later boxes won't be moved
            }
        }
    }
    map[(robot.0, robot.1)] = '.';
    let new_robot = ((robot.0 as i32 + move_y) as usize, (robot.1 as i32 + move_x) as usize);
    map[new_robot] = '@';
    new_robot
    
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
            width = line.len();
            map.extend(line.chars());
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
        println!("Robot: {}, {}", robot_y, robot_x);
        println!("Map:\n {:?}", map);
    }
    println!("Boxes: {:?}", box_coordinates(&map));
    println!("Box sum: {}", box_coordinates(&map).iter().sum::<usize>());
}
