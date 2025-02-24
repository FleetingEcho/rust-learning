use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::routes::{create_routes, task_routes};

pub fn create_app(pool: PgPool) -> Router {
    let shared_pool = Arc::new(pool);
    Router::new()
        .merge(create_routes())
        .merge(task_routes())
        .with_state(shared_pool)
}
