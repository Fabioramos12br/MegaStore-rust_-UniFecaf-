use meu_projeto::graph::Graph;
use meu_projeto::recommendation::recommend_bfs;

#[test]
fn test_recommendation_basic() {
    let mut graph = Graph::new();

    // Cria uma conexão entre o produto 1 e o produto 2
    graph.add_edge(1, 2);

    // Executa a recomendação (removido o terceiro argumento '1' que causava erro)
    let result = recommend_bfs(&graph, 1);

    // Verifica se o produto 2 foi recomendado para quem viu o 1
    assert_eq!(result, vec![2]);
}

#[test]
fn test_no_recommendation() {
    let graph = Graph::new();
    
    // Testa um produto que não tem conexões
    let result = recommend_bfs(&graph, 99);
    
    assert!(result.is_empty());
}