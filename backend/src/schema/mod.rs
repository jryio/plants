use async_graphql::{EmptyMutation, EmptySubscription, Schema};

mod location;
mod person;
mod plant;
mod watering_event;

mod mutations;
mod queries;

pub use queries::QueryRoot;

pub type PlantbookSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
