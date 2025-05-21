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
mod db;
mod routes; // Added routes module

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Load .env file if it exists

    // AI: Initialize tracing/logging as per Phase 4.2

    // Initialize database pool
    // AI: Proper error handling. For now, panicking if DB pool creation fails.
    let db_pool = match db::init_db_pool().await {
        Ok(pool) => {
            println!("Successfully connected to the database and created pool.");
            // Optional: Test the connection
            // if let Err(e) = db::test_db_connection(&pool).await {
            //     eprintln!("Failed to test database connection: {}. Exiting.", e);
            //     std::process::exit(1);
            // }
            pool
        }
        Err(e) => {
            eprintln!("Failed to initialize database pool: {}. Exiting.", e);
            std::process::exit(1);
        }
    };

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

    // Build application with routes
    let app = Router::new()
        .route("/", get(handler)) // Public route
        // Group all /api routes and protect them with JWT auth middleware
        .nest("/api", routes::app_routes(db_pool.clone()) // Pass db_pool here
            .route_layer(middleware::from_fn(auth::middleware::jwt_auth_middleware))
        );
        // .layer(Extension(db_pool)); // AI: Removed as PgPool is now passed via with_state in app_routes

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // AI: Read address from environment variable if available using std::env::var, e.g. for PORT
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World from Axum!</h1><p>AI: This is the root handler. Implement actual endpoints as per DEV-PLAN.md.</p>")
}

// AI: Example handler showing how to access the db_pool (add in a later phase if needed)
// async fn db_example_handler(Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
//     match sqlx::query_scalar("SELECT version()").fetch_one(&pool).await {
//         Ok(version) => Json(json!({ "db_version": version })),
//         Err(e) => Json(json!({ "error": e.to_string() })),
//     }
// }
