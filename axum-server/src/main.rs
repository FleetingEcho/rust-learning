mod app;
mod routes;

#[tokio::main]
async fn main() {
    app::app_service().await;
}
