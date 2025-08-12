use crate::models::experience::{
    Experience, ExperienceDocument, ExperienceInput, ExperiencesInput,
};
use crate::routes::traits::ExperienceRepository;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Database,
};
use rocket::serde::json::Json;

pub struct ExperienceRepo {
    pub db: Database,
}

impl ExperienceRepo {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ExperienceRepository for ExperienceRepo {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Experience>> {
        let collection = self.db.collection::<ExperienceDocument>("experience");

        let filter = doc! { "customer_id": oid };

        let find_options = FindOptions::builder()
            .sort(doc! { "order": -1 })
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = collection.find(filter, find_options).await?;
        let mut resp: Vec<Experience> = vec![];

        while let Some(result) = cursor.try_next().await? {
            let json_resp = Experience {
                id: result.id.to_string(),
                customer_id: result.customer_id.to_string(),
                company: result.company,
                work_type: result.work_type,
                location: result.location,
                start_date: result.start_date.to_string(),
                end_date: Option::from(result.end_date.map(|d| d.to_string()).unwrap_or_default()),
                position: result.position,
                description: result.description,
                order: result.order,
                created_at: result.created_at.to_string(),
            };

            resp.push(json_resp);
        }

        Ok(resp)
    }

    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Experience>> {
        let collection = self.db.collection::<ExperienceDocument>("experience");

        let Some(result) = collection.find_one(doc! {"_id":oid }, None).await? else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Experience {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            company: result.company,
            work_type: result.work_type,
            location: result.location,
            start_date: result.start_date.to_string(),
            end_date: Option::from(result.end_date.map(|d| d.to_string()).unwrap_or_default()),
            position: result.position,
            description: result.description,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn insert(
        &self,
        input: Json<ExperienceInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String> {
        let collection = self.db.collection::<Document>("experience");

        let created_at = mongodb::bson::DateTime::from_chrono(Utc::now());
        let start_date = mongodb::bson::DateTime::parse_rfc3339_str(&input.start_date).unwrap();
        let end_date = match &input.end_date {
            Some(s) => Some(mongodb::bson::DateTime::parse_rfc3339_str(s).unwrap()),
            None => None,
        };

        let resp = collection
            .insert_one(
                doc! {
                    "customer_id": oid,
                    "company": &input.company,
                    "work_type": &input.work_type,
                    "location": &input.location,
                    "start_date": start_date,
                    "end_date": end_date,
                    "position": &input.position,
                    "description": &input.description,
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
        input: Json<ExperienceInput>,
    ) -> mongodb::error::Result<Option<Experience>> {
        let collection = self.db.collection::<ExperienceDocument>("experience");
        let find_one_and_update_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let start_date = mongodb::bson::DateTime::parse_rfc3339_str(&input.start_date).unwrap();
        let end_date = match &input.end_date {
            Some(s) => Some(mongodb::bson::DateTime::parse_rfc3339_str(s).unwrap()),
            None => None,
        };

        let Some(result) = collection
            .find_one_and_update(
                doc! {"_id":oid },
                doc! {
                    "$set": {
                        "company": &input.company,
                        "work_type": &input.work_type,
                        "location": &input.location,
                        "start_date": start_date,
                        "end_date": end_date,
                        "position": &input.position,
                        "description": &input.description,
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
        let resp = Experience {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            company: result.company,
            work_type: result.work_type,
            location: result.location,
            start_date: result.start_date.to_string(),
            end_date: Option::from(result.end_date.map(|d| d.to_string()).unwrap_or_default()),
            position: result.position,
            description: result.description,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn update_many(
        &self,
        input: Json<Vec<ExperiencesInput>>,
    ) -> mongodb::error::Result<Option<Vec<Experience>>> {
        let collection = self.db.collection::<ExperienceDocument>("experience");
        let mut updated_projects = vec![];

        for item in input.iter() {
            let oid = match ObjectId::parse_str(&item.id) {
                Ok(oid) => oid,
                Err(_) => continue, // skip invalid IDs
            };

            let find_one_and_update_options = FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build();

            let start_date = mongodb::bson::DateTime::parse_rfc3339_str(&item.start_date).unwrap();
            let end_date = match &item.end_date {
                Some(s) => Some(mongodb::bson::DateTime::parse_rfc3339_str(s).unwrap()),
                None => None,
            };

            if let Some(result) = collection
                .find_one_and_update(
                    doc! { "_id": oid },
                    doc! {
                        "$set": {
                            "company": &item.company,
                            "work_type": &item.work_type,
                            "location": &item.location,
                            "start_date": start_date,
                            "end_date": end_date,
                            "position": &item.position,
                            "description": &item.description,
                            "order": item.order,
                        }
                    },
                    find_one_and_update_options,
                )
                .await?
            {
                updated_projects.push(Experience {
                    id: result.id.to_string(),
                    customer_id: result.customer_id.to_string(),
                    company: result.company,
                    work_type: result.work_type,
                    location: result.location,
                    start_date: result.start_date.to_string(),
                    end_date: Option::from(
                        result.end_date.map(|d| d.to_string()).unwrap_or_default(),
                    ),
                    position: result.position,
                    description: result.description,
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

    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Experience>> {
        let collection = self.db.collection::<ExperienceDocument>("experience");

        // if you just unwrap,, when there is no document it results in 500 error.
        let Some(result) = collection
            .find_one_and_delete(doc! {"_id":oid }, None)
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Experience {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            company: result.company,
            work_type: result.work_type,
            location: result.location,
            start_date: result.start_date.to_string(),
            end_date: Option::from(result.end_date.map(|d| d.to_string()).unwrap_or_default()),
            position: result.position,
            description: result.description,
            order: result.order,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }
}
