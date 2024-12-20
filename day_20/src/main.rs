use core::panic;
use ndarray::{Array2};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;

fn h(map: &Array2<char>, pos: (usize, usize), goal: (usize, usize)) -> usize {
    // heuristic for distance
    let (x, y) = pos;
    let (x_goal, y_goal) = goal;
    max(x, x_goal) - min(x, x_goal) + max(y, y_goal) - min(y, y_goal)
}

fn find_path(map: &Array2<char>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    // A* algorithm, largely copied from https://en.wikipedia.org/wiki/A*_search_algorithm
    let mut openSet = HashSet::new();
    let mut cameFrom = HashMap::new();

    openSet.insert(start);
    let mut gScore = Array2::from_elem(map.dim(), std::usize::MAX);
    gScore[start] = 0;
    let mut fScore = Array2::from_elem(map.dim(), std::usize::MAX);
    fScore[start] = h(&map, (0, 0), goal);
    loop {
        if openSet.is_empty() {
            break;
        }
        // not optimal, should use a priority queue (also not sure about all the cloning...)
        let current = openSet
            .clone()
            .into_iter()
            .min_by_key(|&x| fScore[(x.0, x.1)])
            .unwrap();
        if (current.0, current.1) == goal {
            let mut current = goal;
            let mut path: Vec<(usize, usize)> = vec![current];
            while let Some(&next) = cameFrom.get(&current) {
                path.push(next);
                current = next;
            }
            return Some(path);
        }
        openSet.remove(&current);
        for neighbour in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let neighbour = (
                current.0 as i16 + neighbour.0 as i16,
                current.1 as i16 + neighbour.1 as i16,
            );
            if neighbour.0 < 0
                || neighbour.0 >= map.dim().0 as i16
                || neighbour.1 < 0
                || neighbour.1 >= map.dim().1 as i16
            {
                continue;
            }
            let neighbour = (neighbour.0 as usize, neighbour.1 as usize);
            if map[neighbour] == '#' {
                // Wall
                continue;
            }
            let tentative_gScore = gScore[(current.0, current.1)] + 1;
            if tentative_gScore < gScore[neighbour] {
                cameFrom.insert(neighbour, current);
                gScore[neighbour] = tentative_gScore;
                fScore[neighbour] = gScore[neighbour] + h(&map, neighbour, goal);
                openSet.insert(neighbour);
            }
        }
    }
    None
}

fn fill_map(coords: &Vec<(i16, i16)>, bytes: usize, map: &mut Array2<usize>) {
    for coord in &coords[..bytes] {
        let (x, y) = coord;
        map[[*x as usize, *y as usize]] = 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let map = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let map = Array2::from_shape_fn((map.len(), map[0].len()), |(i, j)| map[i][j]);
    println!("Map:\n{}", map);
    let start_pos = map
        .indexed_iter()
        .find(|(_, &c)| c == 'S')
        .map(|(idx, _)| idx)
        .expect("No starting position found");
    let target_pos = map
        .indexed_iter()
        .find(|(_, &c)| c == 'E')
        .map(|(idx, _)| idx)
        .expect("No starting position found");
    let best_path = find_path(&map, start_pos, target_pos).expect("No path found");
    let best_path_length = best_path.len();
    println!("Best path has length {}", best_path_length);
    // Brute force... remove each wall and see if we can find a path
    let mut cheat_count = HashMap::new();
    for i in 0..map.dim().0 {
        for j in 0..map.dim().1 {
            if map[[i, j]] == '#' {                
                let mut new_map = map.clone();
                new_map[[i, j]] = '.';
                if let Some(path) = find_path(&new_map, start_pos, target_pos) {
                    let new_path_len = path.len();
                    if new_path_len <= best_path_length - 100 {
                        // println!("Removing wall at ({}, {}) gives a path of length {}", i, j, new_path_len);
                        let saves = best_path_length - new_path_len;
                        *cheat_count.entry(saves).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    for (saves, count) in &cheat_count {
        println!("There are {count} cheats that save {saves} picoseconds");
    }
    println!("Total different cheats: {}", cheat_count.values().sum::<usize>());
}
