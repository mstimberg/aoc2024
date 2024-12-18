use core::panic;
use ndarray::Array2;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

fn h(pos: (usize, usize), goal: (usize, usize)) -> usize {
    // heuristic for distance
    let (x, y) = pos;
    let (x_goal, y_goal) = goal;
    // very generous heuristic
    max(
        (x_goal as i16 - x as i16).abs(),
        (y_goal as i16 - y as i16).abs(),
    )
    .pow(2) as usize
}

fn find_path(map: &Array2<usize>, goal: (usize, usize)) -> Vec<(usize, usize)> {
    // A* algorithm, largely copied from https://en.wikipedia.org/wiki/A*_search_algorithm
    let mut openSet = HashSet::new();
    let mut cameFrom = HashMap::new();
    openSet.insert((0, 0));
    let mut gScore = Array2::from_elem(map.dim(), std::usize::MAX);
    gScore[[0, 0]] = 0;
    let mut fScore = Array2::from_elem(map.dim(), std::usize::MAX);
    fScore[[0, 0]] = h((0, 0), goal);
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
            return path;
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
            if map[neighbour] == 1 {
                // corrupted
                continue;
            }
            let tentative_gScore = gScore[(current.0, current.1)] + 1;
            if tentative_gScore < gScore[neighbour] {
                cameFrom.insert(neighbour, current);
                gScore[neighbour] = tentative_gScore;
                fScore[neighbour] = gScore[neighbour] + h(neighbour, goal);
                openSet.insert(neighbour);
            }
        }
    }
    panic!("No path found");
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let coords = contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut parts = x.split(",");
            let x = parts.next().unwrap().parse::<i16>().unwrap();
            let y = parts.next().unwrap().parse::<i16>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();
    let size = 71;
    let bytes = 1024;
    let mut map = Array2::zeros((size, size));
    for coord in &coords[..bytes] {
        let (x, y) = coord;
        map[[*x as usize, *y as usize]] = 1;
    }
    println!("{:?}", map);
    let path: Vec<(usize, usize)> = find_path(&map, (size - 1, size - 1));
    println!("Path: {:?}", path);
    for (x, y) in path.iter() {
        map[[*x, *y]] = 8;
    }
    println!("{:?}", map.t());
    println!("Steps: {}", path.len() - 1);
}
