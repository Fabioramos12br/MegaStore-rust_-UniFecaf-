# MegaStore: Sistema de Busca e Recomendação Otimizada

Este projeto consiste em um protótipo funcional de um sistema de catálogo de produtos e motor de recomendações, desenvolvido em **Rust**. O sistema demonstra a aplicação prática de estruturas de dados avançadas, como Tabelas Hash e Grafos, para resolver problemas reais de performance e relevância em e-commerce.

## 🚀 Funcionalidades

- **Busca por Categoria Otimizada**: Utiliza indexação via `HashMap` para garantir que a localização de produtos por categoria seja feita em tempo constante médio $O(1)$.
- **Relacionamentos via Grafo**: Os produtos não são apenas itens isolados; eles estão conectados em um grafo que representa comportamentos de compra (ex: "quem comprou este produto também se interessou por aquele").
- **Motor de Recomendação (BFS)**: Implementa o algoritmo de Busca em Largura (Breadth-First Search) para navegar pelos relacionamentos do grafo e sugerir itens complementares ao usuário.
- **Interface CLI Interativa**: Permite ao usuário realizar buscas em tempo real através do terminal.

## 🛠️ Arquitetura e Estrutura de Arquivos

O projeto foi modularizado para seguir as melhores práticas de organização de código em Rust:

- **`src/main.rs`**: Ponto de entrada da aplicação. Contém a lógica da interface de linha de comando, a inicialização dos dados e a orquestração entre a busca indexada e as recomendações.
- **`src/product.rs`**: Define a estrutura `Product`, que representa a entidade base do sistema.
- **`src/graph.rs`**: Implementa a estrutura de `Graph` utilizando uma lista de adjacência (mapeada com `HashMap<u32, Vec<u32>>`).
- **`src/recommendation.rs`**: Contém a lógica algorítmica do sistema, especificamente a função `recommend_bfs`.
- **`src/lib.rs`**: Gerencia a visibilidade dos módulos internos.

## 🧠 Conceitos Técnicos Aplicados

### 1. Indexação Otimizada
Em vez de percorrer a lista inteira de produtos toda vez que uma categoria é pesquisada ($O(n)$), o sistema constrói um `category_index` no início da execução. Isso permite que o acesso aos IDs dos produtos de uma categoria específica seja instantâneo.

### 2. Representação em Grafo
O grafo é direcionado, onde cada aresta representa uma recomendação lógica. Por exemplo:
*   Notebook $\rightarrow$ Mouse
*   Notebook $\rightarrow$ Teclado

### 3. Algoritmo de Busca (BFS)
A função de recomendação utiliza uma fila (`VecDeque`) para explorar os vizinhos de um produto. No estado atual, ela está configurada para uma profundidade de nível 1, mas a arquitetura permite recomendações de níveis mais profundos (amigos de amigos).

## 💻 Como Executar

### Pré-requisitos
*   Possuir o Rust instalado (Cargo incluso).

### Passos
1.  Navegue até a pasta do projeto:
    ```bash
    cd z:\FACULDADE\RUST\meu_projeto
    ```
2.  Execute a aplicação:
    ```bash
    cargo run
    ```

---
*Este projeto foi desenvolvido como parte de um desafio prático de Rust na faculdade.*