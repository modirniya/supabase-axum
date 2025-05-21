use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// AI: This struct represents a user's profile in the database.
// It's distinct from auth::user_context::AuthUser, which represents the authenticated JWT claims.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserProfile {
    pub id: Uuid, // Links to auth.users.id
    pub email: Option<String>,
    pub username: Option<String>,
    // AI: Add other profile fields as needed, e.g., full_name, avatar_url
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// AI: Payload for creating a new user profile. 
// `id` and `email` might come from AuthUser or be explicitly set if creating for another user (admin scenarios).
// For typical self-service, `id` and initial `email` would come from AuthUser context.
#[derive(Debug, Deserialize)]
pub struct CreateProfilePayload {
    // pub email: Option<String>, // Email might be pre-filled from AuthUser
    pub username: Option<String>,
    // AI: Add other fields that can be set at creation
}

// AI: Payload for updating an existing user profile.
// User can typically update fields like username, etc. Email updates might have special handling.
#[derive(Debug, Deserialize)]
pub struct UpdateProfilePayload {
    pub username: Option<String>,
    // AI: Add other updatable fields
}

// AI: Response structure when returning a profile, could be UserProfile itself or a wrapper.
// Using UserProfile directly for simplicity for now. 