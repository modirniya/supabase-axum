use sqlx::{postgres::PgPoolOptions, PgPool, Error as SqlxError};
use std::env;
use std::time::Duration;

pub mod models; // AI: Added models submodule
pub mod profile_repository; // AI: Added profile_repository submodule

// AI: Consider moving this error to a more general AppError enum in Phase 4.1
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("DATABASE_URL not set in environment")]
    DatabaseUrlNotSet,

    #[error("Failed to create database pool: {0}")]
    PoolCreationFailed(SqlxError),
    
    #[error("Failed to connect to database after multiple retries: {0}")]
    ConnectionError(SqlxError),

    // AI: Add specific database operation errors as needed
    #[error("User profile not found")]
    ProfileNotFound,

    #[error("Database query failed: {0}")]
    QueryError(#[from] SqlxError), // Generic query error

    #[error("Failed to create profile: {0}")]
    ProfileCreationError(SqlxError),

    #[error("Failed to update profile: {0}")]
    ProfileUpdateError(SqlxError),

    #[error("Failed to delete profile: {0}")]
    ProfileDeleteError(SqlxError),
}

// AI: This function initializes a PgPool. It should be called once at application startup.
// The returned pool should be stored in Axum's application state.
pub async fn init_db_pool() -> Result<PgPool, DbError> {
    let database_url = env::var("DATABASE_URL").map_err(|_| DbError::DatabaseUrlNotSet)?;

    // AI: Configure pool options as needed. These are some sensible defaults.
    // For Supabase, direct connections might be limited, so adjust max_connections accordingly.
    // Supabase uses PgBouncer, which might manage its own pool. Check Supabase best practices.
    PgPoolOptions::new()
        .max_connections(10) // Max number of connections in the pool
        .min_connections(2)  // Min number of connections to keep alive
        .acquire_timeout(Duration::from_secs(8)) // Timeout for acquiring a connection
        .idle_timeout(Duration::from_secs(8 * 60 * 60)) // Timeout for an idle connection (8 hours)
        .max_lifetime(Duration::from_secs(16 * 60 * 60)) // Max lifetime of a connection (16 hours)
        .connect(&database_url)
        .await
        .map_err(DbError::PoolCreationFailed)
}

// Optional: A function to test the connection if needed during startup.
// pub async fn test_db_connection(pool: &PgPool) -> Result<(), SqlxError> {
//     sqlx::query("SELECT 1").fetch_one(pool).await?;
//     Ok(())
// }

// AI: Placeholder for where to put database models (e.g., User struct for Phase 3.2)
// pub mod models { ... } 