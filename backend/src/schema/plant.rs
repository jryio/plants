use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use crate::db::get_db;

use super::location::Location;
use super::person::Person;

/// A Plant lives in a location in the house, has a name, and has some instructions. It also knows
/// when it was born and who watered it last.
#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Plant {
    pub id: Uuid,
    pub name: String,
    #[graphql(skip)]
    pub location_id: Uuid,
    pub birthday: DateTime<Utc>,
    pub image: String, // This is going to be a path or URL to the image served by the backend
    pub watering_frequency: i16,
    pub watering_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    #[graphql(skip)]
    pub last_watered_by: Uuid,
}

#[ComplexObject]
impl Plant {
    /// location is the associated Location type including its id and name
    async fn location(&self, ctx: &Context<'_>) -> Result<Location> {
        let location = sqlx::query_as!(
            Location,
            r#"select id as "id!", name as "name!" from locations where id = $1"#,
            self.location_id
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(location)
    }

    /// last_watered_by is the associated Person type including its id and name
    async fn last_watered_by(&self, ctx: &Context<'_>) -> Result<Person> {
        let person = sqlx::query_as!(
            Person,
            r#"select id as "id!", name as "name!" from people where id = $1"#,
            self.last_watered_by
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(person)
    }
}
