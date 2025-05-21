use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::error::AuthError;

/// Represents a user's role in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Premium,
    Admin,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Premium => write!(f, "premium"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

/// Represents authenticated user information extracted from JWT claims
#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    /// The unique user identifier from Supabase auth.users
    pub id: String,
    /// The user's email address
    pub email: Option<String>,
    /// The user's role
    pub role: UserRole,
    /// When the token was issued (Unix timestamp)
    pub iat: i64,
    /// When the token expires (Unix timestamp)
    pub exp: i64,
}

/// Custom extractor for getting the authenticated user from request extensions
#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let extension = parts.extensions.get::<AuthUser>().cloned();
        
        match extension {
            Some(user) => Ok(user),
            None => Err(AuthError::InternalError("User context not found. Is the auth middleware applied?".into())),
        }
    }
} 