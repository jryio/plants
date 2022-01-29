mod routes;

use std::{fs, io::Write, path::PathBuf};

use common::schema::{PlantbookSchema, QueryRoot};
use routes::Routes;

extern crate tokio;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::AddExtensionLayer;

fn export_schema(schema: PlantbookSchema) -> Result<String, String> {
    // Export the GraphQL schema in SDL format
    let sdl = &schema.sdl();
    let schema_path = PathBuf::from("schema.graphql");
    match fs::write(schema_path, sdl) {
        Ok(_) => Ok(String::from("Updated GraphQL schema file")),
        Err(e) => Err(format!("Failed to wrie GraphQL schema file. Error = {e}")),
    }
}

#[tokio::main]
async fn main() {
    // TODO: Set this environment variable in .env/*.env file. Coordinate with cargo-make
    // Tracing
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let routes = Routes::all();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let app = routes.layer(AddExtensionLayer::new(schema.clone()));

    match export_schema(schema.clone()) {
        Ok(m) => println!("{m}"),
        Err(e) => eprintln!("{e}"),
    }

    // TODO: Get host and port from env variables (remember to parse/validate the strings before passing them in )
    let port = r#"0.0.0.0:9090"#;
    println!("Server running and listening on port {port}");

    axum::Server::bind(&port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server failed to initialize");
}
