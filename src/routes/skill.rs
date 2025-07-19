use std::sync::Arc;

use super::traits::SkillRepository;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    errors::response::MyError,
    models::{
        response::MessageResponse,
        skill::{Skill, SkillInput, SkillsInput},
    },
    request_guards::basic::ApiKey,
};

#[openapi(tag = "Skill")]
#[get("/skill?<limit>&<page>")]
pub async fn get(
    container: &State<crate::Container>,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<Skill>>, MyError> {
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

    let skill_repo = container
        .get::<Arc<dyn SkillRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match skill_repo.find(limit, page).await {
        Ok(resp) => Ok(Json(resp)),
        Err(error) => Err(MyError::build(400, Some(error.to_string()))),
    }
}

#[openapi(tag = "Skill")]
#[get("/skill/<id>")]
pub async fn get_by_id(
    container: &State<crate::Container>,
    id: &str,
) -> Result<Json<Skill>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    };

    let skill_repo = container
        .get::<Arc<dyn SkillRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match skill_repo.find_by_id(oid).await {
        Ok(resp) => match resp {
            None => Err(MyError::build(
                400,
                Some(format!("Skill not found with _id {}", &id)),
            )),
            Some(resp) => Ok(Json(resp)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Skill not found with _id {}", &id)),
        )),
    }
}

#[openapi(tag = "Skill")]
#[post("/skill", data = "<input>")]
pub async fn post(
    container: &State<crate::Container>,
    _key: ApiKey,
    input: Json<SkillInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    let skill_repo = container
        .get::<Arc<dyn SkillRepository + Send + Sync>>()
        .ok_or_else(|| {
            BadRequest(Json(MessageResponse {
                message: "Service not found".to_string(),
            }))
        })?;

    // can set with a single error like this.
    match skill_repo.insert(input).await {
        Ok(resp) => Ok(Json(resp)),
        Err(_error) => Err(BadRequest(Json(MessageResponse {
            message: "Invalid input".to_string(),
        }))),
    }
}

#[openapi(tag = "Skill")]
#[patch("/skill/<id>", data = "<input>")]
pub async fn patch_by_id(
    container: &State<crate::Container>,
    _key: ApiKey,
    id: &str,
    input: Json<SkillInput>,
) -> Result<Json<Skill>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    let skill_repo = container
        .get::<Arc<dyn SkillRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match skill_repo.update_by_id(oid, input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some(format!("Skill not found with id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Skill not found with id {}", &id)),
        )),
    }
}

#[openapi(tag = "Skill")]
#[patch("/skill", data = "<input>")]
pub async fn patch_many(
    container: &State<crate::Container>,
    _key: ApiKey,
    input: Json<Vec<SkillsInput>>,
) -> Result<Json<Vec<Skill>>, MyError> {
    let skill_repo = container
        .get::<Arc<dyn SkillRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match skill_repo.update_many(input).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(400, Some("Failed to update".to_string()))),
        },
        Err(_error) => Err(MyError::build(400, Some("Failed to update".to_string()))),
    }
}

#[openapi(tag = "Skill")]
#[delete("/skill/<id>")]
pub async fn delete_by_id(
    container: &State<crate::Container>,
    id: &str,
    _key: ApiKey,
) -> Result<Json<Skill>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    let skill_repo = container
        .get::<Arc<dyn SkillRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match skill_repo.delete_by_id(oid).await {
        Ok(resp) => match resp {
            Some(resp) => Ok(Json(resp)),
            None => Err(MyError::build(
                400,
                Some(format!("Skill not found with _id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Skill not found with _id {}", &id)),
        )),
    }
}
