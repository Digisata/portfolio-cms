use std::sync::Arc;

use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    errors::response::MyError,
    models::{
        auth::{LoginInput, LoginResponse},
        customer::CustomerInput,
        response::MessageResponse,
    },
    request_guards::basic::Claims,
};

use super::traits::CustomerRepository;

#[openapi(tag = "Auth")]
#[post("/login", data = "<input>")]
pub async fn login(
    container: &State<crate::Container>,
    input: Json<LoginInput>,
) -> Result<Json<LoginResponse>, MyError> {
    let email = input.email.clone();

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.find_customer_by_email(email.clone()).await {
        Ok(Some(customer_doc)) => {
            // Verify bcrypt password
            let password_matches = verify(&input.password, &customer_doc.password)
                .map_err(|_| MyError::build(400, Some("Incorrect email or password".into())))?;

            if !password_matches {
                return Err(MyError::build(
                    400,
                    Some("Incorrect email or password".into()),
                ));
            }

            // Build JWT claims
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: customer_doc.id.to_string(), // or email if you prefer
                exp: expiration as usize,
            };

            // Sign JWT
            let secret = std::env::var("JWT_SECRET").expect("env.JWT_SECRET is not set");
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .map_err(|_| MyError::build(500, Some("Failed to generate token".into())))?;

            Ok(Json(LoginResponse { jwt: token }))
        }

        // Either not found or error
        Ok(None) | Err(_) => Err(MyError::build(
            400,
            Some("Incorrect email or password".to_string()),
        )),
    }
}

#[openapi(tag = "Auth")]
#[post("/register", data = "<input>")]
pub async fn register(
    container: &State<crate::Container>,
    input: Json<LoginInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    let email = input.email.clone();

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| {
            BadRequest(Json(MessageResponse {
                message: "Service not found".to_string(),
            }))
        })?;

    match customer_repo.find_customer_by_email(email.clone()).await {
        Ok(Some(customer_doc)) => Err(BadRequest(Json(MessageResponse {
            message: format!("Email {} already exist", customer_doc.email),
        }))),

        // Either not found or error
        Ok(None) => {
            let customer_input = CustomerInput {
                name: "".to_string(),
                email: input.email.clone(),
                phone: "".to_string(),
                wa_link: "".to_string(),
                intro: "".to_string(),
                about: "".to_string(),
                profile_picture: "".to_string(),
                password: input.password.clone(),
            };

            match customer_repo.insert_customer(Json(customer_input)).await {
                Ok(customer_doc_id) => Ok(Json(customer_doc_id)),
                Err(_error) => Err(BadRequest(Json(MessageResponse {
                    message: "Invalid input".to_string(),
                }))),
            }
        }

        Err(err) => Err(BadRequest(Json(MessageResponse {
            message: format!("{err}"),
        }))),
    }
}
