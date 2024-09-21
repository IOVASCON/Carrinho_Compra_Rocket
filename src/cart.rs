use rocket::request::{Request, FromRequest, Outcome};
use std::collections::HashMap;
pub struct Cart {
    items: HashMap<usize, usize>,  // Usamos o `id` do produto como chave
}

impl Cart {
    pub fn new() -> Self {
        Cart {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, product_id: usize) {
        let entry = self.items.entry(product_id).or_insert(0);
        *entry += 1;
    }

    pub fn remove(&mut self, product_id: usize) {
        if let Some(qty) = self.items.get_mut(&product_id) {
            if *qty > 1 {
                *qty -= 1;
            } else {
                self.items.remove(&product_id);
            }
        }
    }

    pub fn view(&self) -> &HashMap<usize, usize> {
        &self.items
    }
}

// Implementação do `FromRequest` para o tipo `Cart`
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Cart {
    type Error = ();

    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Aqui, estamos apenas inicializando um carrinho vazio para este exemplo.
        // No futuro, você pode carregar o carrinho de uma sessão ou de um banco de dados.
        Outcome::Success(Cart::new())
    }
}
