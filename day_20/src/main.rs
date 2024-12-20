use indicatif::ProgressIterator;
use itertools::Itertools;
use ndarray::Array2;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

fn h(pos: (usize, usize), goal: (usize, usize)) -> usize {
    // heuristic for distance
    let (x, y) = pos;
    let (x_goal, y_goal) = goal;
    max(x, x_goal) - min(x, x_goal) + max(y, y_goal) - min(y, y_goal)
}

fn find_cheats(
    map: &Array2<char>,
    start: (usize, usize),
    goal: (usize, usize),
    cutoff: usize,
) -> HashSet<((usize, usize), (usize, usize))> {
    let mut cheats = HashSet::new();
    for i in 1..map.dim().0 - 1 {
        for j in 1..map.dim().1 - 1 {
            if map[[i, j]] != '#' {
                // is it possible to pass though this point and be fast enough?
                if h(start, (i, j)) + h((i, j), goal) > cutoff {
                    continue;
                }
                // Start position for a cheat
                for goal_i in 0..=20 {
                    for goal_j in -20..=20 {
                        let goal_i = goal_i + i as i16;
                        let goal_j = goal_j + j as i16;
                        if goal_i < 0
                            || goal_i >= map.dim().0 as i16
                            || goal_j < 0
                            || goal_j >= map.dim().1 as i16
                            || (goal_i as usize, goal_j as usize) == (i, j)
                        {
                            continue;
                        }
                        if h((i, j), (goal_i as usize, goal_j as usize)) <= 20
                            && map[[goal_i as usize, goal_j as usize]] != '#'
                        {
                            cheats.insert(((i, j), (goal_i as usize, goal_j as usize)));
                            cheats.insert(((goal_i as usize, goal_j as usize), (i, j)));
                        }
                    }
                }
            }
        }
    }
    cheats
}

fn find_path(
    map: &Array2<char>,
    start: (usize, usize),
    goal: (usize, usize),
    cheat_start: Option<(usize, usize)>,
    cheat_end: Option<(usize, usize)>,
    known_scores: Option<&Array2<usize>>,
    max_length: Option<usize>,
) -> (Array2<usize>, Option<usize>) {
    // A* algorithm, largely copied from https://en.wikipedia.org/wiki/A*_search_algorithm
    let mut openSet = BTreeSet::new();

    openSet.insert((h(start, goal), start));
    let mut gScore = Array2::from_elem(map.dim(), std::usize::MAX);
    if known_scores.is_some() {
        gScore = known_scores.unwrap().clone() + 1;
    }
    gScore[start] = 0;
    loop {
        if openSet.is_empty() {
            break;
        }
        let (fScore, current) = openSet.pop_first().unwrap();
        if fScore > max_length.unwrap_or(std::usize::MAX) {
            // No need to look any further
            return (gScore, None);
        }
        if (current.0, current.1) == goal {
            let path_length = gScore[goal];
            return (gScore, Some(path_length));
        }
        if cheat_start.is_some() && current == cheat_start.unwrap() {
            let cheat_end = cheat_end.unwrap();
            let tentative_gScore = gScore[(current.0, current.1)] + h(current, cheat_end);
            if tentative_gScore < gScore[cheat_end] {
                gScore[cheat_end] = tentative_gScore;
                let fScore = gScore[cheat_end] + h(cheat_end, goal);
                openSet.insert((fScore, cheat_end));
            }
        }
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
                continue;
            }
            let tentative_gScore = gScore[(current.0, current.1)] + 1;
            if tentative_gScore < gScore[neighbour] {
                gScore[neighbour] = tentative_gScore;
                let fScore = gScore[neighbour] + h(neighbour, goal);
                openSet.insert((fScore, neighbour));
            }
        }
    }
    (gScore, None)
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
    println!("Start position: {:?}", start_pos);

    let (distances, best_path_length) =
        find_path(&map, start_pos, target_pos, None, None, None, None);
    let best_path_length = best_path_length.expect("No path found");
    let cutoff = best_path_length - 100;
    println!("Best path has length {}", best_path_length);
    // Horrible brute-force solution…
    let cheats = find_cheats(&map, start_pos, target_pos, cutoff);
    println!("Total different cheat paths to test: {}", cheats.len());
    let mut savings = HashMap::new();
    for (i, cheat) in cheats.iter().enumerate().progress() {
        if let (_, Some(cheat_path_length)) = find_path(
            &map,
            start_pos,
            target_pos,
            Some(cheat.0),
            Some(cheat.1),
            Some(&distances),
            Some(cutoff),
        ) {
            assert!(cheat_path_length <= cutoff);
            *savings
                .entry(best_path_length - cheat_path_length)
                .or_insert(0) += 1;
        }
    }

    // for key in savings.keys().sorted() {
    //     println!("Savings of {}: {}", key, savings[key]);
    // }
    println!(
        "Total different shortcurts: {}",
        savings.values().sum::<usize>()
    );
}
