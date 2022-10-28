use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Query, QueryResult};
use graphql_client::GraphQLQuery;
use std::rc::Rc;

use crate::api::types::{BounceError, Location, UUID};

use super::graphql_post;
// ------------------
// QueryLocations
// ------------------

// Query
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/locations.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryLocations;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryLocations(Vec<Location>);

#[async_trait(?Send)]
impl Query for BounceQueryLocations {
    type Input = query_locations::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        // Perform the HTTP request given the GraphQLQuery input
        let res = graphql_post::<QueryLocations, Vec<Location>>(
            "http://localhostL:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryLocations(res).into())
    }
}

// ------------------
// QueryLocationById
// ------------------
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/locationById.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryLocationById;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryLocationById(Location);

#[async_trait(?Send)]
impl Query for BounceQueryLocationById {
    type Input = query_location_by_id::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        let res = graphql_post::<QueryLocationById, Location>(
            "http://localhost:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryLocationById(res).into())
    }
}
