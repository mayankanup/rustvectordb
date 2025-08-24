use axum;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod api;
mod db;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = api::routes::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("🚀 VectorDB running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
