use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{bson::doc, Database};
use rocket::{serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    db::customer,
    errors::response::MyError,
    models::auth::{LoginInput, LoginResponse},
    request_guards::basic::Claims,
};

#[openapi(tag = "Auth")]
#[post("/login", data = "<input>")]
pub async fn login(
    db: &State<Database>,
    input: Json<LoginInput>,
) -> Result<Json<LoginResponse>, MyError> {
    let email = input.email.clone();

    match customer::find_customer_by_email(db, email.clone()).await {
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
