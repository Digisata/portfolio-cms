use crate::models::skill::{Skill, SkillDocument, SkillInput, SkillsInput};
use crate::routes::traits::SkillRepository;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Database,
};
use rocket::serde::json::Json;

pub struct SkillRepo {
    pub db: Database,
}

impl SkillRepo {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SkillRepository for SkillRepo {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Skill>> {
        let collection = self.db.collection::<SkillDocument>("skill");

        let filter = doc! { "customer_id": oid };

        let find_options = FindOptions::builder()
            .sort(doc! { "order": 1 })
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = collection.find(filter, find_options).await?;

        let mut resp: Vec<Skill> = vec![];
        while let Some(result) = cursor.try_next().await? {
            // transform ObjectId to String
            let json_resp = Skill {
                id: result.id.to_string(),
                customer_id: result.customer_id.to_string(),
                name: result.name,
                order: result.order,
                created_at: result.created_at.to_string(),
            };
            resp.push(json_resp);
        }

        Ok(resp)
    }

    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Skill>> {
        let collection = self.db.collection::<SkillDocument>("skill");

        let Some(result) = collection.find_one(doc! {"_id":oid }, None).await? else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Skill {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn insert(
        &self,
        input: Json<SkillInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String> {
        let collection = self.db.collection::<Document>("skill");

        let created_at = mongodb::bson::DateTime::from_chrono(Utc::now());

        let resp = collection
            .insert_one(
                doc! {
                    "customer_id": oid,
                    "name": input.name.clone(),
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
        input: Json<SkillInput>,
    ) -> mongodb::error::Result<Option<Skill>> {
        let collection = self.db.collection::<SkillDocument>("skill");
        let find_one_and_update_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let Some(result) = collection
            .find_one_and_update(
                doc! {"_id":oid },
                doc! {
                    "$set": {
                        "name": input.name.clone(),
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
        let resp = Skill {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn update_many(
        &self,
        input: Json<Vec<SkillsInput>>,
    ) -> mongodb::error::Result<Option<Vec<Skill>>> {
        let collection = self.db.collection::<SkillDocument>("skill");
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
                            "name": item.name.clone(),
                            "order": item.order,
                        }
                    },
                    find_one_and_update_options,
                )
                .await?
            {
                updated_projects.push(Skill {
                    id: result.id.to_string(),
                    customer_id: result.customer_id.to_string(),
                    name: result.name,
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

    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Skill>> {
        let collection = self.db.collection::<SkillDocument>("skill");

        // if you just unwrap,, when there is no document it results in 500 error.
        let Some(result) = collection
            .find_one_and_delete(doc! {"_id":oid }, None)
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Skill {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }
}
