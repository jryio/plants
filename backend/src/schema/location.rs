use async_graphql::SimpleObject;
use sqlx::types::Uuid;

#[derive(SimpleObject, sqlx::Type)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
}
