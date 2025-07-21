use std::sync::Arc;

use mongodb::bson::{doc, oid::ObjectId};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use super::traits::{CustomerRepository, ExperienceRepository};
use crate::{
    errors::response::MyError,
    models::{
        experience::{Experience, ExperienceInput, ExperiencesInput},
        response::MessageResponse,
    },
    request_guards::basic::{ApiKey, ClientApiKey},
};

#[openapi(tag = "Experience")]
#[get("/experience?<limit>&<page>")]
pub async fn get_all(
    container: &State<crate::Container>,
    key: ApiKey,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<Experience>>, MyError> {
    // Error handling
    // This is also valid when strict checking is necessary.
    // if limit < 0 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "limit cannot be less than 0".to_string(),
    //     }))));
    // }
    // if !page.is_none() && page.unwrap() < 1 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "page cannot be less than 1".to_string(),
    //     }))));
    // }

    let claims = &key.0; // Access the Claims struct
    let id = &claims.sub;
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(
            400,
            Some("Invalid user id format.".to_string()),
        ));
    };
    // Setting default values
    let limit: i64 = limit.unwrap_or(100);
    let page: i64 = page.unwrap_or(1);

    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match experience_repo.find(limit, page, oid).await {
        Ok(resp) => Ok(Json(resp)),
        Err(error) => Err(MyError::build(400, Some(error.to_string()))),
    }
}

#[openapi(tag = "Experience")]
#[get("/experiences?<limit>&<page>")]
pub async fn get(
    container: &State<crate::Container>,
    client_key: ClientApiKey,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<Experience>>, MyError> {
    // Error handling
    // This is also valid when strict checking is necessary.
    // if limit < 0 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "limit cannot be less than 0".to_string(),
    //     }))));
    // }
    // if !page.is_none() && page.unwrap() < 1 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "page cannot be less than 1".to_string(),
    //     }))));
    // }

    // Setting default values
    let limit: i64 = limit.unwrap_or(100);
    let page: i64 = page.unwrap_or(1);

    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.find_customer_by_api_key(&client_key.0).await {
        Ok(Some(customer_doc)) => {
            let Ok(oid) = ObjectId::parse_str(customer_doc.id) else {
                return Err(MyError::build(
                    400,
                    Some("Invalid user id format.".to_string()),
                ));
            };

            match experience_repo.find(limit, page, oid).await {
                Ok(resp) => Ok(Json(resp)),
                Err(error) => Err(MyError::build(400, Some(error.to_string()))),
            }
        }

        // Either not found or error
        Ok(None) | Err(_) => Err(MyError::build(
            400,
            Some("Incorrect email or password".to_string()),
        )),
    }
}

#[openapi(tag = "Experience")]
#[get("/experience/<id>")]
pub async fn get_by_id(
    container: &State<crate::Container>,
    _key: ApiKey,
    id: &str,
) -> Result<Json<Experience>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    };

    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match experience_repo.find_by_id(oid).await {
        Ok(resp) => match resp {
            None => Err(MyError::build(
                400,
                Some(format!("Experience not found with _id {}", &id)),
            )),
            Some(resp) => Ok(Json(resp)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Experience not found with _id {}", &id)),
        )),
    }
}

#[openapi(tag = "Experience")]
#[post("/experience", data = "<input>")]
pub async fn post(
    container: &State<crate::Container>,
    key: ApiKey,
    input: Json<ExperienceInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| {
            BadRequest(Json(MessageResponse {
                message: "Service not found".to_string(),
            }))
        })?;

    let claims = &key.0; // Access the Claims struct
    let id = &claims.sub;
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(BadRequest(Json(MessageResponse {
            message: "Invalid user id format".to_string(),
        })));
    };

    match experience_repo.insert(input, oid).await {
        Ok(resp) => Ok(Json(resp)),
        Err(_error) => Err(BadRequest(Json(MessageResponse {
            message: "Invalid input".to_string(),
        }))),
    }
}

#[openapi(tag = "Experience")]
#[patch("/experience/<id>", data = "<input>")]
pub async fn patch_by_id(
    container: &State<crate::Container>,
    _key: ApiKey,
    id: &str,
    input: Json<ExperienceInput>,
) -> Result<Json<Experience>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match experience_repo.update_by_id(oid, input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some(format!("Experience not found with id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Experience not found with id {}", &id)),
        )),
    }
}

#[openapi(tag = "Experience")]
#[patch("/experience", data = "<input>")]
pub async fn patch_many(
    container: &State<crate::Container>,
    _key: ApiKey,
    input: Json<Vec<ExperiencesInput>>,
) -> Result<Json<Vec<Experience>>, MyError> {
    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match experience_repo.update_many(input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(400, Some("Failed to update".to_string()))),
        },
        Err(_error) => Err(MyError::build(400, Some("Failed to update".to_string()))),
    }
}

#[openapi(tag = "Experience")]
#[delete("/experience/<id>")]
pub async fn delete_by_id(
    container: &State<crate::Container>,
    id: &str,
    _key: ApiKey,
) -> Result<Json<Experience>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    let experience_repo = container
        .get::<Arc<dyn ExperienceRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match experience_repo.delete_by_id(oid).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some(format!("Experience not found with _id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Experience not found with _id {}", &id)),
        )),
    }
}
