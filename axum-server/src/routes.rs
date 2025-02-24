use axum::{
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::task::{create_task, delete_task, get_task, get_tasks, update_task};

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

pub fn create_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
}

pub fn task_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/api/tasks", get(get_tasks))
        .route("/api/tasks", post(create_task))
        .route("/api/tasks/:task_id", get(get_task))
        .route("/api/tasks/:task_id", put(update_task))
        .route("/api/tasks/:task_id", delete(delete_task))
}
