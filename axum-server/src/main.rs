mod app;
mod routes;
mod handlers;
mod models;
mod error;
mod auth;

use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:root@127.0.0.1:5432/zteng";
    let pool = PgPool::connect(database_url).await.unwrap();
    let app = app::create_app(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service())
        .await
        .unwrap();
}
