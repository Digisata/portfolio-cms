use crate::models::customer::{Customer, CustomerDocument, CustomerInput};
use crate::routes::traits::CustomerRepository;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Document},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Database,
};
use rocket::serde::json::Json;

pub struct CustomerRepo {
    pub db: Database,
}

impl CustomerRepo {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CustomerRepository for CustomerRepo {
    async fn find_customer(&self, limit: i64, page: i64) -> mongodb::error::Result<Vec<Customer>> {
        let collection = self.db.collection::<CustomerDocument>("customer");

        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = collection.find(None, find_options).await?;

        let mut customers: Vec<Customer> = vec![];
        while let Some(result) = cursor.try_next().await? {
            // transform ObjectId to String
            let customer_json = Customer {
                id: result.id.to_string(),
                name: result.name.to_string(),
                email: result.email.to_string(),
                phone: result.phone.to_string(),
                wa_link: result.wa_link.to_string(),
                intro: result.intro.to_string(),
                about: result.about.to_string(),
                profile_picture: result.profile_picture.to_string(),
                password: result.password.to_string(),
                created_at: result.created_at.to_string(),
            };
            customers.push(customer_json);
        }

        Ok(customers)
    }

    async fn find_customer_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Customer>> {
        let collection = self.db.collection::<CustomerDocument>("customer");

        let Some(customer_doc) = collection.find_one(doc! {"_id":oid }, None).await? else {
            return Ok(None);
        };

        // transform ObjectId to String
        let customer_json = Customer {
            id: customer_doc.id.to_string(),
            name: customer_doc.name.to_string(),
            email: customer_doc.email.to_string(),
            phone: customer_doc.phone.to_string(),
            wa_link: customer_doc.wa_link.to_string(),
            intro: customer_doc.intro.to_string(),
            about: customer_doc.about.to_string(),
            profile_picture: customer_doc.profile_picture.to_string(),
            password: customer_doc.password.to_string(),
            created_at: customer_doc.created_at.to_string(),
        };

        Ok(Some(customer_json))
    }

    async fn find_customer_by_email(
        &self,
        email: String,
    ) -> mongodb::error::Result<Option<Customer>> {
        let collection = self.db.collection::<CustomerDocument>("customer");

        let Some(customer_doc) = collection.find_one(doc! {"email":email }, None).await? else {
            return Ok(None);
        };

        // transform ObjectId to String
        let customer_json = Customer {
            id: customer_doc.id.to_string(),
            name: customer_doc.name.to_string(),
            email: customer_doc.email.to_string(),
            phone: customer_doc.phone.to_string(),
            wa_link: customer_doc.wa_link.to_string(),
            intro: customer_doc.intro.to_string(),
            about: customer_doc.about.to_string(),
            profile_picture: customer_doc.profile_picture.to_string(),
            password: customer_doc.password.to_string(),
            created_at: customer_doc.created_at.to_string(),
        };

        Ok(Some(customer_json))
    }

    async fn insert_customer(&self, input: Json<CustomerInput>) -> mongodb::error::Result<String> {
        let collection = self.db.collection::<Document>("customer");

        // Hash the password using bcrypt
        let hashed_password = match hash(&input.password, DEFAULT_COST) {
            Ok(h) => h,
            Err(e) => return Err(mongodb::error::Error::custom(e.to_string())),
        };

        let created_at = Utc::now();

        let insert_one_result = collection
            .insert_one(
                doc! {
                    "name": input.name.clone(),
                    "email": input.email.clone(),
                    "phone": input.phone.clone(),
                    "wa_link": input.wa_link.clone(),
                    "intro": input.intro.clone(),
                    "about": input.about.clone(),
                    "profile_picture": input.profile_picture.clone(),
                    "password": hashed_password,
                    "createdAt": created_at,
                },
                None,
            )
            .await?;

        Ok(insert_one_result.inserted_id.to_string())
    }

    async fn update_customer_by_id(
        &self,
        oid: ObjectId,
        input: Json<CustomerInput>,
    ) -> mongodb::error::Result<Option<Customer>> {
        let collection = self.db.collection::<CustomerDocument>("customer");
        let find_one_and_update_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let created_at: DateTime = DateTime::now();

        let Some(customer_doc) = collection
            .find_one_and_update(
                doc! {"_id":oid },
                doc! {
                        "$set": {
                            "name": input.name.clone(),
                            "email": input.email.clone(),
                            "phone": input.phone.clone(),
                            "wa_link": input.wa_link.clone(),
                            "intro": input.intro.clone(),
                            "about": input.about.clone(),
                            "profile_picture": input.profile_picture.clone(),
                            "createdAt": created_at,
                        }
                },
                find_one_and_update_options,
            )
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let customer_json = Customer {
            id: customer_doc.id.to_string(),
            name: customer_doc.name.to_string(),
            email: customer_doc.email.to_string(),
            phone: customer_doc.phone.to_string(),
            wa_link: customer_doc.wa_link.to_string(),
            intro: customer_doc.intro.to_string(),
            about: customer_doc.about.to_string(),
            profile_picture: customer_doc.profile_picture.to_string(),
            password: customer_doc.password.to_string(),
            created_at: customer_doc.created_at.to_string(),
        };

        Ok(Some(customer_json))
    }

    async fn delete_customer_by_id(
        &self,
        oid: ObjectId,
    ) -> mongodb::error::Result<Option<Customer>> {
        let collection = self.db.collection::<CustomerDocument>("customer");

        // if you just unwrap,, when there is no document it results in 500 error.
        let Some(customer_doc) = collection
            .find_one_and_delete(doc! {"_id":oid }, None)
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let customer_json = Customer {
            id: customer_doc.id.to_string(),
            name: customer_doc.name.to_string(),
            email: customer_doc.email.to_string(),
            phone: customer_doc.phone.to_string(),
            wa_link: customer_doc.wa_link.to_string(),
            intro: customer_doc.intro.to_string(),
            about: customer_doc.about.to_string(),
            profile_picture: customer_doc.profile_picture.to_string(),
            password: customer_doc.password.to_string(),
            created_at: customer_doc.created_at.to_string(),
        };

        Ok(Some(customer_json))
    }
}
