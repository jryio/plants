use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

/// When a plant has been watered a watering event is created
#[derive(SimpleObject)]
pub struct WateringEvent {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub person_id: Uuid,
    pub ctime: DateTime<Utc>,
    pub notes: String,
}
