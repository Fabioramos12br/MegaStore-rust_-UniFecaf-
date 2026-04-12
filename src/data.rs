use crate::models::Product;
use std::collections::HashMap;

pub fn carregar_catalogo() -> (
    HashMap<String, Product>,
    HashMap<u32, Product>,
    HashMap<String, Vec<u32>>, // índice por marca
    HashMap<String, Vec<u32>>, // índice por categoria
) {
    let mut mapa_nome = HashMap::new();
    let mut mapa_id = HashMap::new();
    let mut index_marca: HashMap<String, Vec<u32>> = HashMap::new();
    let mut index_categoria: HashMap<String, Vec<u32>> = HashMap::new();

    for i in 1..=1000000 {
        let nome = format!("Produto {:04}", i);
        let marca = "Marca Geral".to_string();
        let categoria = "Eletrónicos".to_string();

        let p = Product::new(i, &nome, &marca, &categoria, 20.0 + (i as f64 * 0.1));

        mapa_nome.insert(p.nome.clone(), p.clone());
        mapa_id.insert(p.id, p.clone());

        index_marca.entry(marca.clone()).or_default().push(p.id);
        index_categoria.entry(categoria.clone()).or_default().push(p.id);
    }

    let itens_especificos = vec![
        Product::new(5001, "Smartphone Galaxy S24", "Samsung", "Eletrónicos", 5999.0),
        Product::new(5002, "iPhone 15 Pro", "Apple", "Eletrónicos", 7200.0),
        Product::new(5003, "Monitor Gamer 27", "Dell", "Informática", 1850.0),
    ];

    for p in itens_especificos {
        mapa_nome.insert(p.nome.clone(), p.clone());
        mapa_id.insert(p.id, p.clone());

        index_marca.entry(p.marca.clone()).or_default().push(p.id);
        index_categoria.entry(p.categoria.clone()).or_default().push(p.id);
    }

    (mapa_nome, mapa_id, index_marca, index_categoria)
}