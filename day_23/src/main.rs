use itertools::sorted;
use std::collections::{HashMap, HashSet};
use std::fs;

fn biggest_clique(graph: HashMap<String, HashSet<String>>) -> Vec<String> {
    // This assumes that the there is only one node missing from the clique for each node
    for node in graph.keys() {
        let edges_vec = graph[node].clone().into_iter().collect::<Vec<_>>();
        for remove_idx in 0..graph[node].len() {
            let mut new_edges = edges_vec.clone();
            new_edges.remove(remove_idx);
            let mut all_connected = true;
            for check_idx in 0..new_edges.len() {
                let edge = new_edges[check_idx].clone();
                if !graph[&edge].contains(node) {
                    all_connected = false;
                    break;
                } else {
                    for (idx, edge2) in new_edges.clone().into_iter().enumerate() {
                        if idx == check_idx {
                            continue;
                        }
                        if !graph[&edge2].contains(&edge) {
                            // only check each pair once
                            all_connected = false;
                            break;
                        }
                    }
                    if !all_connected {
                        break;
                    }
                }
                if !all_connected {
                    break;
                }
            }
            if all_connected {
                let mut clique = vec![node.clone()];
                clique.extend(new_edges);
                clique.sort();
                return clique;
            }
        }
    }
    vec![]
}
fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read the file");
    let connections = content
        .lines()
        .map(|x| (x[..2].to_string(), x[3..5].to_string()))
        .collect::<Vec<_>>();
    let mut graph = std::collections::HashMap::new();
    for (a, b) in connections {
        graph
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        graph
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
    }
    let clique = biggest_clique(graph.clone());
    println!("Biggest clique: {:?}", clique.join(","));
    for node in clique {
        print!("{}: ", node);
        for edge in sorted(graph[&node].clone()) {
            print!("{}, ", edge);
        }
        println!();
    }
}
