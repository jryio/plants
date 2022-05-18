use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

/// Whenever someone waters a plant they generate a WateringEvent entry
#[derive(SimpleObject)]
pub struct WateringEvent {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub person_id: Uuid,
    pub ctime: DateTime<Utc>,
    pub notes: String,
}
