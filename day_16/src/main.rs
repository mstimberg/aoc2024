use ndarray::Array2;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn next_pos(pos: (usize, usize), facing: Direction) -> (usize, usize) {
    match facing {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
        Direction::East => (pos.0, pos.1 + 1),
    }
}

fn turn_left(facing: Direction) -> Direction {
    match facing {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::East => Direction::North,
    }
}

fn turn_right(facing: Direction) -> Direction {
    match facing {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
    }
}

fn find_dist_and_prev(
    map: &Array2<char>,
    start_pos: (usize, usize),
    facing: Direction,
    target_pos: (usize, usize),
) -> (
    HashMap<(usize, usize, Direction), usize>,
    HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>>,
) {
    // Dijkstra's algorithm
    // Create HashMap with distances for all nodes
    let mut distances = HashMap::new();
    let mut prev_nodes: HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>> =
        HashMap::new();
    for (i, j) in map
        .indexed_iter()
        .filter(|(_, &c)| c == '.' || c == 'E' || c == 'S')
        .map(|(idx, _)| idx)
    {
        distances.insert((i, j, Direction::North), usize::MAX - 1000);
        distances.insert((i, j, Direction::South), usize::MAX - 1000);
        distances.insert((i, j, Direction::West), usize::MAX - 1000);
        distances.insert((i, j, Direction::East), usize::MAX - 1000);
    }
    let keys: Vec<(usize, usize, Direction)> = distances.keys().cloned().collect();
    let mut unvisited: HashSet<(usize, usize, Direction)> = keys.into_iter().collect();
    distances.insert((start_pos.0, start_pos.1, facing), 0);
    let mut current = (start_pos.0, start_pos.1, facing);
    loop {
        println!("Remaining unvisited nodes: {:?}", unvisited.len());
        let dist_to_current = distances[&current];
        let forward_step = next_pos((current.0, current.1), current.2);
        if distances.contains_key(&(forward_step.0, forward_step.1, facing)) {
            let forward_dist = dist_to_current + 1;
            if forward_dist < distances[&(forward_step.0, forward_step.1, current.2)] {
                distances.insert((forward_step.0, forward_step.1, current.2), forward_dist);
                prev_nodes.insert((forward_step.0, forward_step.1, current.2), vec![current]);
            } else if forward_dist == distances[&(forward_step.0, forward_step.1, current.2)] {
                prev_nodes
                    .get_mut(&(forward_step.0, forward_step.1, current.2))
                    .unwrap()
                    .push(current);
            }
        }
        let left_turn = (current.0, current.1, turn_left(current.2));
        if distances.contains_key(&left_turn) {
            let left_dist = dist_to_current + 1000;
            if left_dist < distances[&left_turn] {
                distances.insert(left_turn, left_dist);
                prev_nodes.insert(left_turn, vec![current]);
            } else if left_dist == distances[&left_turn] {
                prev_nodes.get_mut(&left_turn).unwrap().push(current);
            }
        }
        let right_turn = (current.0, current.1, turn_right(current.2));
        let right_dist = dist_to_current + 1000;
        if right_dist < distances[&right_turn] {
            let right_dist = dist_to_current + 1000;
            if right_dist < distances[&right_turn] {
                distances.insert(right_turn, right_dist);
                prev_nodes.insert(right_turn, vec![current]);
            } else if right_dist == distances[&right_turn] {
                prev_nodes.get_mut(&right_turn).unwrap().push(current);
            }
        }
        unvisited.remove(&current);
        if unvisited.is_empty() {
            break;
        }
        // Find next node with smallest distance
        current = *unvisited.iter().min_by_key(|&pos| distances[pos]).unwrap();
    }

    (distances, prev_nodes)
}

fn _on_best_path(
    start_pos: (usize, usize),
    target_pos: (usize, usize, Direction),
    distances: &HashMap<(usize, usize, Direction), usize>,
    prev_nodes: &HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>>,
) -> HashSet<(usize, usize, Direction)> {
    let mut result = HashSet::new();
    let prev = prev_nodes[&target_pos].clone();
    for p in prev {
        result.insert(p);
        if p == (start_pos.0, start_pos.1, Direction::East) {            
            result.insert(target_pos);
        } else {
            let mut sub_result = _on_best_path(start_pos, p, &distances, &prev_nodes);
            result.insert(p);
            for sub in sub_result {
                result.insert(sub);
            }            
        }
    }
    result
}

fn on_best_path(
    start_pos: (usize, usize),
    target_pos: (usize, usize),
    distances: HashMap<(usize, usize, Direction), usize>,
    prev_nodes: HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>>,
    min_solution: usize,
) -> HashSet<(usize, usize, Direction)> {
    let mut paths =HashSet::new();
    for facing in &[
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ] {
        let solution = distances[&(target_pos.0, target_pos.1, *facing)];
        if solution == min_solution {
            paths.insert((target_pos.0, target_pos.1, *facing));
            for on_best in _on_best_path(
                start_pos,
                (target_pos.0, target_pos.1, *facing),
                &distances,
                &prev_nodes,
            ) {
                paths.insert(on_best);
            }
        }
    }
    paths
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
    let start_facing = Direction::East;
    let (distances, prevs) = find_dist_and_prev(&map, start_pos, start_facing, target_pos);
    let mut min_solution = usize::MAX;
    for facing in &[
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ] {
        min_solution = min(
            min_solution,
            distances[&(target_pos.0, target_pos.1, *facing)],
        );
    }
    println!("Min solution: {}", min_solution);
    let all_on_best = on_best_path(start_pos, target_pos, distances, prevs, min_solution);
    // Remove different facing directions
    let all_on_best: HashSet<(usize, usize)> = all_on_best
        .iter()
        .map(|(i, j, _)| (*i, *j))
        .collect();
    println!("Total number of nodes on best path: {}", all_on_best.len());
}
