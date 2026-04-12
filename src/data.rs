use crate::models::Product;
use std::collections::HashMap;

pub fn carregar_catalogo() -> HashMap<String, Product> {
    let mut mapa = HashMap::new();

    // 1. Gerar milhares de produtos automaticamente para escala
    for i in 1..=1000000 {
        let nome = format!("Produto {:04}", i);
        let p = Product::new(
            i,
            &nome,
            "Marca Geral",
            "Eletrónicos",
            20.0 + (i as f64 * 0.1),
        );
        mapa.insert(p.nome.clone(), p);
    }

    // 2. Adicionar itens específicos para testar a busca manual
    let itens_especificos = vec![
        Product::new(5001, "Smartphone Galaxy S24", "Samsung", "Eletrónicos", 5999.0),
        Product::new(5002, "iPhone 15 Pro", "Apple", "Eletrónicos", 7200.0),
        Product::new(5003, "Monitor Gamer 27", "Dell", "Informática", 1850.0),
    ];

    for p in itens_especificos {
        mapa.insert(p.nome.clone(), p);
    }

    mapa
}