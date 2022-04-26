use async_graphql::{Schema, EmptyMutation, EmptySubscription};

mod location;
mod person;
mod plant;
mod query;
mod watering_event;

pub use query::QueryRoot;

pub type PlantbookSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
