use axum::{
    extract::{State, Json, Path},
    response::{Response, IntoResponse},
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::user_context::AuthUser;
use crate::db::profile_repository;
use crate::db::models::{CreateProfilePayload, UpdateProfilePayload, UserProfile};
use crate::db::DbError;

// AI: Error handling for API routes. This could be part of a larger AppError in Phase 4.1.
// For now, converting DbError into an appropriate HTTP response.
impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DbError::DatabaseUrlNotSet => (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error".to_string()),
            DbError::PoolCreationFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not connect to database".to_string()),
            DbError::ConnectionError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database connection issue".to_string()),
            DbError::ProfileNotFound => (StatusCode::NOT_FOUND, "Profile not found".to_string()),
            DbError::QueryError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database query failed: {}", e)),
            DbError::ProfileCreationError(e) => (StatusCode::BAD_REQUEST, format!("Could not create profile: {}", e)), // Or INTERNAL_SERVER_ERROR depending on cause
            DbError::ProfileUpdateError(e) => (StatusCode::BAD_REQUEST, format!("Could not update profile: {}", e)), // Or INTERNAL_SERVER_ERROR
            DbError::ProfileDeleteError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Could not delete profile: {}", e)),
        };
        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}

// Router is Router<PgPool>, state is provided by with_state
pub fn profile_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/me", get(get_my_profile_handler))
        .route("/me", post(create_my_profile_handler))
        .route("/me", put(update_my_profile_handler))
        .route("/me", delete(delete_my_profile_handler))
        // AI: Admin routes for managing any user's profile could be added here, protected by role-based auth
        .route("/:user_id", get(get_user_profile_handler))
        .with_state(pool)
}

/// Handler to create the authenticated user's profile.
async fn create_my_profile_handler(
    auth_user: AuthUser, // Extracted from JWT
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProfilePayload>,
) -> Result<impl IntoResponse, DbError> {
    let user_id = auth_user.id.parse().map_err(|_| DbError::QueryError(sqlx::Error::Decode("Invalid user ID format in token".into())))?;
    let profile = profile_repository::create_profile(&pool, user_id, auth_user.email, payload).await?;
    Ok((StatusCode::CREATED, Json(profile)))
}

/// Handler to get the authenticated user's profile.
async fn get_my_profile_handler(
    auth_user: AuthUser, // Extracted from JWT
    State(pool): State<PgPool>,
) -> Result<Json<UserProfile>, DbError> {
    let user_id = auth_user.id.parse().map_err(|_| DbError::QueryError(sqlx::Error::Decode("Invalid user ID format in token".into())))?;
    let profile = profile_repository::get_profile_by_id(&pool, user_id).await?;
    Ok(Json(profile))
}

/// Handler to update the authenticated user's profile.
async fn update_my_profile_handler(
    auth_user: AuthUser, // Extracted from JWT
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateProfilePayload>,
) -> Result<Json<UserProfile>, DbError> {
    let user_id = auth_user.id.parse().map_err(|_| DbError::QueryError(sqlx::Error::Decode("Invalid user ID format in token".into())))?;
    let profile = profile_repository::update_profile(&pool, user_id, payload).await?;
    Ok(Json(profile))
}

/// Handler to delete the authenticated user's profile.
async fn delete_my_profile_handler(
    auth_user: AuthUser, // Extracted from JWT
    State(pool): State<PgPool>,
) -> Result<StatusCode, DbError> {
    let user_id = auth_user.id.parse().map_err(|_| DbError::QueryError(sqlx::Error::Decode("Invalid user ID format in token".into())))?;
    profile_repository::delete_profile(&pool, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Admin handler to get any user's profile by ID (requires role check)
async fn get_user_profile_handler(
    Path(user_id_str): Path<String>,
    auth_user: AuthUser, // For role check
    State(pool): State<PgPool>,
) -> Result<Json<UserProfile>, DbError> {
    // Implement role check here
    if auth_user.role != crate::auth::user_context::UserRole::Admin {
        return Err(DbError::QueryError(sqlx::Error::Decode("Unauthorized: Admin role required".into())));
    }
    
    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| DbError::QueryError(sqlx::Error::Decode("Invalid UUID format".into())))?;
    let profile = profile_repository::get_profile_by_id(&pool, user_id).await?;
    Ok(Json(profile))
} 