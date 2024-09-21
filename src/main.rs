mod routes;
mod cart;
mod products;
mod models;

use rocket_dyn_templates::Template;
use rocket::{launch, routes};  // Importa `launch` e `routes` aqui
use routes::{index, view_cart, add_to_cart, remove_from_cart};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())  // Anexa o Tera ao Rocket
        .mount("/", routes![index, view_cart, add_to_cart, remove_from_cart])
}
