#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

mod db;
mod errors;
mod fairings;
mod models;
mod request_guards;
mod routes;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init())
        .attach(fairings::cors::Cors)
        .mount(
            "/",
            openapi_get_routes![
                routes::index,
                routes::auth::login,
                routes::customer::get_customers,
                routes::customer::get_customer_by_id,
                routes::customer::post_customer,
                routes::customer::patch_customer_by_id,
                routes::customer::delete_customer_by_id,
                routes::experience::get,
                routes::experience::get_by_id,
                routes::experience::post,
                routes::experience::patch_by_id,
                routes::experience::patch_many,
                routes::experience::delete_by_id,
                routes::project::get,
                routes::project::get_by_id,
                routes::project::post,
                routes::project::patch_by_id,
                routes::project::patch_many,
                routes::project::delete_by_id,
                routes::skill::get,
                routes::skill::get_by_id,
                routes::skill::post,
                routes::skill::patch_by_id,
                routes::skill::patch_many,
                routes::skill::delete_by_id,
                routes::social::get,
                routes::social::get_by_id,
                routes::social::post,
                routes::social::patch_by_id,
                routes::social::patch_many,
                routes::social::delete_by_id,
            ],
        )
        .mount(
            "/api-docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

// Unit testings
#[cfg(test)]
mod tests;
