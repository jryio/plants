use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Query, QueryResult};
use graphql_client::GraphQLQuery;
use std::rc::Rc;

use crate::api::types::{BounceError, WateringEvents, UUID};

use super::graphql_post;

// ------------------
// QueryWateringEvents
// ------------------

// Query
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/watering_events.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryWateringEvents;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryWateringEvents(Vec<Person>);

#[async_trait(?Send)]
impl Query for BounceQueryWateringEvents {
    type Input = query_watering_events::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        // Perform the HTTP request given the GraphQLQuery input
        let res = graphql_post::<QueryWateringEvents, Vec<Person>>(
            "http://localhostL:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryWateringEvents(res).into())
    }
}

// -----------------------
// QueryWateringEventById
// -----------------------

// Query
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/watering_eventById.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryWateringEventById;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryWateringEventById(Vec<Person>);

#[async_trait(?Send)]
impl Query for BounceQueryWateringEventById {
    type Input = query_watering_events_by_id::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        // Perform the HTTP request given the GraphQLQuery input
        let res = graphql_post::<QueryWateringEventById, Vec<Person>>(
            "http://localhostL:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryWateringEventById(res).into())
    }
}
