use async_graphql::SimpleObject;
use sqlx::types::Uuid;

#[derive(SimpleObject, sqlx::Type)]
pub struct Location {
    id: Uuid,
    name: String,
}
