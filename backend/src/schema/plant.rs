use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use super::location::Location;
use super::person::Person;

/// A Plant lives in a location in the house, has a name, and has some instructions. It also knows
/// when it was born and who watered it last.
#[derive(SimpleObject)]
pub struct Plant {
    pub id: Uuid,
    pub name: String,
    pub location: Location,
    pub birthday: DateTime<Utc>,
    pub image: String, // This is going to be a path or URL to the image served by the backend
    pub watering_frequency: i16,
    pub watering_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    pub last_watered_by: Person,
}
