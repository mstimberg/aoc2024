use ndarray::{array, Array, Array2};
use std::fs;

fn count_matches(pattern: &Array2<char>, crossword: &Array2<char>) -> usize {
    let pattern_shape = pattern.shape();
    let fixed_shape = [pattern_shape[0], pattern_shape[1]]; // Convert to fixed-size array
    crossword
        .windows(fixed_shape)
        .into_iter()
        .fold(0, |acc, window| {
            if window
                .iter()
                .zip(pattern.iter())
                .fold(0, |s, (c, p)| s + (*p != '.' && *p == *c) as i32)
                == 4
            {
                acc + 1
            } else {
                acc
            }
        })
}

fn main() {
    let patterns = [
        array![['X', 'M', 'A', 'S']],
        array![['S', 'A', 'M', 'X']],
        array![['X'], ['M'], ['A'], ['S']],
        array![['S'], ['A'], ['M'], ['X']],
        array![
            ['X', '.', '.', '.'],
            ['.', 'M', '.', '.'],
            ['.', '.', 'A', '.'],
            ['.', '.', '.', 'S']
        ],
        array![
            ['S', '.', '.', '.'],
            ['.', 'A', '.', '.'],
            ['.', '.', 'M', '.'],
            ['.', '.', '.', 'X']
        ],
        array![
            ['.', '.', '.', 'X'],
            ['.', '.', 'M', '.'],
            ['.', 'A', '.', '.'],
            ['S', '.', '.', '.']
        ],
        array![
            ['.', '.', '.', 'S'],
            ['.', '.', 'A', '.'],
            ['.', 'M', '.', '.'],
            ['X', '.', '.', '.']
        ],
    ];
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let content_vec = contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.extend(line.chars());
        acc
    });
    // Convert the contents to a 2D array
    let size = (content_vec.len() as f32).sqrt() as usize;
    let crossword = Array::from_shape_vec((size, size), content_vec).unwrap();
    let matches = patterns
        .iter()
        .map(|pattern| count_matches(pattern, &crossword))
        .sum::<usize>();
    println!("matches: {:?}", matches);
}