use meu_projeto::graph::Graph;
use meu_projeto::recommendation::recommend_bfs;

#[test]
fn test_recommendation() {
    let mut graph = Graph::new();

    // conexão
    graph.add_edge(1, 2);

    // executa recomendação
    let result = recommend_bfs(&graph, 1, 1);

    // verifica se está correto
    assert_eq!(result, vec![2]);
}