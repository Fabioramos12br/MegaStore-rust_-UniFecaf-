mod models;
mod graph;
mod recommendation;
mod data;

use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Instant;
use models::Product;
use graph::Graph;
use recommendation::recommend_bfs;
use data::carregar_catalogo;

fn main() {
    println!("=========================================================");
    println!("   SISTEMA DE BUSCA OTIMIZADO - MEGASTORE (UniFecaf)    ");
    println!("=========================================================");

    let catalogo: HashMap<String, Product> = carregar_catalogo();
    
    let mut grafo = Graph::new();
    configurar_conexoes_grafo(&mut grafo);

    println!("\n✅ {} produtos indexados. Sistema pronto!", catalogo.len());

    loop {
        print!("\n🔍 Digite o nome do produto (ou 'sair'): ");
        io::stdout().flush().unwrap();

        let mut busca = String::new();
        io::stdin().read_line(&mut busca).expect("Falha ao ler entrada");
        let termo = busca.trim();

        if termo.to_lowercase() == "sair" {
            break;
        }

        let agora = Instant::now();
        let resultado = catalogo.get(termo);
        let duracao_busca = agora.elapsed();

        match resultado {
            Some(p) => {
                println!("\n✅ RESULTADO ENCONTRADO (Tempo: {:?})", duracao_busca);
                println!("ID: {} | Nome: {} | Preço: R${:.2}", p.id, p.nome, p.preco);
                println!("Marca: {} | Categoria: {}", p.marca, p.categoria);

                println!("\n💡 RECOMENDAÇÃO: Clientes que viram este produto também viram:");
                let recomendacoes = recommend_bfs(&grafo, p.id);

                let mut encontrou_rec = false;
                for id_rec in recomendacoes {
                    if let Some(prod_rec) = catalogo.values().find(|x| x.id == id_rec) {
                        println!(" -> {}", prod_rec.nome); // Aqui ele imprime o Nome em vez do ID
                        encontrou_rec = true;
    }
}

if !encontrou_rec {
    println!(" -> Nenhuma recomendação vinculada.");
}
            },
            None => {
                println!("\n❌ Produto '{}' não encontrado.", termo);
            }
        }
        println!("---------------------------------------------------------");
    }
}

fn configurar_conexoes_grafo(grafo: &mut Graph) {
    grafo.add_edge(5001, 5002);
    grafo.add_edge(5001, 5003);
    grafo.add_edge(5002, 5003);
    grafo.add_edge(5001, 10); 
}