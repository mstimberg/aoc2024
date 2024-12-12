use disjoint::DisjointSet;
use ndarray::{s, Array, Array2, Axis};
use std::cmp;
use std::collections::HashSet;
use std::fs;

fn expand(map: &Array2<u32>) -> Vec<Array2<bool>> {
    let unique_numbers = map.iter().collect::<HashSet<_>>();
    let mut maps = Vec::new();
    for &num in unique_numbers.iter() {
        maps.push(Array2::from_shape_fn(
            (map.shape()[0], map.shape()[1]),
            |idx| map[idx] == *num,
        ));
    }
    maps
}

fn label(map: &Array2<bool>) -> Array2<u32> {
    let mut label_map = Array2::from_elem([map.shape()[0], map.shape()[1]], 0);
    let mut label = 0;
    let mut equivalences = DisjointSet::with_len(map.shape()[0] * map.shape()[1]);
    let rows = map.shape()[0];
    let cols = map.shape()[1];
    // first pass
    for i in 0..rows {
        for j in 0..cols {
            if map[(i, j)] {
                // are we touching a label?
                if i > 0 && label_map[(i - 1, j)] != 0 && j > 0 && label_map[(i, j - 1)] != 0 {
                    // same region
                    label_map[(i, j)] = cmp::min(label_map[(i - 1, j)], label_map[(i - 1, j)]);
                    // store equivalence
                    equivalences.join((i - 1) * cols + j, i * cols + j);
                    equivalences.join(i * cols + j - 1, i * cols + j);
                } else if j > 0 && label_map[(i, j - 1)] != 0 {
                    label_map[(i, j)] = label_map[(i, j - 1)];
                    equivalences.join(i * cols + j, i * cols + j - 1);
                } else if i > 0 && label_map[(i - 1, j)] != 0 {
                    label_map[(i, j)] = label_map[(i - 1, j)];
                    equivalences.join(i * cols + j, (i - 1) * cols + j);
                } else {
                    label += 1;
                    label_map[(i, j)] = label;
                }
            }
        }
    }

    // println!("First pass label map:\n{:?}", label_map);

    //second pass,fix equivalences
    for set in equivalences.sets() {
        let min_label = set
            .iter()
            .fold(99999, |acc, &x| cmp::min(acc, label_map.flatten()[x]));
        for &index in set.iter() {
            let r = index / cols;
            let c = index % cols;
            label_map[(r, c)] = min_label;
        }
    }
    // print!("Second pass label map:\n{:?}", label_map);

    label_map
}

fn areas_perimeter(map: &Array2<u32>) -> Vec<(u32, u32)> {
    let regions = map.iter().filter(|&x| *x != 0).collect::<HashSet<_>>();
    let mut areas_perimeters = Vec::new();
    for region in regions {
        let area = map.iter().filter(|&&x| x == *region).count() as u32;
        let zero_one_map = Array2::from_shape_fn([map.shape()[0], map.shape()[1]], |idx| {
            (map[idx] == *region) as i8
        });
        let mut padded_zero_one_map = Array2::<i8>::zeros([map.shape()[0] + 2, map.shape()[1] + 2]);
        padded_zero_one_map
            .slice_mut(s![1..-1, 1..-1])
            .assign(&zero_one_map);
        let row_borders = padded_zero_one_map
            .diff(1, Axis(0))
            .map(|x| x.abs() as u32)
            .sum();
        let col_borders = padded_zero_one_map
            .diff(1, Axis(1))
            .map(|x| x.abs() as u32)
            .sum();
        areas_perimeters.push((area, row_borders + col_borders));
    }
    areas_perimeters
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
        .map(|&c| c as u32 - 'A' as u32)
        .collect::<Vec<_>>();
    // Convert the contents to a 2D array
    let size = (content_vec.len() as f32).sqrt() as usize;
    let map = Array::from_shape_vec((size, size), content_vec).unwrap();
    println!("Map:\n{:?}", map);
    let maps = expand(&map);
    let labels = maps.iter().map(|m| label(m));
    let mut total = 0;
    for l in labels {
        println!("Labeled map:\n{:?}", l);
        let areas_perimeters = areas_perimeter(&l);
        let price = areas_perimeters
            .iter()
            .map(|(area, perimeter)| area * perimeter)
            .collect::<Vec<_>>();
        println!(
            "Areas and perimeters: {:?}, price: {:?}",
            areas_perimeters, price
        );
        total += price.iter().sum::<u32>();
    }
    println!("Total price: {}", total);
}
