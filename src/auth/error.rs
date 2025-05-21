use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

// AI: This AuthError will be expanded in Phase 4.1 (Centralized Error Handling)
// For now, it focuses on JWT validation errors.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Authorization token not found")]
    MissingToken,

    #[error("Invalid token format")]
    InvalidTokenFormat,

    #[error("Token invalid: {0}")]
    InvalidToken(String),

    #[error("Token signature expired")]
    TokenExpired,

    #[error("Token claim invalid: {claim} - {reason}")]
    TokenClaimInvalid { claim: String, reason: String },

    #[error("Could not find key to verify token signature (JWK with kid {kid} not found)")]
    JwkKidNotFound { kid: String },

    #[error("Internal error during JWKS processing: {0}")]
    JwksProcessingError(#[from] crate::auth::jwks::JwksError),
    
    #[error("Required environment variable for validation not set: {0}")]
    MissingEnvVar(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidTokenFormat => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired".to_string()),
            AuthError::TokenClaimInvalid { .. } => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::JwkKidNotFound { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "Could not verify token (key not found)".to_string()),
            AuthError::JwksProcessingError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error processing signing keys".to_string()),
            AuthError::MissingEnvVar(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error for auth".to_string()),
            AuthError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// Helper to convert jsonwebtoken::errors::Error into AuthError
impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidToken => AuthError::InvalidToken("Token is invalid".to_string()),
            jsonwebtoken::errors::ErrorKind::InvalidSignature => AuthError::InvalidToken("Token signature is invalid".to_string()),
            jsonwebtoken::errors::ErrorKind::InvalidAudience => AuthError::TokenClaimInvalid { claim: "aud".to_string(), reason: "Invalid audience".to_string() },
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthError::TokenClaimInvalid { claim: "iss".to_string(), reason: "Invalid issuer".to_string() },
            // AI: Add more detailed mappings as needed
            _ => AuthError::InvalidToken(format!("JWT validation error: {}", err)),
        }
    }
} 