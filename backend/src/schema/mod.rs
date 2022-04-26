use async_graphql::*;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use crate::db;

pub type PlantbookSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;

// https://async-graphql.github.io/async-graphql/en/define_enum.html
#[derive(async_graphql::Enum, sqlx::Type, Copy, Clone, Eq, PartialEq)]
// #[sqlx(type_name = "location")] // only for PostgreSQL to match a type definition
// #[sqlx(rename_all = "PascalCase")] // https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#rename_all
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
// #[sqlx(type_name = "location")] // only for PostgreSQL to match a type definition
// #[sqlx(rename_all = "PascalCase")] // https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#rename_all
pub enum Person {
    Jacob,
    Magda,
    None,
}

#[derive(SimpleObject)]
/// A Plant lives in a location in the house, has a name, and has some instructions. It also knows when it was born and who watered it last
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
/// Whenever someone waters a plant they generate a WateringLog entry
pub struct WateringLog {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub date: DateTime<Utc>,
    pub person: Person,
}

#[Object]
impl QueryRoot {
    // TODO: @jacob - I will need to write SQLx query here to get the data for all plants in the table
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
                last_watered_by as "last_watered_by!: Person"
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
