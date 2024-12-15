use std::fs;
use std::io::Write;
use std::collections::HashSet;

fn update_positions(robots: &mut Vec<(i32, i32, i32, i32)>, width: i32, height: i32) {
    for robot in robots.iter_mut() {
        robot.0 += robot.2;
        if robot.0 < 0 {
            robot.0 += width as i32;
        } else if robot.0 >= width as i32 {
            robot.0 -= width as i32;
        }
        robot.1 += robot.3;
        if robot.1 < 0 {
            robot.1 += height as i32;
        } else if robot.1 >= height as i32 {
            robot.1 -= height as i32;
        }
    }
}

fn show_positions(robots: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32, filename: &str) {
    let mut map = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots.iter() {
        map[robot.1 as usize][robot.0 as usize] = 'x';
    }
    let mut file = fs::File::create(filename).expect("Should have been able to create the file");
    for row in map.iter() {
        // Write the row to the file
        file.write_all(row.iter().collect::<String>().as_bytes())
            .expect("Should have been able to write to the file");
        file.write(b"\n")
            .expect("Should have been able to write to the file");
    }
}

fn count_robots(robots: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32) -> Vec<usize> {
    let mut counts = Vec::new();
    for (min_x, max_x, min_y, max_y) in [
        (0, width / 2, 0, height / 2),
        (0, width / 2, height / 2 + 1, height),
        (width / 2 + 1, width, 0, height / 2),
        (width / 2 + 1, width, height / 2 + 1, height),
    ] {
        counts.push(
            robots
                .iter()
                .filter(|r| r.0 >= min_x && r.0 < max_x && r.1 >= min_y && r.1 < max_y)
                .count(),
        );
    }
    counts
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut robots = contents
        .lines()
        .map(|l| {
            let vec = l.split_whitespace().collect::<Vec<_>>();
            let position = vec[0][2..]
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let velocity = vec[1][2..]
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (position[0], position[1], velocity[0], velocity[1])
        })
        .collect::<Vec<(_, _, _, _)>>();
    let (width, height) = (101 as i32, 103 as i32);
    for iteration in 0..10000 {
        update_positions(&mut robots, width, height);
        let unique_robots = robots.iter().map(|(x, y, _, _)| (x, y)).collect::<HashSet<_>>();
        if unique_robots.len() == robots.len() {
            println!("Unique robots found after {} iterations", iteration+1);
            show_positions(&robots, width, height, format!("output_{:05}.txt", iteration+1).as_str());
            break;
        }
    }
    let counts = count_robots(&robots, 101, 103);
    println!("{:?}", counts);
    println!("Total count: {}", counts.iter().product::<usize>());
}
