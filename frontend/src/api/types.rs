use chrono::{DateTime as Dt, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use uuid::Uuid;

#[allow(clippy::upper_case_acronyms)] // Match GraphQL upper case naming convention
pub type UUID = Uuid;

#[allow(clippy::upper_case_acronyms)] // Match GraphQL upper case naming convention
pub type DateTime = Dt<Utc>;

// Plant is the response data from a query
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Plant {
    pub id: Uuid,
    pub name: String,
    pub location: Location,
    pub birthday: DateTime,
    pub image: String,
    pub water_frequency: u32,
    pub water_instructions: String,
    pub last_watered_date: DateTime,
    pub last_watered_by: Person, // Person
}

// Person is the return data from a query
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Person {}

// Location is the return data from a query
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Location {}

// WateringEvent is the return data from a query
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WateringEvent {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub person_id: Uuid,
    pub ctime: DateTime,
    pub notes: String,
}

// ------------------
// Errors
// ------------------
// Bounce requires that errors be: std::error::Error + PartialEq + Clone + 'static
#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq)]
pub enum BounceError {
    JsError(String),
    SerdeError(String),
    ServerError(Vec<graphql_client::Error>),
}

impl From<gloo_net::Error> for BounceError {
    fn from(error: gloo_net::Error) -> Self {
        match error {
            gloo_net::Error::JsError(err) => {
                Self::JsError(format!("{}: {}", err.name, err.message))
            }
            gloo_net::Error::SerdeError(err) => Self::SerdeError(err.to_string()),
        }
    }
}

impl Display for BounceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsError(msg) => f.write_str(msg),
            Self::SerdeError(msg) => f.write_str(msg),
            Self::ServerError(errors) => {
                let error_strings: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
                f.write_str(&error_strings.join("-"))
            }
        }
    }
}

impl Debug for BounceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsError(msg) => f.write_str(msg),
            Self::SerdeError(msg) => f.write_str(msg),
            Self::ServerError(errors) => f
                .debug_list()
                .entries(errors.iter().map(|e| e.message.clone()))
                .finish(),
        }
    }
}

impl std::error::Error for BounceError {}
