use crate::schema::PlantbookSchema;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Response,
};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    Json,
};

pub async fn handle_get_playground() -> impl IntoResponse {
    // GraphQLPlaygroundConfig can be given
    // - endpoint (str)
    // - subscription_endpoint (str)
    // - headers (HashMap)
    // - settings (HashMap)
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn handle_post_query(
    schema: Extension<PlantbookSchema>,
    req: Json<Request>,
) -> Json<Response> {
    schema.execute(req.0).await.into()
}
