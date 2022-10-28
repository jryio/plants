pub mod locations;
pub mod people;
pub mod plants;

use gloo_net::http::Request;
use graphql_client::{GraphQLQuery, Response as GraphQLResponse};
use serde::Deserialize;
use std::result::Result;

use super::types::BounceError;

pub async fn graphql_post<Q: GraphQLQuery, ResponseType: for<'de> Deserialize<'de>>(
    url: &str,
    variables: Q::Variables,
) -> Result<ResponseType, BounceError> {
    // Convert the GraphQLQuery type into JSON by calling its
    // generated build_query function.
    let json_body = Q::build_query(variables);

    // Create a resonse using gloo-net. The documentation should
    // be consulted here.
    let response = Request::post(url)
        .header("Content-Type", "application/json")
        .json(&json_body)?
        .send()
        .await?;

    // Attempt to deserialize the JSON object into a
    // graphql_client::Response<GraphQLQuery::ResponseData> type.
    //
    // The JSON shape of the response should be
    //
    // pub struct Response<Data> {
    //     pub data: Option<Data>,
    //     pub errors: Option<Vec<Error>>,
    //     pub extensions: Option<HashMap<String, Value>>,
    // }
    //
    // See: https://docs.rs/graphql_client/0.11.0/graphql_client/struct.Response.html#
    let json_response = response.json::<GraphQLResponse<ResponseType>>().await?;
    if let Some(errors) = &json_response.errors {
        return Err(BounceError::ServerError(errors.clone()));
    }

    match (json_response.data, json_response.errors) {
        // Successful query
        (Some(data), None) => Ok(data),
        // Failed with server error
        (None, Some(errors)) => Err(BounceError::ServerError(errors)),
        // The GraphQL specification says that we should never have `data` and `errors` at the same
        // time. One or the other must be present
        (Some(_), Some(_)) => unreachable!(),
        // We should have some type of data response
        (None, None) => unreachable!(),
    }
}
