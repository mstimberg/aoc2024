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
) -> Array2<char> {
    let mut row = start_row;
    let mut col = start_col;
    let mut dir = start_dir;
    let mut new_map = map.clone();
    loop {
        // we will break out of the loop when we leave the map

        // To simplify the code afterwards, we turn the map so that we are always
        // moving horizontally to the right
        if dir == 'v' {
            // transpose the map
            new_map.swap_axes(0, 1);
            new_map.invert_axis(Axis(0));
            (row, col) = (new_map.shape()[0] - col - 1, row);
        } else if dir == '^' {
            // transpose the map
            new_map.invert_axis(Axis(0));
            new_map.swap_axes(0, 1);
            (row, col) = (col, new_map.shape()[0] - row - 1);
        }

        // Get the row in which we are moving
        let mut line = new_map.row_mut(row);

        // Find '#' to the right of the current position
        match line.slice(s![col..]).iter().position(|&x| x == '#') {
            Some(c) => {
                line.slice_mut(s![col..col + c]).fill('X');
                col += c - 1;
                dir = 'v'; // turn to the right
            }
            None => {
                // We will move out of the map
                line.slice_mut(s![col..]).fill('X');
                break;
            }
        }
    }
    new_map
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let content_vec = contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.extend(line.chars());
        acc
    });
    // Convert the contents to a 2D array
    let size = (content_vec.len() as f32).sqrt() as usize;
    let map = Array::from_shape_vec((size, size), content_vec).unwrap();
    // Extract the start position and direction
    let (row, colum, dir) = extract_start(&map);
    let new_map = fill_map(&map, row, colum, dir);
    let visited = new_map.iter().filter(|&c| *c == 'X').count();
    println!("visited: {}", visited);
}
