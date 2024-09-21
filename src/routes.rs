use crate::cart::Cart;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::get;  // Importa `get` aqui
use crate::products::get_products;

#[get("/")]
pub fn index() -> Template {
    let products = get_products();
    let context = context! {
        products: products  // Passa a lista de produtos para o template
    };
    Template::render("index", &context)  // Renderiza o template "index.tera"
}

#[get("/cart")]
pub fn view_cart(cart: Cart) -> Template {
    let products = get_products();  // Obtenha os produtos para encontrar o nome e o preço
    let mut items_html = String::new();

    for (product_id, qty) in cart.view() {
        if let Some(product) = products.iter().find(|p| p.id == *product_id) {
            items_html.push_str(&format!(
                "<li>{} - Quantidade: {} <a href='/remove/{}'>Remover</a></li>",
                product.name, qty, product.id
            ));
        }
    }

    let context = context! {
        items: items_html
    };
    Template::render("cart", &context)  // Renderiza o template "cart.tera"
}

#[get("/add/<product_id>")]
pub fn add_to_cart(mut cart: Cart, product_id: usize) -> Template {
    cart.add(product_id);
    let products = get_products();
    let context = context! {
        message: "Produto adicionado ao carrinho.",
        products: products
    };
    Template::render("index", &context)  // Renderiza o template "index.tera"
}

#[get("/remove/<product_id>")]
pub fn remove_from_cart(mut cart: Cart, product_id: usize) -> Template {
    cart.remove(product_id);
    let products = get_products();
    let context = context! {
        message: "Produto removido do carrinho.",
        products: products
    };
    Template::render("index", &context)  // Renderiza o template "index.tera"
}
