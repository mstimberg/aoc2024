use ndarray::{s, Array, Array2, Axis};
use std::fs;

fn extract_start(map: &Array2<char>) -> (usize, usize, char) {
    for (row, line) in map.outer_iter().enumerate() {
        for (colum, &cell) in line.iter().enumerate() {
            if cell == 'v' || cell == '^' || cell == '<' || cell == '>' {
                return (row, colum, cell);
            }
        }
    }
    panic!("No start position found");
}

fn fill_map(
    map: &Array2<char>,
    start_row: usize,
    start_col: usize,
    start_dir: char,
) -> (Array2<char>, bool) {
    let mut row = start_row;
    let mut col = start_col;
    let mut dir = start_dir;
    let mut new_map = map.clone();
    let mut has_loop = false;
    let mut turn_marker = 'T';
    let mut rotations = 0;
    loop {
        // we will break out of the loop when we leave the map

        // To simplify the code afterwards, we turn the map so that we are always
        // moving horizontally to the right
        if dir == 'v' {
            // transpose the map
            new_map.swap_axes(0, 1);
            new_map.invert_axis(Axis(0));
            (row, col) = (new_map.shape()[0] - col - 1, row);
            rotations -= 1;
        } else if dir == '^' {
            // transpose the map
            new_map.invert_axis(Axis(0));
            new_map.swap_axes(0, 1);
            (row, col) = (col, new_map.shape()[0] - row - 1);
            rotations += 1;
        }
        if turn_marker == 'T' {
            turn_marker = 't';
        } else {
            turn_marker = 'T';
        }
        // Get the row in which we are moving
        let mut line = new_map.row_mut(row);

        // Find '#' to the right of the current position
        match line
            .slice(s![col..])
            .iter()
            .position(|&x| x == '#' || x == 'O')
        {
            Some(c) => {
                if line[col + c - 1] == 'T' && c > 1 {
                    // ignore if we are turning on the spot
                    // We have found a loop
                    has_loop = true;
                }
                line.slice_mut(s![col + 1..col + c]).fill('X');
                line[col + c - 1] = 'T'; // mark the turn
                col += c - 1;
                dir = 'v'; // turn to the right
            }
            None => {
                // We will move out of the map
                line.slice_mut(s![col..]).fill('X');
                break;
            }
        }
        if has_loop {
            break;
        }
    }
    // Turn the map back to the original orientation
    if rotations > 0 {
        for _ in 0..rotations {
            new_map.swap_axes(0, 1);
            new_map.invert_axis(Axis(0));
        }
    } else if rotations < 0 {
        for _ in 0..-rotations {
            new_map.invert_axis(Axis(0));
            new_map.swap_axes(0, 1);
        }
    }
    (new_map, has_loop)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let content_vec = contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.extend(line.chars());
        acc
    });
    // Convert the contents to a 2D array
    let size = (content_vec.len() as f32).sqrt() as usize;
    let start_index = content_vec
        .iter()
        .position(|&c| c == 'v' || c == '^' || c == '<' || c == '>')
        .unwrap();
    let map = Array::from_shape_vec((size, size), content_vec).unwrap();
    // Extract the start position and direction
    let (row, colum, dir) = extract_start(&map);
    let (new_map, _) = fill_map(&map, row, colum, dir);
    // println!("Visited map:\n{}", new_map);
    let visited = new_map
        .iter()
        .enumerate()
        .filter(|(_i, &c)| c != '#' && c != '.')
        .map(|(i, _c)| i)
        .collect::<Vec<_>>();
    // Try all variants of the map with an added obstacle on the path
    let mut count = 0;
    for (index, visited_index) in visited.into_iter().enumerate() {
        if visited_index == start_index {
            continue; // we cannot add an obstacle at the start position
        }
        let mut new_content = map.iter().cloned().collect::<Vec<_>>();
        new_content[visited_index] = 'O';
        let map = Array::from_shape_vec((size, size), new_content).unwrap();
        let (filled_map, has_loop) = fill_map(&map, row, colum, dir);
        if has_loop {
            // println!("The map with an obstacle at index {} has a loop!", visited_index);
            // println!("Start map:\n{}", map);
            // println!("Visited map:\n{}", filled_map);
            count += 1;
        }
    }
    println!("Number of maps with loops: {}", count);
}
