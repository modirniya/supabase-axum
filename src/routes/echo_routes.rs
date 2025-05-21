use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::user_context::{AuthUser, UserRole};

/// Request payload for the echo endpoints
#[derive(Debug, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

/// Response payload for the echo endpoints
#[derive(Debug, Serialize)]
pub struct EchoResponse {
    pub echoed_message: String,
    pub user_id: String,
    pub role: String,
}

/// Error response for unauthorized access
#[derive(Debug, Serialize)]
struct ErrorResponse {
    pub error: String,
}

/// Router for the echo endpoints
pub fn echo_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/echo", post(echo_handler))
        .route("/premium_echo", post(premium_echo_handler))
        .with_state(pool)
}

/// Handler for the regular echo endpoint
/// Accessible by any authenticated user
async fn echo_handler(
    auth_user: AuthUser, 
    Json(payload): Json<EchoRequest>,
) -> Json<EchoResponse> {
    let response = EchoResponse {
        echoed_message: payload.message,
        user_id: auth_user.id,
        role: auth_user.role.to_string(),
    };
    
    Json(response)
}

/// Handler for the premium echo endpoint
/// Only accessible by users with a "premium" role
async fn premium_echo_handler(
    auth_user: AuthUser,
    Json(payload): Json<EchoRequest>,
) -> Result<Json<EchoResponse>, impl IntoResponse> {
    // Verify that the user has the premium role
    if auth_user.role != UserRole::Premium && auth_user.role != UserRole::Admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Unauthorized: Premium or Admin role required".to_string(),
            }),
        ));
    }
    
    // If authorized, return a premium-specific echo response
    let response = EchoResponse {
        echoed_message: format!("PREMIUM: {}", payload.message),
        user_id: auth_user.id,
        role: auth_user.role.to_string(),
    };
    
    Ok(Json(response))
} 