use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    db::social,
    errors::response::MyError,
    models::{
        response::MessageResponse,
        social::{Social, SocialInput, SocialsInput},
    },
    request_guards::basic::ApiKey,
};

#[openapi(tag = "Social")]
#[get("/social?<limit>&<page>")]
pub async fn get(
    db: &State<Database>,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<Social>>, MyError> {
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
    match social::find(db, limit, page).await {
        Ok(resp) => Ok(Json(resp)),
        Err(error) => Err(MyError::build(400, Some(error.to_string()))),
    }
}

#[openapi(tag = "Social")]
#[get("/social/<id>")]
pub async fn get_by_id(db: &State<Database>, id: &str) -> Result<Json<Social>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    };

    match social::find_by_id(db, oid).await {
        Ok(resp) => match resp {
            None => Err(MyError::build(
                400,
                Some(format!("Social not found with _id {}", &id)),
            )),
            Some(resp) => Ok(Json(resp)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Social not found with _id {}", &id)),
        )),
    }
}

#[openapi(tag = "Social")]
#[post("/social", data = "<input>")]
pub async fn post(
    db: &State<Database>,
    _key: ApiKey,
    input: Json<SocialInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match social::insert(db, input).await {
        Ok(resp) => Ok(Json(resp)),
        Err(_error) => Err(BadRequest(Json(MessageResponse {
            message: "Invalid input".to_string(),
        }))),
    }
}

#[openapi(tag = "Social")]
#[patch("/social/<id>", data = "<input>")]
pub async fn patch_by_id(
    db: &State<Database>,
    _key: ApiKey,
    id: &str,
    input: Json<SocialInput>,
) -> Result<Json<Social>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    match social::update_by_id(db, oid, input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some(format!("Social not found with id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Social not found with id {}", &id)),
        )),
    }
}

#[openapi(tag = "Social")]
#[patch("/social", data = "<input>")]
pub async fn patch_many(
    db: &State<Database>,
    _key: ApiKey,
    input: Json<Vec<SocialsInput>>,
) -> Result<Json<Vec<Social>>, MyError> {
    match social::update_many(db, input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(400, Some("Failed to update".to_string()))),
        },
        Err(_error) => Err(MyError::build(400, Some("Failed to update".to_string()))),
    }
}

#[openapi(tag = "Social")]
#[delete("/social/<id>")]
pub async fn delete_by_id(
    db: &State<Database>,
    id: &str,
    _key: ApiKey,
) -> Result<Json<Social>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    match social::delete_by_id(db, oid).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some(format!("Social not found with _id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Social not found with _id {}", &id)),
        )),
    }
}
