use crate::models::social::{Social, SocialDocument, SocialInput, SocialsInput};
use crate::routes::traits::SocialRepository;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Database,
};
use rocket::serde::json::Json;

pub struct SocialRepo {
    pub db: Database,
}

impl SocialRepo {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SocialRepository for SocialRepo {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Social>> {
        let collection = self.db.collection::<SocialDocument>("social");

        let filter = doc! { "customer_id": oid };

        let find_options = FindOptions::builder()
            .sort(doc! { "order": -1 })
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = collection.find(filter, find_options).await?;

        let mut resp: Vec<Social> = vec![];
        while let Some(result) = cursor.try_next().await? {
            // transform ObjectId to String
            let json_resp = Social {
                id: result.id.to_string(),
                customer_id: result.customer_id.to_string(),
                name: result.name,
                link: result.link,
                order: result.order,
                created_at: result.created_at.to_string(),
            };
            resp.push(json_resp);
        }

        Ok(resp)
    }

    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Social>> {
        let collection = self.db.collection::<SocialDocument>("social");

        let Some(result) = collection.find_one(doc! {"_id":oid }, None).await? else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Social {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            link: result.link,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn insert(
        &self,
        input: Json<SocialInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String> {
        let collection = self.db.collection::<Document>("social");

        let created_at = mongodb::bson::DateTime::from_chrono(Utc::now());

        let resp = collection
            .insert_one(
                doc! {
                    "customer_id": oid,
                    "name": &input.name,
                    "link": &input.link,
                    "order": input.order,
                    "created_at": created_at,
                },
                None,
            )
            .await?;

        Ok(resp.inserted_id.to_string())
    }

    async fn update_by_id(
        &self,
        oid: ObjectId,
        input: Json<SocialInput>,
    ) -> mongodb::error::Result<Option<Social>> {
        let collection = self.db.collection::<SocialDocument>("social");
        let find_one_and_update_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let Some(result) = collection
            .find_one_and_update(
                doc! {"_id":oid },
                doc! {
                    "$set": {
                        "name": &input.name,
                        "link": &input.link,
                        "order": input.order,
                    }
                },
                find_one_and_update_options,
            )
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Social {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            link: result.link,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn update_many(
        &self,
        input: Json<Vec<SocialsInput>>,
    ) -> mongodb::error::Result<Option<Vec<Social>>> {
        let collection = self.db.collection::<SocialDocument>("social");
        let mut updated_projects = vec![];

        for item in input.iter() {
            let oid = match ObjectId::parse_str(&item.id) {
                Ok(oid) => oid,
                Err(_) => continue, // skip invalid IDs
            };

            let find_one_and_update_options = FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build();

            if let Some(result) = collection
                .find_one_and_update(
                    doc! { "_id": oid },
                    doc! {
                        "$set": {
                            "name": &item.name,
                            "link": &item.link,
                            "order": item.order,
                        }
                    },
                    find_one_and_update_options,
                )
                .await?
            {
                updated_projects.push(Social {
                    id: result.id.to_string(),
                    customer_id: result.customer_id.to_string(),
                    name: result.name,
                    link: result.link,
                    order: result.order,
                    created_at: result.created_at.to_string(),
                });
            }
        }

        if updated_projects.is_empty() {
            Ok(None)
        } else {
            Ok(Some(updated_projects))
        }
    }

    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Social>> {
        let collection = self.db.collection::<SocialDocument>("social");

        // if you just unwrap,, when there is no document it results in 500 error.
        let Some(result) = collection
            .find_one_and_delete(doc! {"_id":oid }, None)
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Social {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            link: result.link,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }
}
