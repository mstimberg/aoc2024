use ndarray::{Array, Array2, Axis};
use std::collections::HashSet;
use std::fs;

fn count_paths(map: &Array2<i32>, start: (usize, usize), positions: &mut HashSet<(usize, usize)>) {
    let digit = map[[start.0, start.1]];
    if digit == 9 {
        positions.insert(start);
    }
    for (row_diff, col_diff) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_row = start.0 as i32 + row_diff;
        let new_col = start.1 as i32 + col_diff;
        if new_row < 0
            || new_row >= map.shape()[0] as i32
            || new_col < 0
            || new_col >= map.shape()[1] as i32
        {
            continue;
        }
        if map[[new_row as usize, new_col as usize]] == digit + 1 {
            count_paths(&map, (new_row as usize, new_col as usize), positions);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let content_vec = contents
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.extend(line.chars());
            acc
        })
        .iter()
        .map(|&c| c.to_digit(10).unwrap_or(11) as i32)
        .collect::<Vec<_>>();
    // Convert the contents to a 2D array
    let size = (content_vec.len() as f32).sqrt() as usize;
    let map = Array::from_shape_vec((size, size), content_vec).unwrap();
    println!("Map:\n{:?}", map);
    let start_positions = map
        .indexed_iter()
        .filter(|(_, &x)| x == 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    println!("Start positions: {:?}", start_positions);
    let mut count = 0;
    for pos in start_positions {
        let mut positions = HashSet::new();
        count_paths(&map, pos, &mut positions);
        count += positions.len();
        println!("Positions: {:?}", positions);
    }
    println!("Count: {}", count);
}
