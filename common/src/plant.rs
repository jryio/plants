use std::convert::TryFrom;
use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::US::Eastern;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::location::Location;
use crate::person::Person;

pub type PlantId = Uuid;

// ====================================================
// Plant Definition
// ====================================================
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Plant {
    pub id: PlantId,
    pub name: String,
    pub location: Location,
    pub birthday: DateTime<Utc>,
    pub image: String,
    pub water_frequency: u32,
    pub water_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    pub last_watered_by: Person, // Person
}

// ====================================================
// Frontend Form : Plant
// ----------------------------------------------------
// We need a seperate struct to represent the Plant for
// yew_form as it cannot convert from strings to DateTime,
// Person, PersonId, or Location. To get around this we
// set those fiels as Strings in PlantForm then define
// TryFrom<PlantForm> for Plant to convert the strings
// to their struct objects.
// ====================================================
#[derive(Model, Validate, Serialize, Deserialize, Default, PartialEq, Debug, Clone)]
pub struct PlantForm {
    pub id: PlantId,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "The plant's location is required"))]
    pub location: String,
    #[validate(length(min = 1, message = "The plant's first day at home is required"))]
    pub birthday: String,
    #[validate(length(min = 1, message = "Image is requried"))]
    pub image: String,
    #[validate(range(min = 1, message = "Water frequency requried"))]
    pub water_frequency: u32,
    #[validate(length(min = 1, message = "Watering instructions are requried"))]
    pub water_instructions: String,
    #[validate(length(min = 1, message = "Last time the plant was watered is required"))]
    pub last_watered_date: String, // DateTime<Utc>
    #[validate(length(min = 1, message = "Last person to water the plant is required"))]
    pub last_watered_by: String, // Person
}

pub enum PlantConvertError {
    Location(String),
    Birthday(String),
    LastWateredDate(String),
    LastWateredBy(String),
}

// - PlantForm -> Plant
// - String -> DateTime<UTC>
// - String -> enum Location
// - String -> enum Person
impl TryFrom<PlantForm> for Plant {
    type Error = PlantConvertError;
    fn try_from(p: PlantForm) -> Result<Self, Self::Error> {
        let location = match Location::from_str(&p.location) {
            Ok(loc) => loc,
            Err(_) => {
                return Err(PlantConvertError::Location(format!(
                    "Failed to parse p.location = {} into enum Location",
                    &p.location
                )))
            }
        };

        // p.birthday from <input type="date" />
        let birthday = {
            let date = match NaiveDate::parse_from_str(&p.birthday, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    return Err(PlantConvertError::Birthday(format!(
                        "Failed to parse p.birthday = {} into NaiveDate",
                        &p.birthday,
                    )))
                }
            };
            let time = NaiveTime::from_hms(12, 0, 0);
            let datetime = NaiveDateTime::new(date, time);
            DateTime::<Utc>::from_utc(datetime, Utc)
        };

        // p.last_watered_date from <input type="datetime-local"
        // Format is yyyy-mm-ddThh:mm in 24 hour time regardless of system settings
        let last_watered_date = if p.last_watered_date.is_empty() {
            let local = Local::now();
            let utc: DateTime<Utc> = DateTime::from(local);
            utc
            // Eastern.from_utc_datetime(&Utc::now().naive_utc())
        } else {
            let date = match NaiveDateTime::parse_from_str(&p.last_watered_date, "%Y-%m-%dT%R") {
                Ok(datetime) => datetime,
                Err(e) => {
                    return Err(PlantConvertError::LastWateredDate(format!(
                        "Failed to parse p.last_watered_date = {:?} into NaiveDateTime because of error = {:?}",
                        &p.last_watered_date,
                        e.to_string()
                    )))
                }
            };
            let date = Eastern.from_local_datetime(&date).unwrap();
            date.with_timezone(&Utc)
        };

        let last_watered_by = match Person::from_str(&p.last_watered_by) {
            Ok(person) => person,
            Err(_) => {
                return Err(PlantConvertError::LastWateredBy(format!(
                    "Failed to parse p.last_watered_by = {} into enum Person",
                    &p.last_watered_by
                )))
            }
        };

        Ok(Self {
            id: p.id,
            name: p.name,
            birthday,
            location,
            image: p.image,
            water_frequency: p.water_frequency,
            water_instructions: p.water_instructions,
            last_watered_date,
            last_watered_by,
        })
    }
}

// ====================================================
// Plant Request
// ====================================================
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlantGetById {
    pub id: PlantId,
}

// ====================================================
// Plant Response
// ====================================================
#[derive(Validate, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlantResponse {
    pub id: PlantId,
    pub name: String,
    pub image: String,
    pub water_frequency: u32,
    pub water_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    pub last_watered_by: Person, // Person
    pub birthday: DateTime<Utc>,
}

impl PlantResponse {
    pub fn of(p: Plant) -> Self {
        Self {
            id: p.id,
            name: p.name,
            image: p.image,
            water_frequency: p.water_frequency,
            water_instructions: p.water_instructions,
            last_watered_date: p.last_watered_date,
            last_watered_by: p.last_watered_by,
            birthday: p.birthday,
        }
    }
}
