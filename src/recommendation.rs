use std::collections::{VecDeque, HashSet};
use crate::graph::Graph;

pub fn recommend_bfs(graph: &Graph, start: u32) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut recommendations = Vec::new();

    // começa pelo produto inicial
    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.adjacency_list.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    recommendations.push(neighbor);
                }
            }
        }
    }

    recommendations
}