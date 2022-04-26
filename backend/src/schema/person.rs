use async_graphql::SimpleObject;
use sqlx::types::Uuid;

#[derive(SimpleObject, sqlx::Type)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}
