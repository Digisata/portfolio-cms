use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExperienceDocument {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub customer_id: ObjectId,
    pub company: String,
    pub work_type: String,
    pub location: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub end_date: DateTime<Utc>,
    pub is_present: bool,
    pub position: String,
    pub description: Vec<String>,
    pub order: i32,
    /// createdAt
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        rename = "created_at"
    )]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Experience {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: String,
    pub customer_id: String,
    pub company: String,
    pub work_type: String,
    pub location: String,
    pub start_date: String,
    pub end_date: String,
    pub is_present: bool,
    pub position: String,
    pub description: Vec<String>,
    pub order: i32,
    /// createdAt
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct ExperienceInput {
    pub company: String,
    pub work_type: String,
    pub location: String,
    pub start_date: String,
    pub end_date: String,
    pub is_present: bool,
    pub position: String,
    pub description: Vec<String>,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct ExperiencesInput {
    #[serde(rename = "_id")]
    pub id: String,
    pub company: String,
    pub work_type: String,
    pub location: String,
    pub start_date: String,
    pub end_date: String,
    pub is_present: bool,
    pub position: String,
    pub description: Vec<String>,
    pub order: i32,
}
