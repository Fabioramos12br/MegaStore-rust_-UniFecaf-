#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32, // ID numérico para o Grafo
    pub nome: String,
    pub marca: String,
    pub categoria: String,
    pub preco: f64,
}

impl Product {
    pub fn new(id: u32, nome: &str, marca: &str, categoria: &str, preco: f64) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            marca: marca.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}