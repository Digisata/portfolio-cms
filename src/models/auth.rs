use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Responder, Debug, Deserialize, Serialize, JsonSchema)]
pub struct LoginResponse {
    pub jwt: String,
}
