use meu_projeto::graph::Graph;
use meu_projeto::recommendation::recommend_bfs;

#[test]
fn test_recommendation_basic() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2);

    let result = recommend_bfs(&graph, 1);

    assert_eq!(result, vec![2]);
}

#[test]
fn test_multiple_recommendations() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);

    let result = recommend_bfs(&graph, 1);

    assert_eq!(result.len(), 3);
    assert!(result.contains(&2));
    assert!(result.contains(&3));
    assert!(result.contains(&4));
}

#[test]
fn test_bfs_depth() {
    let mut graph = Graph::new();

    // 1 -> 2 -> 3
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);

    let result = recommend_bfs(&graph, 1);

    // Espera que o BFS alcance nível 2
    assert!(result.contains(&2));
    assert!(result.contains(&3));
}

#[test]
fn test_no_duplicates() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2);
    graph.add_edge(1, 2); // duplicado

    let result = recommend_bfs(&graph, 1);

    // Não deve repetir
    let count = result.iter().filter(|&&x| x == 2).count();
    assert_eq!(count, 1);
}

#[test]
fn test_no_self_recommendation() {
    let mut graph = Graph::new();

    graph.add_edge(1, 1);

    let result = recommend_bfs(&graph, 1);

    // Não deve recomendar ele mesmo
    assert!(!result.contains(&1));
}

#[test]
fn test_no_recommendation() {
    let graph = Graph::new();

    let result = recommend_bfs(&graph, 99);

    assert!(result.is_empty());
}

#[test]
fn test_large_graph() {
    let mut graph = Graph::new();

    // Simula muitos produtos
    for i in 1..1000 {
        graph.add_edge(i, i + 1);
    }

    let result = recommend_bfs(&graph, 1);

    // Deve recomendar vários produtos
    assert!(result.len() > 1);
    assert!(result.contains(&2));
}