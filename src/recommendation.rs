use crate::graph::Graph;
use std::collections::VecDeque;

pub fn recommend_bfs(graph: &Graph, start_node: u32) -> Vec<u32> {
    let mut recommendations = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start_node);
    queue.push_back(start_node);

    while let Some(current) = queue.pop_front() {
        for &neighbor in graph.get_neighbors(current).iter() {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                recommendations.push(neighbor);
                // Limitamos a recomendação aos vizinhos mais próximos para ser rápido
                if recommendations.len() >= 3 { break; }
            }
        }
        if recommendations.len() >= 3 { break; }
    }
    recommendations
}