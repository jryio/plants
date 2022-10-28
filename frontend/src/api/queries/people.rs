use async_trait::async_trait;
use bounce::prelude::*;
use bounce::query::{Query, QueryResult};
use graphql_client::GraphQLQuery;
use std::rc::Rc;

use crate::api::types::{BounceError, Person, UUID};

use super::graphql_post;

// ------------------
// QueryPeople
// ------------------

// Query
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/people.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryPeople;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryPeople(Vec<Person>);

#[async_trait(?Send)]
impl Query for BounceQueryPeople {
    type Input = query_people::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        // Perform the HTTP request given the GraphQLQuery input
        let res = graphql_post::<QueryPeople, Vec<Person>>(
            "http://localhostL:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryPeople(res).into())
    }
}

// ------------------
// QueryPersonById
// ------------------

// Query
#[derive(GraphQLQuery, PartialEq, Eq, Hash)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "src/api/graphql/personById.graphql",
    response_derives = "Debug,PartialEq,Eq",
    variables_derives = "Hash,PartialEq,Eq,Clone,Copy"
)]
pub struct QueryPersonById;

// Bounce
#[derive(Debug, PartialEq, Eq)]
pub struct BounceQueryPersonById(Person);

#[async_trait(?Send)]
impl Query for BounceQueryPersonById {
    type Input = query_person_by_id::Variables;
    type Error = BounceError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        // Perform the HTTP request given the GraphQLQuery input
        let res = graphql_post::<QueryPersonById, Person>(
            "http://localhostL:8080/graphql",
            *input.clone(),
        )
        .await?;
        Ok(BounceQueryPersonById(res).into())
    }
}
