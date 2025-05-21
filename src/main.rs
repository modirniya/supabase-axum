use axum::{
    routing::get,
    Router,
    response::Html,
    serve,
    middleware,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

// AI: Declare modules according to project structure
mod auth;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Load .env file if it exists

    // AI: Initialize tracing/logging as per Phase 4.2

    // Fetch and cache JWKS on startup
    // AI: Proper error handling should be implemented here. 
    // For now, panicking if JWKS fetching fails.
    match auth::jwks::get_jwks().await {
        Ok(_) => println!("Successfully fetched and cached JWKS."),
        Err(e) => {
            eprintln!("Failed to fetch JWKS: {}. Exiting.", e);
            // AI: In a real app, consider more graceful shutdown or retry logic.
            std::process::exit(1);
        }
    }

    let app = Router::new()
        .route("/", get(handler))
        .route("/protected", get(protected_handler))
        .route_layer(middleware::from_fn(auth::middleware::jwt_auth_middleware));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // AI: Read address from environment variable if available using std::env::var, e.g. for PORT
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World from Axum!</h1><p>AI: This is the root handler. Implement actual endpoints as per DEV-PLAN.md.</p>")
}

// AI: This is a placeholder protected handler.
async fn protected_handler() -> Html<&'static str> {
    Html("<h1>Protected Route</h1><p>AI: If you see this, JWT validation was successful.</p>")
}
