use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseIntError, str::FromStr};
use strum::{EnumIter, EnumVariantNames};
use uuid::Uuid;

// ====================================================
// Person
// ====================================================
pub type PersonId = Uuid;

#[derive(EnumVariantNames, EnumIter, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "Person")]
pub enum Person {
    Jacob = 0,
    Magda = 1,
    Marcella = 2,
}

impl Person {
    pub fn id(&self) -> isize {
        match self {
            Person::Jacob => 0,
            Person::Magda => 1,
            Person::Marcella => 2,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Person::Jacob => "Jacob",
            Person::Magda => "Magda",
            Person::Marcella => "Marcella",
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_str = match self {
            Person::Jacob => "0",
            Person::Magda => "1",
            Person::Marcella => "2",
        };
        write!(f, "{}", id_str)
    }
}

impl FromStr for Person {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = match s.parse::<isize>() {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let person = match id {
            0 => Person::Jacob,
            1 => Person::Magda,
            2 => Person::Marcella,
            _ => {
                panic!("We were given an id that does not match one of the variants of enum Person")
            }
        };

        Ok(person)
    }
}
