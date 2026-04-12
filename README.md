# 🚀 MegaStore: Sistema de Busca e Recomendação de Alta Performance

Este projeto é um motor de busca e recomendação desenvolvido em **Rust** para a disciplina de estrutura de dados (UniFecaf). Ele demonstra como lidar com grandes volumes de dados (Big Data) e relações complexas entre produtos utilizando estruturas altamente eficientes como **HashMaps** e **Grafos**.

## 🚀 Funcionalidades

- **Busca Instantânea**: Localização de itens por nome exato em tempo constante $O(1)$ através de um `HashMap`, testado com um catálogo de **1.000.000 de produtos**.
- **Motor de Recomendação Inteligente**: Utiliza um **Grafo não-direcionado** para mapear afinidades entre produtos.
- **Algoritmo BFS (Breadth-First Search)**: Implementação de busca em largura para encontrar recomendações de vizinhança imediata.
- **Geração Dinâmica de Dados**: Script para simulação de inventário em larga escala e inserção de itens específicos para testes.
- **Interface CLI**: Terminal interativo com medição de tempo de resposta em microssegundos.

## 🛠️ Arquitetura do Sistema

O software é dividido em módulos para garantir manutenibilidade e separação de conceitos:

- `src/main.rs`: Orquestrador principal. Gerencia o loop de entrada do usuário e exibe os resultados.
- `src/models.rs`: Define a estrutura `Product` com metadados como preço, marca e categoria.
- `src/graph.rs`: Implementação da estrutura de dados de Grafo (Lista de Adjacência).
- `src/recommendation.rs`: Lógica algorítmica de travessia do grafo.
- `src/data.rs`: Camada de persistência simulada que popula o catálogo.

## 🧠 Engenharia de Software Aplicada

### 1. Performance de Busca
Diferente de uma busca linear em vetores ($O(n)$), o uso de uma Tabela Hash para o catálogo permite que o sistema encontre um produto específico entre milhões de entradas em apenas alguns microssegundos ($\approx 15\mu s$).

### 2. O Grafo de Relacionamentos
As conexões entre produtos (Ex: *iPhone 15* ↔ *Monitor Gamer*) são representadas por arestas em um grafo. Isso permite expandir o motor de recomendações para sistemas de "Quem comprou, também comprou".

### 3. Gestão de Memória com Rust
O projeto tira proveito do sistema de *Ownership* do Rust para garantir que o catálogo massivo seja gerenciado sem *memory leaks* e sem a necessidade de um Garbage Collector.

## 💻 Como Instalar e Rodar

### 1. Pré-requisitos
*   Rust & Cargo instalados.

### 2. Execução
Para testar com a performance máxima (especialmente devido ao 1 milhão de itens), execute em modo release:
```bash
cargo run --release
```

## 📊 Exemplo de Fluxo

1. **Inicialização**: O sistema gera 1.000.000 de produtos e indexa nomes como "Smartphone Galaxy S24".
2. **Busca**: O usuário digita "iPhone 15 Pro".
3. **Processamento**: 
   - O `HashMap` retorna o objeto `Product` instantaneamente.
   - O motor de recomendação consulta o `Graph` pelo ID 5002.
   - O BFS encontra IDs vizinhos (ex: 5001 e 5003).
4. **Saída**: Exibe detalhes do produto e as sugestões de compra.

## 🚀 Próximos Passos (Roadmap)
- [ ] **Otimização de Memória**: Utilizar um `HashMap<u32, &Product>` para busca de recomendações por ID em $O(1)$ (atualmente utiliza `.find()` que é linear).
- [ ] **Busca por Prefixo**: Implementar uma árvore *Trie* para permitir buscas parciais (ex: digitar "Smart" e aparecer "Smartphone").
- [ ] **Persistência Real**: Conectar a um banco de dados NoSQL ou SQLite.

---
*Projeto desenvolvido para fins acadêmicos na UniFecaf.*