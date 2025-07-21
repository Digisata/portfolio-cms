use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDocument {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub customer_id: ObjectId,
    pub name: String,
    pub description: String,
    pub link: String,
    pub photo_link: String,
    pub order: i32,
    pub stack: Vec<String>,
    /// createdAt
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        rename = "created_at"
    )]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Project {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: String,
    pub customer_id: String,
    pub name: String,
    pub description: String,
    pub link: String,
    pub photo_link: String,
    pub order: i32,
    pub stack: Vec<String>,
    /// createdAt
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct ProjectInput {
    pub name: String,
    pub description: String,
    pub link: String,
    pub photo_link: String,
    pub order: i32,
    pub stack: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct ProjectsInput {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub description: String,
    pub link: String,
    pub photo_link: String,
    pub order: i32,
    pub stack: Vec<String>,
}
