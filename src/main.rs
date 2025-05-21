use axum::{
    routing::get,
    Router,
    response::Html,
    serve
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // AI: Initialize dotenvy to load .env file (if it exists)
    // Example: dotenvy::dotenv().ok();

    // AI: Initialize tracing/logging as per Phase 4.2

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // AI: Read address from environment variable if available
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World from Axum!</h1><p>AI: This is the root handler. Implement actual endpoints as per DEV-PLAN.md.</p>")
}
