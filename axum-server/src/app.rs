use axum::{Router};
use tokio::net::TcpListener;
use crate::routes::create_routes;

pub async fn app_service() {
    let app = create_routes();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
