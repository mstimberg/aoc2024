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

fn find_path(map: &Array2<usize>, goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
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
    let mut map = Array2::zeros((size, size));
    // There might be smarter ways, but we simply use bisect to find the first byte that cuts all paths
    let mut lower_bound = 1024_usize; // we know there is a path
    let mut upper_bound = coords.len();
    let mut current_guess = (upper_bound - lower_bound) / 2;
    loop {
        map.fill(0);
        fill_map(&coords, current_guess, &mut map);
        if find_path(&map, (size - 1, size - 1)).is_some() {
            lower_bound = current_guess;
        } else {
            upper_bound = current_guess;
            current_guess = upper_bound;
        }
        if upper_bound - lower_bound <= 1 {
            break;
        }
        current_guess = lower_bound + (upper_bound - lower_bound) / 2;
    }

    map.fill(0);
    fill_map(&coords, lower_bound, &mut map);
    let path = find_path(&map, (size - 1, size - 1)).unwrap();
    println!("Path: {:?}", path);
    println!("Coordinates: {:?}", coords[lower_bound]); // first coordinate not read in
}
