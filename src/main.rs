mod models;
mod graph;
mod recommendation;
mod data;

use std::io::{self, Write};
use std::time::Instant;

use graph::Graph;
use recommendation::recommend_bfs;
use data::carregar_catalogo;

fn main() {
    println!("=========================================================");
    println!("   SISTEMA DE BUSCA OTIMIZADO - MEGASTORE (UniFecaf)    ");
    println!("=========================================================");

    // 🔥 Agora com 4 estruturas (nível profissional)
    let (catalogo_nome, catalogo_id, index_marca, index_categoria) = carregar_catalogo();

    let mut grafo = Graph::new();
    configurar_conexoes_grafo(&mut grafo);

    println!(
        "\n✅ {} produtos indexados. Sistema pronto!",
        catalogo_nome.len()
    );

    loop {
        print!("\n🔍 Digite nome, marca ou categoria (ou 'sair'): ");
        io::stdout().flush().unwrap();

        let mut busca = String::new();
        io::stdin()
            .read_line(&mut busca)
            .expect("Falha ao ler entrada");

        let termo = busca.trim();

        if termo.to_lowercase() == "sair" {
            break;
        }

        let agora = Instant::now();

        let termo_lower = termo.to_lowercase();
        let mut resultados_ids: Vec<u32> = Vec::new();

        // 🔥 busca por marca (O(1))
        if let Some(ids) = index_marca.get(&termo_lower) {
            resultados_ids.extend(ids);
        }

        // 🔥 busca por categoria (O(1))
        if let Some(ids) = index_categoria.get(&termo_lower) {
            resultados_ids.extend(ids);
        }

        // 🔥 fallback: busca por nome parcial (O(n))
        if resultados_ids.is_empty() {
            resultados_ids = catalogo_nome
                .values()
                .filter(|p| p.nome.to_lowercase().contains(&termo_lower))
                .map(|p| p.id)
                .take(10)
                .collect();
        }

        // 🔥 transformar IDs em produtos
        let resultados: Vec<_> = resultados_ids
            .iter()
            .take(10)
            .filter_map(|id| catalogo_id.get(id))
            .collect();

        let duracao = agora.elapsed();

        if resultados.is_empty() {
            println!("\n❌ Nenhum produto encontrado. (Tempo: {:?})", duracao);
        } else {
            println!(
                "\n✅ {} resultado(s) encontrado(s)! (Tempo: {:?})",
                resultados.len(),
                duracao
            );

            for p in &resultados {
                println!(
                    "ID: {} | Nome: {} | Preço: R${:.2}",
                    p.id, p.nome, p.preco
                );
                println!("Marca: {} | Categoria: {}", p.marca, p.categoria);
                println!("-----------------------------------");
            }

            // 🔥 RECOMENDAÇÕES (BFS)
            let produto_base = resultados[0];

            println!("\n💡 RECOMENDAÇÕES:");
            let recomendacoes = recommend_bfs(&grafo, produto_base.id);

            let mut encontrou = false;

            for id_rec in recomendacoes {
                if let Some(prod_rec) = catalogo_id.get(&id_rec) {
                    println!(" -> {}", prod_rec.nome);
                    encontrou = true;
                }
            }

            if !encontrou {
                println!(" -> Nenhuma recomendação.");
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