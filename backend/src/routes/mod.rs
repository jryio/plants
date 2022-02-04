pub mod graphql;
pub mod home;

use axum::{
    routing::{get, post},
    Router,
};

use self::{
    graphql::{handle_get_playground, handle_post_query},
    home::handle_get_home,
};

pub struct Routes;

impl Routes {
    pub fn all() -> Router {
        Router::new()
            .route("/", get(handle_get_home))
            .route("/graphql", get(handle_get_playground))
            .route("/graphql", post(handle_post_query))
    }
}
