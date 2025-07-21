use crate::models::project::{Project, ProjectDocument, ProjectInput, ProjectsInput};
use crate::routes::traits::ProjectRepository;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Database,
};
use rocket::serde::json::Json;

pub struct ProjectRepo {
    pub db: Database,
}

impl ProjectRepo {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepo {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Project>> {
        let collection = self.db.collection::<ProjectDocument>("project");

        let filter = doc! { "customer_id": oid };

        let find_options = FindOptions::builder()
            .sort(doc! { "order": 1 })
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = collection.find(filter, find_options).await?;

        let mut resp: Vec<Project> = vec![];
        while let Some(result) = cursor.try_next().await? {
            // transform ObjectId to String
            let json_resp = Project {
                id: result.id.to_string(),
                customer_id: result.customer_id.to_string(),
                name: result.name,
                description: result.description,
                link: result.link,
                photo_link: result.photo_link,
                order: result.order,
                stack: result.stack,
                created_at: result.created_at.to_string(),
            };
            resp.push(json_resp);
        }

        Ok(resp)
    }

    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Project>> {
        let collection = self.db.collection::<ProjectDocument>("project");

        let Some(result) = collection.find_one(doc! {"_id":oid }, None).await? else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Project {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            description: result.description,
            link: result.link,
            photo_link: result.photo_link,
            order: result.order,
            stack: result.stack,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn insert(
        &self,
        input: Json<ProjectInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String> {
        let collection = self.db.collection::<Document>("project");

        let created_at = mongodb::bson::DateTime::from_chrono(Utc::now());

        let resp = collection
            .insert_one(
                doc! {
                    "customer_id": oid,
                    "name": input.name.clone(),
                    "description": input.description.clone(),
                    "link": input.link.clone(),
                    "photo_link": input.photo_link.clone(),
                    "order": input.order,
                    "stack": input.stack.clone(),
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
        input: Json<ProjectInput>,
    ) -> mongodb::error::Result<Option<Project>> {
        let collection = self.db.collection::<ProjectDocument>("project");
        let find_one_and_update_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let Some(result) = collection
            .find_one_and_update(
                doc! {"_id":oid },
                doc! {
                    "$set": {
                        "name": input.name.clone(),
                        "description": input.description.clone(),
                        "link": input.link.clone(),
                        "photo_link": input.photo_link.clone(),
                        "order": input.order,
                        "stack": input.stack.clone(),
                    }
                },
                find_one_and_update_options,
            )
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Project {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            description: result.description,
            link: result.link,
            photo_link: result.photo_link,
            order: result.order,
            stack: result.stack,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }

    async fn update_many(
        &self,
        input: Json<Vec<ProjectsInput>>,
    ) -> mongodb::error::Result<Option<Vec<Project>>> {
        let collection = self.db.collection::<ProjectDocument>("project");
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
                            "description": item.description.clone(),
                            "link": item.link.clone(),
                            "photo_link": item.photo_link.clone(),
                            "order": item.order,
                            "stack": item.stack.clone(),
                        }
                    },
                    find_one_and_update_options,
                )
                .await?
            {
                updated_projects.push(Project {
                    id: result.id.to_string(),
                    customer_id: result.customer_id.to_string(),
                    name: result.name,
                    description: result.description,
                    link: result.link,
                    photo_link: result.photo_link,
                    order: result.order,
                    stack: result.stack,
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

    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Project>> {
        let collection = self.db.collection::<ProjectDocument>("project");

        // if you just unwrap,, when there is no document it results in 500 error.
        let Some(result) = collection
            .find_one_and_delete(doc! {"_id":oid }, None)
            .await?
        else {
            return Ok(None);
        };

        // transform ObjectId to String
        let resp = Project {
            id: result.id.to_string(),
            customer_id: result.customer_id.to_string(),
            name: result.name,
            description: result.description,
            link: result.link,
            photo_link: result.photo_link,
            order: result.order,
            stack: result.stack,
            created_at: result.created_at.to_string(),
        };

        Ok(Some(resp))
    }
}
