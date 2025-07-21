use crate::models::{
    customer::{Customer, CustomerInput, CustomerUpdateInput},
    experience::{Experience, ExperienceInput, ExperiencesInput},
    project::{Project, ProjectInput, ProjectsInput},
    skill::{Skill, SkillInput, SkillsInput},
    social::{Social, SocialInput, SocialsInput},
};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;

#[async_trait]
pub trait CustomerRepository {
    // async fn find_customer(&self, limit: i64, page: i64) -> mongodb::error::Result<Vec<Customer>>;
    async fn find_customer_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Customer>>;
    async fn find_customer_by_api_key(&self, api_key: &str) -> mongodb::error::Result<Option<Customer>>;
    async fn find_customer_by_email(
        &self,
        email: String,
    ) -> mongodb::error::Result<Option<Customer>>;
    async fn insert_customer(&self, input: Json<CustomerInput>) -> mongodb::error::Result<String>;
    async fn update_customer_by_id(
        &self,
        oid: ObjectId,
        input: Json<CustomerUpdateInput>,
    ) -> mongodb::error::Result<Option<Customer>>;
    // async fn delete_customer_by_id(
    //     &self,
    //     oid: ObjectId,
    // ) -> mongodb::error::Result<Option<Customer>>;
}

#[async_trait]
pub trait ExperienceRepository {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Experience>>;
    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Experience>>;
    async fn insert(
        &self,
        input: Json<ExperienceInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String>;
    async fn update_by_id(
        &self,
        oid: ObjectId,
        input: Json<ExperienceInput>,
    ) -> mongodb::error::Result<Option<Experience>>;
    async fn update_many(
        &self,
        input: Json<Vec<ExperiencesInput>>,
    ) -> mongodb::error::Result<Option<Vec<Experience>>>;
    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Experience>>;
}

#[async_trait]
pub trait ProjectRepository {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Project>>;
    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Project>>;
    async fn insert(
        &self,
        input: Json<ProjectInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String>;
    async fn update_by_id(
        &self,
        oid: ObjectId,
        input: Json<ProjectInput>,
    ) -> mongodb::error::Result<Option<Project>>;
    async fn update_many(
        &self,
        input: Json<Vec<ProjectsInput>>,
    ) -> mongodb::error::Result<Option<Vec<Project>>>;
    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Project>>;
}

#[async_trait]
pub trait SkillRepository {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Skill>>;
    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Skill>>;
    async fn insert(
        &self,
        input: Json<SkillInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String>;
    async fn update_by_id(
        &self,
        oid: ObjectId,
        input: Json<SkillInput>,
    ) -> mongodb::error::Result<Option<Skill>>;
    async fn update_many(
        &self,
        input: Json<Vec<SkillsInput>>,
    ) -> mongodb::error::Result<Option<Vec<Skill>>>;
    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Skill>>;
}

#[async_trait]
pub trait SocialRepository {
    async fn find(
        &self,
        limit: i64,
        page: i64,
        oid: ObjectId,
    ) -> mongodb::error::Result<Vec<Social>>;
    async fn find_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Social>>;
    async fn insert(
        &self,
        input: Json<SocialInput>,
        oid: ObjectId,
    ) -> mongodb::error::Result<String>;
    async fn update_by_id(
        &self,
        oid: ObjectId,
        input: Json<SocialInput>,
    ) -> mongodb::error::Result<Option<Social>>;
    async fn update_many(
        &self,
        input: Json<Vec<SocialsInput>>,
    ) -> mongodb::error::Result<Option<Vec<Social>>>;
    async fn delete_by_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Social>>;
}
