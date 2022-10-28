use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Query, QueryResult};
use graphql_client::GraphQLQuery;
use std::rc::Rc;

use super::{graphql_post, BounceError};
use crate::api::types::{DateTime, Plant, UUID};

// ------------------
// QueryPlants
// ------------------

// Query
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/plants.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryPlants;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryPlants(Vec<Plant>);

#[async_trait(?Send)]
impl Query for BounceQueryPlants {
    type Input = query_plants::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        // Perform the HTTP request given the GraphQLQuery input
        let res = graphql_post::<QueryPlants, Vec<Plant>>(
            "http://localhost:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryPlants(res).into())
    }
}

// ------------------
// QueryPlantById
// ------------------
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/plantById.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryPlantById;

#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryPlantById(Plant);

#[async_trait(?Send)]
impl Query for BounceQueryPlantById {
    type Input = query_plant_by_id::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        let res =
            graphql_post::<QueryPlantById, Plant>("http://localhost:8080/graphql", *input.clone())
                .await?;
        Ok(BounceQueryPlantById(res).into())
    }
}
