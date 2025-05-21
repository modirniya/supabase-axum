use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse, Json},
    serve,
    middleware,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;

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

// Protected handler that demonstrates using AuthUser extractor
async fn protected_handler(auth_user: auth::user_context::AuthUser) -> impl IntoResponse {
    Json(json!({
        "message": "Protected route access successful",
        "user_id": auth_user.id,
        "email": auth_user.email,
        "role": auth_user.role.to_string(),
        "token_issued_at": auth_user.iat,
        "token_expires_at": auth_user.exp
    }))
}
