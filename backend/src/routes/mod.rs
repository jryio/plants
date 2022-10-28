pub mod graphql;
pub mod home;

use axum::{
    http::header::CONTENT_TYPE,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};

use tower_http::cors::CorsLayer;

use self::{
    graphql::{handle_get_playground, handle_post_query},
    home::handle_get_home,
};

pub struct Routes;

impl Routes {
    pub fn all() -> Router {
        let home = Router::new().route("/", get(handle_get_home));
        let graphql = Router::new()
            .route("/graphql", get(handle_get_playground))
            .route("/graphql", post(handle_post_query))
            .layer(
                // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
                // for more details
                //
                // pay attention that for some request types like posting content-type: application/json
                // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
                // or see this issue https://github.com/tokio-rs/axum/issues/849
                CorsLayer::new()
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                    // By default CORS will allow text/plain and other content types, however if we
                    // want any content type, we need to explicitly allow this header during the
                    // pre-flight step. If we don't allow `Content-Type: *` then the browser will
                    // block the `POST` request with `Content-Type: application/json`
                    .allow_headers([CONTENT_TYPE])
                    .allow_methods([Method::GET, Method::POST]),
            );

        Router::new().merge(home).merge(graphql)
    }
}
