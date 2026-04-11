use std::collections::{HashSet, VecDeque};
use crate::graph::Graph;

pub fn recommend_bfs(graph: &Graph, start: u32, depth: usize) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let mut recommendations = Vec::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((node, level)) = queue.pop_front() {
        if level >= depth {
            continue;
        }

        for neighbor in graph.get_neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                recommendations.push(neighbor);
                queue.push_back((neighbor, level + 1));
            }
        }
    }

    recommendations
}