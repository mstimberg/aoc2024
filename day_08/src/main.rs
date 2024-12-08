use itertools::Itertools;
use ndarray::{Array, Array2};
use std::collections::HashSet;
use std::fs;

fn find_antinodes(c: char, map: &Array2<char>) -> Vec<(i32, i32)> {
    // convert flat positions into 2D positions
    let size = map.shape()[0];
    let positions: Vec<(i32, i32)> = map
        .indexed_iter()
        .filter(|(_, &x)| x == c)
        .map(|(i, _)| (i.0 as i32, i.1 as i32))
        .collect();
    let mut antinodes = Vec::new();
    for p in positions.into_iter().combinations(2) {
        let p1 = p[0];
        let p2 = p[1];
        let dist = (p1.0 - p2.0, p1.1 - p2.1);
        antinodes.push((p1.0 + dist.0, p1.1 + dist.1));
        antinodes.push((p2.0 - dist.0, p2.1 - dist.1));
    }
    antinodes
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let content_vec = contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.extend(line.chars());
        acc
    });
    // Get the unique characters
    let unique_chars: HashSet<char> = content_vec.iter().cloned().collect();
    println!("{:?}", unique_chars);
    // Convert the contents to a 2D array
    let size = (content_vec.len() as f32).sqrt() as usize;
    let mut map = Array::from_shape_vec((size, size), content_vec).unwrap();
    let antinodes = unique_chars
        .iter()
        .filter(|&c| *c != '.')
        .map(|&c| find_antinodes(c, &map))
        .flatten()
        .filter(|a| a.0 >= 0 && a.0 < size as i32 && a.1 >= 0 && a.1 < size as i32)
        .collect::<HashSet<(i32, i32)>>();
    for a in &antinodes {
        map[[a.0 as usize, a.1 as usize]] = '#';
    }
    println!("Antinodes:\n{:?}", antinodes.len());
    println!("Final map:\n{:?}", map);
}
