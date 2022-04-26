extern crate tokio;

mod db;
mod env;
mod routes;
mod schema;

use std::{fs, path::PathBuf};

use routes::Routes;
use schema::{PlantbookSchema, QueryRoot};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::AddExtensionLayer;

fn export_schema(schema: PlantbookSchema) {
    // Export the GraphQL schema in SDL format
    let sdl = &schema.sdl();
    let schema_path = PathBuf::from("schema.graphql");
    match fs::write(schema_path, sdl) {
        Ok(_) => println!("Exported graphql to schema.graphql"),
        Err(e) => panic!("Failed to wrie GraphQL schema file. Error = {e}"),
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Try to access all necessary environment variables, if any are missing, panic
    env::require_env_vars()?;

    tracing_subscriber::fmt::init();

    let pool = db::init().await?;

    let routes = Routes::all();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool) // Database connection pool added as async_graphql Context here
        .finish();
    let app = routes.layer(AddExtensionLayer::new(schema.clone()));

    export_schema(schema.clone());

    let addr = env::get_app_url()?;

    println!("Server running and listening on: {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to initialize");

    Ok(())
}
