# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
O **MegaStore** é um sistema de busca de alta performance desenvolvido como projeto prático para a disciplina de Estrutura de Dados. O objetivo principal é demonstrar a viabilidade de processamento de grandes volumes de dados (Big Data) em tempo real, utilizando a linguagem Rust. O sistema permite a indexação de milhões de produtos e oferece funcionalidades de busca instantânea por múltiplos critérios e um motor de recomendação baseado em relacionamentos.

### Funcionalidades
- **Busca Multi-índice**: Localização de itens por Marca, Categoria ou Nome.
- **Motor de Recomendação**: Sugestões inteligentes baseadas em afinidade de produtos.
- **Geração Massiva de Dados**: Simulação de um catálogo com 1.000.000 de itens.
- **Análise de Latência**: Medição precisa do tempo de resposta em microssegundos.

## Tecnologias Utilizadas
- **Linguagem**: Rust (Edição 2021).
- **Gerenciador de Pacotes**: Cargo.
- **Bibliotecas (Crates)**: Utiliza prioritariamente a `std` (Standard Library) do Rust para garantir máxima performance sem overhead, focando em `std::collections` (HashMap, VecDeque, HashSet).
- **Ferramentas de Teste**: `cargo test` para testes unitários e de integração.

## Instruções de Como Executar o Sistema de Busca

### Pré-requisitos
- Ter o Rust e Cargo instalados em sua máquina.

### Passo a Passo
1. Clone o repositório ou navegue até a pasta do projeto.
2. Para compilar e executar com otimizações de performance (recomendado para o volume de 1M de itens):
   ```bash
   cargo run --release
   ```
3. Para compilar em modo debug:
   ```bash
   cargo build
   ```

## Instruções de Como Executar os Testes
O projeto conta com uma suíte de testes para garantir a integridade das buscas e do motor de recomendação.

1. Para executar todos os testes:
   ```bash
   cargo test
   ```
2. Para ver a saída detalhada dos testes:
   ```bash
   cargo test -- --nocapture
   ```

## Exemplos de Uso
Ao iniciar o sistema, você verá um prompt interativo. Aqui estão alguns exemplos de como interagir:

*   **Busca por Marca**: Digite `Apple` ou `Samsung`.
*   **Busca por Categoria**: Digite `Eletrônicos` ou `Móveis`.
*   **Busca por Nome Parcial**: Digite `iPhone` ou `Galaxy`.
*   **Recomendações**: Ao selecionar um produto (ex: ID 5001), o sistema sugere automaticamente itens relacionados através do grafo.

## Arquitetura do Sistema
O sistema é modularizado para garantir separação de responsabilidades:

*   `src/main.rs`: Orquestrador da Interface de Linha de Comando (CLI) e fluxo principal.
*   `src/models.rs`: Definição da entidade `Product`.
*   `src/graph.rs`: Implementação da estrutura de dados de Grafo (Lista de Adjacência).
*   `src/recommendation.rs`: Implementação do algoritmo BFS.
*   `src/data.rs`: Responsável pela geração e indexação inicial dos dados.

## Algoritmos e Estruturas de Dados Utilizados
A eficiência do sistema baseia-se na escolha criteriosa de estruturas de dados:

1.  **Tabelas Hash (HashMaps)**:
    - **Indexação por ID**: Busca $O(1)$ para recuperar detalhes do produto.
    - **Indexação por Nome/Marca/Categoria**: Mapeia termos para listas de IDs, permitindo que a busca por filtros ocorra em tempo constante, independentemente do tamanho do catálogo.
2.  **Grafos**:
    - Utilizado para representar o ecossistema de produtos. Cada produto é um nó e as relações (afinidade de compra) são arestas.
3.  **BFS (Breadth-First Search)**:
    - Algoritmo de busca em largura utilizado no grafo para encontrar produtos "vizinhos" e gerar recomendações de primeiro e segundo nível.

## Considerações sobre Desempenho e Escalabilidade
O sistema foi desenhado para escalabilidade vertical:
- **Latência**: Em testes realizados com 1 milhão de produtos, o tempo de resposta médio para buscas indexadas foi de **15 a 30 microssegundos**.
- **Uso de Memória**: O Rust gerencia a memória de forma eficiente através do sistema de *Ownership*, evitando pausar o sistema para Garbage Collection, o que é crítico em sistemas de recomendação em tempo real.
- **Escalabilidade**: A complexidade $O(1)$ das tabelas hash garante que, mesmo dobrando o número de produtos, o tempo de busca permaneça praticamente inalterado.

## Contribuições
Este é um projeto acadêmico. Para contribuir:
1. Faça um Fork do projeto.
2. Crie uma branch para sua feature (`git checkout -b feature/nova-funcionalidade`).
3. Commit suas mudanças (`git commit -m 'Adicionando nova funcionalidade'`).
4. Push para a branch (`git push origin feature/nova-funcionalidade`).
5. Abra um Pull Request.

## Licença
Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.

---
*Projeto desenvolvido para a disciplina de Estrutura de Dados - UniFecaf.*