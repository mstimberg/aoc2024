use std::fs;
use ndarray::Array2;
use itertools::Itertools;

fn main() {
    let content = fs::read_to_string("input.txt").expect("File reading failed");
    let mut locks = Vec::<Array2<bool>>::new();
    let mut keys = Vec::<Array2<bool>>::new();
    for block in &content.lines().filter(|&x| !x.is_empty()).chunks(7) {
        let block = block.map(|x| x.chars().map(|c| c == '#').collect::<Vec<_>>()).flatten().collect::<Vec::<_>>();
        let array = Array2::from_shape_vec((7, 5), block).unwrap();
        if array[(0, 0)] {
            locks.push(array);
        } else {
            keys.push(array);
        }
    }
    let mut compatible = 0;
    for lock in &locks {
        for key in &keys {
            if !(lock.clone() & key).iter().any(|&x| x) {  // no overlap
                compatible += 1;
            } 
        }
    }
    println!("{} keys/locks are compatible", compatible);
}
