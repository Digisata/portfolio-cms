use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::models::response::MessageResponse;

pub mod auth;
pub mod customer;
pub mod experience;
pub mod project;
pub mod skill;
pub mod social;
pub mod traits;

#[options("/<_..>")]
pub fn all_options_handler() -> &'static str {
    ""
}
