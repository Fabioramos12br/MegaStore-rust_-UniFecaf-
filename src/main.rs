mod product;
mod graph;
mod recommendation;

use std::collections::HashMap;
use std::io::{self, Write}; // Necessário para entrada de dados
use product::Product;
use graph::Graph;
use recommendation::recommend_bfs;

fn main() {
    // 1. Catálogo de Produtos (Vasto catálogo conforme o desafio) [cite: 6]
    let products = vec![
        Product { id: 1, name: "Notebook".to_string(), category: "Eletronicos".to_string() },
        Product { id: 2, name: "Mouse".to_string(), category: "Eletronicos".to_string() },
        Product { id: 3, name: "Teclado".to_string(), category: "Eletronicos".to_string() },
        Product { id: 4, name: "Cadeira Gamer".to_string(), category: "Moveis".to_string() },
        Product { id: 5, name: "Monitor 4K".to_string(), category: "Eletronicos".to_string() },
        Product { id: 6, name: "Mesa de Escritorio".to_string(), category: "Moveis".to_string() },
        Product { id: 7, name: "Headset Wireless".to_string(), category: "Acessorios".to_string() },
        Product { id: 8, name: "Webcam HD".to_string(), category: "Acessorios".to_string() },
        Product { id: 9, name: "Suporte para Monitor".to_string(), category: "Acessorios".to_string() },
    ];

    // 2. Indexação Otimizada com Tabelas Hash [cite: 44, 74]
    let mut category_index: HashMap<String, Vec<u32>> = HashMap::new();
    for p in &products {
        category_index
            .entry(p.category.to_lowercase())
            .or_insert(Vec::new())
            .push(p.id);
    }

    // 3. Grafo de Relacionamentos 
    let mut graph = Graph::new();
    graph.add_edge(1, 2); // Notebook -> Mouse
    graph.add_edge(1, 3); // Notebook -> Teclado
    graph.add_edge(1, 5); // Notebook -> Monitor
    graph.add_edge(7, 8); // Headset -> Webcam
    graph.add_edge(7, 9); // Headset -> Suporte

    println!("=== Sistema de Busca Otimizada MegaStore ===");

    // 4. Loop de Busca Interativa
    loop {
        print!("\nDigite a categoria que deseja procurar (ou 'sair'): ");
        io::stdout().flush().unwrap(); // Garante que a mensagem apareça antes da entrada

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler a entrada");
        let busca = input.trim().to_lowercase();

        if busca == "sair" {
            println!("Encerrando o sistema...");
            break;
        }

        // Realiza a busca rápida via HashMap [cite: 15, 22]
        if let Some(ids) = category_index.get(&busca) {
            println!("\n--- Resultados encontrados para: {} ---", busca);
            for id in ids {
                // Encontra os detalhes do produto
                if let Some(p) = products.iter().find(|prod| prod.id == *id) {
                    println!("Produto: {}", p.name);
                    
                    // Gera recomendações via BFS no Grafo 
                    let recs = recommend_bfs(&graph, *id, 1);
                    if !recs.is_empty() {
                        print!("  -> Clientes também levaram: ");
                        for r_id in recs {
                            if let Some(rp) = products.iter().find(|prod| prod.id == r_id) {
                                print!("[{}] ", rp.name);
                            }
                        }
                        println!();
                    }
                }
            }
        } else {
            println!("Nenhum produto encontrado para a categoria '{}'.", busca);
        }
    }
}