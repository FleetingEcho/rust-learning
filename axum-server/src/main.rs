mod app;
mod auth;
mod error;
mod handlers;
mod models;
mod routes;

use axum::{serve, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:root@127.0.0.1:5432/zteng";
    let pool = PgPool::connect(database_url).await.unwrap();
    let app = app::create_app(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
