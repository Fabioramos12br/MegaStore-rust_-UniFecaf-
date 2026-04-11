use std::collections::HashMap;

pub struct Graph {
    pub edges: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    pub fn get_neighbors(&self, id: u32) -> Vec<u32> {
        self.edges.get(&id).cloned().unwrap_or(Vec::new())
    }
}