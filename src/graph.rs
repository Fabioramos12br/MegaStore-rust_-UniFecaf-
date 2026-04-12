use std::collections::HashMap;

pub struct Graph {
    // Mapeia o ID de um produto para uma lista de IDs de produtos relacionados
    pub adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.adjacency_list.entry(u).or_insert(Vec::new()).push(v);
        self.adjacency_list.entry(v).or_insert(Vec::new()).push(u); // Grafo não-direcionado
    }

    pub fn get_neighbors(&self, id: u32) -> Vec<u32> {
        self.adjacency_list.get(&id).cloned().unwrap_or_default()
    }
}