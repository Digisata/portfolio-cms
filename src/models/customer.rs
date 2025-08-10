use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerDocument {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: ObjectId,
    /// customer name
    pub api_key: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub wa_link: Option<String>,
    pub intro: Option<String>,
    pub about: Option<String>,
    pub profile_picture: Option<String>,
    pub password: String,
    /// createdAt
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        rename = "createdAt"
    )]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Customer {
    /// Document Id
    #[serde(rename = "_id")]
    pub id: String,
    /// customer name
    pub api_key: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub wa_link: Option<String>,
    pub intro: Option<String>,
    pub about: Option<String>,
    pub profile_picture: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
    /// createdAt
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CustomerInput {
    /// customer name
    pub api_key: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub wa_link: Option<String>,
    pub intro: Option<String>,
    pub about: Option<String>,
    pub profile_picture: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CustomerUpdateInput {
    /// customer name
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub wa_link: Option<String>,
    pub intro: Option<String>,
    pub about: Option<String>,
    pub profile_picture: Option<String>,
}
