use axum::{routing::get, Router};

mod handlers;

use handlers::handle_get_home;

#[tokio::main]
async fn main() {
    // TODO: Set this environment variable in .env/*.env file. Coordinate with cargo-make
    // Tracing
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(handle_get_home));
    // TODO: Get host and port from env variables (remember to parse/validate the strings before passing them in )
    axum::Server::bind(&"0.0.0.0:9090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server failed to initialize");
}
