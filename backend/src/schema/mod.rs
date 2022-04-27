use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use crate::db;

pub type PlantbookSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;

// Defining Enums using Async Grphql
// https://async-graphql.github.io/async-graphql/en/define_enum.html
// Note: Async Graphql will convert enum names to CONSTANT_CASE, we can manually rename to
// PascalCase by using #[graphql(name = "PascalCase")]
//
// Defining Enums using sqlx::postgres::types
// https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html#enumerations
// Note we don't want to crease a Postgres ENUM type because they are fixed strings which cannot be modified.
// Instead we create a table and join by id to simulate an enum. By deriving `sqlx::Type` we can tell sqlx that
// The string value of the
#[derive(async_graphql::Enum, sqlx::Type, Copy, Clone, Eq, PartialEq)]
#[sqlx(rename_all = "PascalCase")] // https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#rename_all
pub enum Location {
    #[graphql(name = "LivingRoom")]
    LivingRoom,
    #[graphql(name = "Kitchen")]
    Kitchen,
    #[graphql(name = "DiningRoom")]
    DiningRoom,
    #[graphql(name = "Bedroom")]
    Bedroom,
}

#[derive(async_graphql::Enum, sqlx::Type, Copy, Clone, Eq, PartialEq)]
#[sqlx(rename_all = "PascalCase")] // https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#rename_all
pub enum Person {
    #[graphql(name = "Jacob")]
    Jacob,
    #[graphql(name = "Magda")]
    Magda,
    #[graphql(name = "None")]
    None,
}

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

#[derive(SimpleObject)]
/// Whenever someone waters a plant they generate a WateringEvent entry
pub struct WateringEvent {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub date: DateTime<Utc>,
    pub person: Person,
}

#[Object]
impl QueryRoot {
    pub async fn plants(&self, ctx: &Context<'_>) -> Result<Vec<Plant>> {
        let plants = sqlx::query_as!(
            Plant,
            r#"
            select
                plants.id as "id!",
                plants.name as "name!",
                locs.name as "location!: Location",
                birthday as "birthday!",
                image as "image!",
                watering_frequency as "watering_frequency!",
                watering_instructions as "watering_instructions!",
                last_watered_date as "last_watered_date!",
                ppl.name as "last_watered_by!: Person"
            from plants
            join locations as locs
                on plants.location = locs.id
            join people as ppl
                on plants.last_watered_by = ppl.id
            "#
        )
        .fetch_all(db::get_db(ctx)?)
        .await?;
        Ok(plants)

        // TEMP: Return default Plant struct for testing
        // Ok(Plant {
        //     id: Uuid::default(),
        //     name: String::new(),
        //     location: Location::Kitchen,
        //     birthday: Utc::now(),
        //     image: String::new(),
        //     watering_frequency: 1,
        //     watering_instructions: String::new(),
        //     last_watered_date: Utc::now(),
        //     last_watered_by: Person::None,
        // })
    }
}
