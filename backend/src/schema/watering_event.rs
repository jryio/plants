use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use super::{person::Person, plant::Plant, queries};

/// When a plant has been watered a watering event is created
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct WateringEvent {
    pub id: Uuid,
    #[graphql(skip)]
    pub plant_id: Uuid,
    #[graphql(skip)]
    pub person_id: Uuid,
    pub ctime: DateTime<Utc>,
    pub notes: String,
}

#[ComplexObject]
impl WateringEvent {
    async fn plant(&self, ctx: &Context<'_>) -> Result<Plant> {
        // Use the existing Plant query
        let plant = queries::QueryRoot.plant(ctx, self.plant_id).await?;
        Ok(plant)
    }

    async fn person(&self, ctx: &Context<'_>) -> Result<Person> {
        // Use the existing Location query
        let person = queries::QueryRoot.person(ctx, self.person_id).await?;
        Ok(person)
    }
}
