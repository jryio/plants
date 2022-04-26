extern crate tokio;

mod db;
mod env;
mod routes;
mod schema;

use std::{fs, path::PathBuf};

use routes::Routes;
use schema::{PlantbookSchema, QueryRoot};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::Extension;

fn export_schema(schema: PlantbookSchema) -> anyhow::Result<()> {
    // Export the GraphQL schema in SDL format
    let sdl = &schema.sdl();
    let mut schema_path = project_root::get_project_root()?;
    let schema_filename = PathBuf::from("schema.graphql");
    schema_path.push(schema_filename);
    fs::write(schema_path, sdl)?;
    println!("Exported graphql schema");
    Ok(())
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
    let app = routes.layer(Extension(schema.clone()));

    export_schema(schema.clone())?;

    let addr = env::get_app_url()?;

    println!("Server running and listening on: {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to initialize");

    Ok(())
}
