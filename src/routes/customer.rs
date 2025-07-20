use std::sync::Arc;

use mongodb::bson::{doc, oid::ObjectId};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    errors::response::MyError,
    models::{
        customer::{Customer, CustomerInput, CustomerUpdateInput},
        response::MessageResponse,
    },
    request_guards::basic::ApiKey,
};

use super::traits::CustomerRepository;

/// get customer documents
#[openapi(tag = "Customer")]
#[get("/customer?<limit>&<page>")]
pub async fn get_customers(
    container: &State<crate::Container>,
    limit: Option<i64>,
    _key: ApiKey,
    page: Option<i64>,
) -> Result<Json<Vec<Customer>>, MyError> {
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

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.find_customer(limit, page).await {
        Ok(customer_docs) => Ok(Json(customer_docs)),
        Err(error) => Err(MyError::build(400, Some(error.to_string()))),
    }
}

/// get customer document by _id
#[openapi(tag = "Customer")]
#[get("/customer/<id>")]
pub async fn get_customer_by_id(
    container: &State<crate::Container>,
    _key: ApiKey,
    id: &str,
) -> Result<Json<Customer>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    };

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.find_customer_by_id(oid).await {
        Ok(customer_doc) => match customer_doc {
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &id)),
            )),
            Some(customer_doc) => Ok(Json(customer_doc)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Customer not found with _id {}", &id)),
        )),
    }
}

#[openapi(tag = "Customer")]
#[get("/customer/email/<email>")]
pub async fn get_customer_by_email(
    container: &State<crate::Container>,
    email: &str,
) -> Result<Json<Customer>, MyError> {
    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo
        .find_customer_by_email(email.to_string())
        .await
    {
        Ok(customer_doc) => match customer_doc {
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with email {}", email)),
            )),
            Some(customer_doc) => Ok(Json(customer_doc)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Customer not found with email {}", email)),
        )),
    }
}

/// create a customer document
#[openapi(tag = "Customer")]
#[post("/customer", data = "<input>")]
pub async fn post_customer(
    container: &State<crate::Container>,
    _key: ApiKey,
    input: Json<CustomerInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| {
            BadRequest(Json(MessageResponse {
                message: "Service not found".to_string(),
            }))
        })?;

    match customer_repo.insert_customer(input).await {
        Ok(customer_doc_id) => Ok(Json(customer_doc_id)),
        Err(_error) => Err(BadRequest(Json(MessageResponse {
            message: "Invalid input".to_string(),
        }))),
    }
}

/// update a customer document by _id
#[openapi(tag = "Customer")]
#[patch("/customer/<id>", data = "<input>")]
pub async fn patch_customer_by_id(
    container: &State<crate::Container>,
    _key: ApiKey,
    id: &str,
    input: Json<CustomerUpdateInput>,
) -> Result<Json<Customer>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.update_customer_by_id(oid, input).await {
        Ok(customer_doc) => match customer_doc {
            Some(customer_doc) => Ok(Json(customer_doc)),
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Customer not found with id {}", &id)),
        )),
    }
}

/// delete a customer document by _id
#[openapi(tag = "Customer")]
#[delete("/customer/<id>")]
pub async fn delete_customer_by_id(
    container: &State<crate::Container>,
    id: &str,
    _key: ApiKey,
) -> Result<Json<Customer>, MyError> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    };

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.delete_customer_by_id(oid).await {
        Ok(customer_doc) => match customer_doc {
            Some(customer_doc) => Ok(Json(customer_doc)),
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Customer not found with _id {}", &id)),
        )),
    }
}
