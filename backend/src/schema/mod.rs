use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type PlantbookSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub struct QueryRoot;

// https://async-graphql.github.io/async-graphql/en/define_enum.html
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Location {
    LivingRoom,
    Kitchen,
    DiningRoom,
    Bedroom,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
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
    pub water_frequency: u8,
    pub water_instructions: String,
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
    pub async fn plants(&self, _ctx: &Context<'_>) -> Result<Plant> {
        Ok(Plant {
            id: Uuid::default(),
            name: String::new(),
            location: Location::Kitchen,
            birthday: Utc::now(),
            image: String::new(),
            water_frequency: 1,
            water_instructions: String::new(),
            last_watered_date: Utc::now(),
            last_watered_by: Person::None,
        })
    }
}
