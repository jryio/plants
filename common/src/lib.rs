use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

type PersonId = Uuid;
type PlantId = Uuid;
// ====================================================
// Person
// ====================================================
// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
// pub struct PersonId(Uuid);
// impl Display for PersonId {
//     fn fmt(&self, &mut f: Formatter) -> Result {
//         self.0.fmt(f)
//     }
// }
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Person {
    id: PersonId,
    name: String, // TODO: See if we can serialize an enum here
}

// ====================================================
// Plant
// ====================================================
// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
// pub struct PlantId(Uuid);
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Plant {
    pub id: PlantId,
    pub name: String,
    pub image: String,
    pub water_frequency: u32,
    pub water_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    pub last_watered_by_person: PersonId, // Person
    pub birthday: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlantRequest {
    pub id: PlantId,
}

#[derive(Validate, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlantResponse {
    pub id: PlantId,
    pub name: String,
    pub image: String,
    pub water_frequency: u32,
    pub water_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    pub last_watered_by_person: PersonId, // Person
    pub birthday: DateTime<Utc>,
}

impl PlantResponse {
    pub fn of(p: Plant) -> PlantResponse {
        PlantResponse {
            id: p.id,
            name: p.name,
            image: p.image,
            water_frequency: p.water_frequency,
            water_instructions: p.water_instructions,
            last_watered_date: p.last_watered_date,
            last_watered_by_person: p.last_watered_by_person,
            birthday: p.birthday,
        }
    }
}

// Convert from Plant to PlantResponse for easier Json return
// TODO: Plant request
// TODO: Plant response
