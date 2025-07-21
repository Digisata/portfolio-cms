use std::sync::Arc;

use mongodb::bson::{doc, oid::ObjectId};
use rocket::{serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    errors::response::MyError,
    models::customer::{Customer, CustomerUpdateInput},
    request_guards::basic::{ApiKey, ClientApiKey},
};

use super::traits::CustomerRepository;

// get customer documents
// #[openapi(tag = "Customer")]
// #[get("/customer?<limit>&<page>")]
// pub async fn get_customers(
//     container: &State<crate::Container>,
//     limit: Option<i64>,
//     _key: ApiKey,
//     page: Option<i64>,
// ) -> Result<Json<Vec<Customer>>, MyError> {
//     // Error handling
//     // This is also valid when strict checking is necessary.
//     // if limit < 0 {
//     //     return Err(BadRequest(Some(Json(MessageResponse {
//     //         message: "limit cannot be less than 0".to_string(),
//     //     }))));
//     // }
//     // if !page.is_none() && page.unwrap() < 1 {
//     //     return Err(BadRequest(Some(Json(MessageResponse {
//     //         message: "page cannot be less than 1".to_string(),
//     //     }))));
//     // }
//
//     // Setting default values
//     let limit: i64 = limit.unwrap_or(100);
//     let page: i64 = page.unwrap_or(1);
//
//     let customer_repo = container
//         .get::<Arc<dyn CustomerRepository + Send + Sync>>()
//         .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;
//
//     match customer_repo.find_customer(limit, page).await {
//         Ok(customer_docs) => Ok(Json(customer_docs)),
//         Err(error) => Err(MyError::build(400, Some(error.to_string()))),
//     }
// }

/// get customer document by _id
#[openapi(tag = "Customer")]
#[get("/customer")]
pub async fn get_customer_by_id(
    container: &State<crate::Container>,
    key: ApiKey,
) -> Result<Json<Customer>, MyError> {
    let claims = &key.0; // Access the Claims struct
    let id = &claims.sub;
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(
            400,
            Some("Invalid user id format.".to_string()),
        ));
    };

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.find_customer_by_id(oid).await {
        Ok(customer_doc) => match customer_doc {
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {id}")),
            )),
            Some(customer_doc) => Ok(Json(customer_doc)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Customer not found with _id {id}")),
        )),
    }
}

#[openapi(tag = "Customer")]
#[get("/customer/profile")]
pub async fn get_customer_profile(
    container: &State<crate::Container>,
    client_key: ClientApiKey,
) -> Result<Json<Customer>, MyError> {
    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    match customer_repo.find_customer_by_api_key(&client_key.0).await {
        Ok(customer_doc) => match customer_doc {
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with api key {}", client_key.0)),
            )),
            Some(customer_doc) => Ok(Json(customer_doc)),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Customer not found with api key {}", client_key.0)),
        )),
    }
}

// create a customer document
// #[openapi(tag = "Customer")]
// #[post("/customer", data = "<input>")]
// pub async fn post_customer(
//     container: &State<crate::Container>,
//     _key: ApiKey,
//     input: Json<CustomerInput>,
// ) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
//     let customer_repo = container
//         .get::<Arc<dyn CustomerRepository + Send + Sync>>()
//         .ok_or_else(|| {
//             BadRequest(Json(MessageResponse {
//                 message: "Service not found".to_string(),
//             }))
//         })?;
//
//     match customer_repo.insert_customer(input).await {
//         Ok(customer_doc_id) => Ok(Json(customer_doc_id)),
//         Err(_error) => Err(BadRequest(Json(MessageResponse {
//             message: "Invalid input".to_string(),
//         }))),
//     }
// }

/// update a customer document by _id
#[openapi(tag = "Customer")]
#[patch("/customer", data = "<input>")]
pub async fn patch_customer_by_id(
    container: &State<crate::Container>,
    key: ApiKey,
    input: Json<CustomerUpdateInput>,
) -> Result<Json<Customer>, MyError> {
    let claims = &key.0; // Access the Claims struct
    let id = &claims.sub;
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(MyError::build(
            400,
            Some("Invalid user id format.".to_string()),
        ));
    };

    let customer_repo = container
        .get::<Arc<dyn CustomerRepository + Send + Sync>>()
        .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;

    // First, try to find customer by customer_id (from JWT)
    match customer_repo.find_customer_by_id(oid).await {
        Ok(customer_doc) => match customer_doc {
            // Customer found by customer_id, proceed with update
            Some(_customer_doc) => {
                let email = &input.email;
                match &_customer_doc.email {
                    name if name == email => {
                        match customer_repo.update_customer_by_id(oid, input).await {
                            Ok(updated_customer) => match updated_customer {
                                Some(updated_customer) => Ok(Json(updated_customer)),
                                None => Err(MyError::build(
                                    400,
                                    Some(format!("Customer not found with id {oid}")),
                                )),
                            },
                            Err(_error) => Err(MyError::build(
                                400,
                                Some(format!("Failed to update customer with id {oid}")),
                            )),
                        }
                    }
                    _ => match customer_repo
                        .find_customer_by_email(email.to_string())
                        .await
                    {
                        Ok(email_customer) => match email_customer {
                            // Email already exists, return error
                            Some(_existing_customer) => Err(MyError::build(
                                400,
                                Some(format!("Customer with email {email} already exists")),
                            )),
                            // Email doesn't exist, proceed with update logic
                            None => match customer_repo.update_customer_by_id(oid, input).await {
                                Ok(updated_customer) => match updated_customer {
                                    Some(updated_customer) => Ok(Json(updated_customer)),
                                    None => Err(MyError::build(
                                        400,
                                        Some(format!("Customer not found with id {oid}")),
                                    )),
                                },
                                Err(_error) => Err(MyError::build(
                                    400,
                                    Some(format!("Failed to update customer with id {oid}")),
                                )),
                            },
                        },
                        Err(_error) => Err(MyError::build(
                            400,
                            Some(format!("Failed to check email {email}")),
                        )),
                    },
                }
            }
            // Customer not found by customer_id, check by email
            None => Err(MyError::build(
                400,
                Some(format!("Customer not found with id {}", &id)),
            )),
        },
        Err(_error) => Err(MyError::build(
            400,
            Some(format!("Failed to find customer with customer_id {id}")),
        )),
    }
}

// delete a customer document by _id
// #[openapi(tag = "Customer")]
// #[delete("/customer/<id>")]
// pub async fn delete_customer_by_id(
//     container: &State<crate::Container>,
//     id: &str,
//     _key: ApiKey,
// ) -> Result<Json<Customer>, MyError> {
//     let Ok(oid) = ObjectId::parse_str(id) else {
//         return Err(MyError::build(400, Some("Invalid id format.".to_string())));
//     };
//
//     let customer_repo = container
//         .get::<Arc<dyn CustomerRepository + Send + Sync>>()
//         .ok_or_else(|| MyError::build(500, Some("Service not found".to_string())))?;
//
//     match customer_repo.delete_customer_by_id(oid).await {
//         Ok(customer_doc) => match customer_doc {
//             Some(customer_doc) => Ok(Json(customer_doc)),
//             None => Err(MyError::build(
//                 400,
//                 Some(format!("Customer not found with _id {}", &id)),
//             )),
//         },
//         Err(_error) => Err(MyError::build(
//             400,
//             Some(format!("Customer not found with _id {}", &id)),
//         )),
//     }
// }
