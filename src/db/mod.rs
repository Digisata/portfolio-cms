// use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Database};
// use rocket::fairing::AdHoc;
use std::env;

pub mod customer;
pub mod experience;
pub mod project;
pub mod skill;
pub mod social;

// pub fn init() -> AdHoc {
//     AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
//         match connect().await {
//             Ok(database) => rocket.manage(database),
//             Err(error) => {
//                 panic!("Cannot connect to instance:: {:?}", error)
//             }
//         }
//     })
// }

pub async fn connect() -> mongodb::error::Result<Database> {
    let mongo_host = env::var("MONGO_DB_HOST").expect("MONGO_DB_HOST is not found.");
    let mongo_port = env::var("MONGO_DB_PORT").expect("MONGO_DB_PORT is not found.");
    let mongo_user = env::var("MONGO_DB_USER").expect("MONGO_DB_USER is not found.");
    let mongo_pass = env::var("MONGO_DB_PASS").expect("MONGO_DB_PASS is not found.");
    let mongo_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not found.");
    let mongo_uri = format!("mongodb://{mongo_user}:{mongo_pass}@{mongo_host}:{mongo_port}");

    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(mongo_name.as_str());

    Ok(database)
}
