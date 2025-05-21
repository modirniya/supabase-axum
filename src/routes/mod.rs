use sqlx::PgPool;
pub mod profile_routes;
pub mod echo_routes;

// AI: Add other route modules here as the application grows

// Function to combine all application routes
pub fn app_routes(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .nest("/api/profiles", profile_routes::profile_routes(pool.clone()))
        .nest("/api", echo_routes::echo_routes(pool.clone()))
    // AI: Nest other route modules here, e.g.:
    // .nest("/api/items", items_routes::items_routes(pool.clone()))
}
