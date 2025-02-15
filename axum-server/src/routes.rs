use axum::{Router, routing::{get, post}, response::IntoResponse};

async fn root() -> impl IntoResponse {
    "Hello root"
}

async fn get_foo() -> impl IntoResponse {
    "Hello get_foo"
}

async fn post_foo() -> impl IntoResponse {
    "Hello post_foo"
}

async fn foo_bar() -> impl IntoResponse {
    "Hello foo_bar"
}


pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
}
