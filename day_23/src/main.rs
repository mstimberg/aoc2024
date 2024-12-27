use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

fn sets_of_three(
    graph: HashSet<(String, String)>,
    has_to_start_with: &str,
) -> HashSet<(String, String, String)> {
    let mut sets = HashSet::new();
    for (a, b) in graph.iter() {
        let sets_with_a = graph
            .iter()
            .filter(|(x, y)| x == a || y == a)
            .map(|(x, y)| if x == a { y } else { x })
            .collect::<HashSet<_>>();
        for bb in sets_with_a {
            if graph.contains(&(bb.clone(), b.clone())) || graph.contains(&(b.clone(), bb.clone()))
            {
                if a.starts_with(has_to_start_with)
                    || b.starts_with(has_to_start_with)
                    || bb.starts_with(has_to_start_with)
                {
                    let mut new_set = vec![];
                    new_set.push(a);
                    new_set.push(bb);
                    new_set.push(b);
                    new_set.sort();
                    sets.insert((new_set[0].clone(), new_set[1].clone(), new_set[2].clone()));
                }
            }
        }
    }
    sets
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read the file");
    let connections = content
        .lines()
        .map(|x| (x[..2].to_string(), x[3..5].to_string()))
        .collect::<Vec<_>>();
    let mut graph = std::collections::HashSet::new();
    for (a, b) in connections {
        // to avoid storing both directions, we always add in alphabetical order
        if a < b {
            graph.insert((a, b));
        } else {
            graph.insert((b, a));
        }
    }
    let sets = sets_of_three(graph, "t");
    for (a, b, c) in sets.iter() {
        println!("{} {} {}", a, b, c);
    }
    println!(
        "There are {} sets of three that start with {}",
        sets.len(),
        "t"
    );
}
