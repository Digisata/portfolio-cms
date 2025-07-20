#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::fs::FileServer;
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use std::collections::HashMap;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

mod db;
mod errors;
mod fairings;
mod models;
mod request_guards;
mod routes;

pub struct Container {
    services: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Send + Sync>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let mut container = Container::new();

    let database = db::connect().await.unwrap();

    let customer_repo = db::customer::CustomerRepo::new(database.clone());
    let customer_service: Arc<dyn routes::traits::CustomerRepository + Send + Sync> =
        Arc::new(customer_repo);

    let experience_repo = db::experience::ExperienceRepo::new(database.clone());
    let experience_service: Arc<dyn routes::traits::ExperienceRepository + Send + Sync> =
        Arc::new(experience_repo);

    let project_repo = db::project::ProjectRepo::new(database.clone());
    let project_service: Arc<dyn routes::traits::ProjectRepository + Send + Sync> =
        Arc::new(project_repo);

    let skill_repo = db::skill::SkillRepo::new(database.clone());
    let skill_service: Arc<dyn routes::traits::SkillRepository + Send + Sync> =
        Arc::new(skill_repo);

    let social_repo = db::social::SocialRepo::new(database.clone());
    let social_service: Arc<dyn routes::traits::SocialRepository + Send + Sync> =
        Arc::new(social_repo);

    container.register(customer_service);
    container.register(experience_service);
    container.register(project_service);
    container.register(skill_service);
    container.register(social_service);

    rocket::build()
        // .attach(db::init())
        .manage(container)
        .attach(fairings::cors::Cors::new())
        // Serve React frontend from /var/www/html (as copied in Dockerfile)
        .mount("/", FileServer::from("public"))
        .mount(
            "/api",
            openapi_get_routes![
                routes::index,
                routes::auth::login,
                routes::customer::get_customers,
                routes::customer::get_customer_by_id,
                routes::customer::get_customer_by_email,
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
        .mount("/api", routes![routes::all_options_handler])
        .mount(
            "/api-docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

// Unit testings
#[cfg(test)]
mod tests;
