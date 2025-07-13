use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    db::experience,
    errors::response::MyError,
    models::{
        experience::{Experience, ExperienceInput, ExperiencesInput},
        response::MessageResponse,
    },
    request_guards::basic::ApiKey,
};

#[openapi(tag = "Experience")]
#[get("/experience?<limit>&<page>")]
pub async fn get(
    db: &State<Database>,
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
    match experience::find(db, limit, page).await {
        Ok(resp) => Ok(Json(resp)),
        Err(error) => Err(MyError::build(400, Some(error.to_string()))),
    }
}

#[openapi(tag = "Experience")]
#[get("/experience/<id>")]
pub async fn get_by_id(
    db: &State<Database>,
    id: &str,
) -> Result<Json<Experience>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    };

    match experience::find_by_id(db, oid).await {
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
    db: &State<Database>,
    _key: ApiKey,
    input: Json<ExperienceInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match experience::insert(db, input).await {
        Ok(resp) => Ok(Json(resp)),
        Err(_error) => Err(BadRequest(Json(MessageResponse {
            message: "Invalid input".to_string(),
        }))),
    }
}

#[openapi(tag = "Experience")]
#[patch("/experience/<id>", data = "<input>")]
pub async fn patch_by_id(
    db: &State<Database>,
    _key: ApiKey,
    id: &str,
    input: Json<ExperienceInput>,
) -> Result<Json<Experience>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    match experience::update_by_id(db, oid, input).await {
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
    db: &State<Database>,
    _key: ApiKey,
    input: Json<Vec<ExperiencesInput>>,
) -> Result<Json<Vec<Experience>>, MyError> {
    match experience::update_many(db, input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some("Failed to update".to_string()),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some("Failed to update".to_string()),
        )),
    }
}

#[openapi(tag = "Experience")]
#[delete("/experience/<id>")]
pub async fn delete_by_id(
    db: &State<Database>,
    id: &str,
    _key: ApiKey,
) -> Result<Json<Experience>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    match experience::delete_by_id(db, oid).await {
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
