use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillDocument {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub customer_id: ObjectId,
    pub name: String,
    pub order: i32,
    /// createdAt
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        rename = "created_at"
    )]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Skill {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: String,
    pub customer_id: String,
    pub name: String,
    pub order: i32,
    /// createdAt
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct SkillInput {
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct SkillsInput {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub order: i32,
}
