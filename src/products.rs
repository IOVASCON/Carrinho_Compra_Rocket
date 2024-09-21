use serde::Serialize;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub price: u64,  // Alterado para u64 para permitir Hash e Eq
}

#[allow(dead_code)]
impl Product {
    pub fn get_by_id(id: usize) -> Product {
        let products = get_products();
        products.into_iter().find(|p| p.id == id).unwrap()
    }
}

pub fn get_products() -> Vec<Product> {
    vec![
        Product { id: 1, name: "Produto A".to_string(), price: 1000 }, // 1000 centavos = 10 reais
        Product { id: 2, name: "Produto B".to_string(), price: 2000 }, // 2000 centavos = 20 reais
        Product { id: 3, name: "Produto C".to_string(), price: 3000 }, // 3000 centavos = 30 reais
    ]
}
