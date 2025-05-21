use sqlx::{PgPool, query_as, query};
use uuid::Uuid;

use super::models::{UserProfile, CreateProfilePayload, UpdateProfilePayload};
use super::DbError;

// AI: Repository for UserProfile CRUD operations

/// Creates a new user profile.
/// Assumes `id` and `email` are provided, typically derived from `AuthUser`.
pub async fn create_profile(
    pool: &PgPool,
    user_id: Uuid,
    email: Option<String>,
    payload: CreateProfilePayload,
) -> Result<UserProfile, DbError> {
    let profile = query_as::<_, UserProfile>(
        "INSERT INTO public.profiles (id, email, username) 
        VALUES ($1, $2, $3)
        RETURNING id, email, username, created_at, updated_at"
    )
    .bind(user_id)
    .bind(email)
    .bind(&payload.username)
    .fetch_one(pool)
    .await
    .map_err(DbError::ProfileCreationError)?;
    
    Ok(profile)
}

/// Fetches a user profile by its ID.
pub async fn get_profile_by_id(pool: &PgPool, user_id: Uuid) -> Result<UserProfile, DbError> {
    query_as::<_, UserProfile>(
        "SELECT id, email, username, created_at, updated_at 
        FROM public.profiles 
        WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or(DbError::ProfileNotFound)
}

/// Updates an existing user profile.
pub async fn update_profile(
    pool: &PgPool,
    user_id: Uuid,
    payload: UpdateProfilePayload,
) -> Result<UserProfile, DbError> {
    // AI: This implementation only updates username. Extend to update other fields as needed.
    // Consider how to handle partial updates (e.g., only update fields that are Some(value)).
    // For more complex partial updates, a query builder or dynamic query string might be needed,
    // or multiple specific update functions.
    
    let updated_profile = query_as::<_, UserProfile>(
        "UPDATE public.profiles
        SET username = $2, updated_at = now()
        WHERE id = $1
        RETURNING id, email, username, created_at, updated_at"
    )
    .bind(user_id)
    .bind(&payload.username)
    .fetch_optional(pool) // Use fetch_optional in case the ID doesn't exist
    .await?
    .ok_or(DbError::ProfileNotFound)?; // Return ProfileNotFound if update affected 0 rows for the given ID
    
    Ok(updated_profile)
}

/// Deletes a user profile by its ID.
pub async fn delete_profile(pool: &PgPool, user_id: Uuid) -> Result<(), DbError> {
    let result = query(
        "DELETE FROM public.profiles WHERE id = $1"
    )
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(DbError::ProfileDeleteError)?;

    if result.rows_affected() == 0 {
        Err(DbError::ProfileNotFound)
    } else {
        Ok(())
    }
}

// AI: Placeholder for database schema (SQL for `profiles` table)
/*
-- Example SQL for public.profiles table
-- Ensure this table is created in your Supabase PostgreSQL database.

CREATE TABLE IF NOT EXISTS public.profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    email TEXT UNIQUE,
    username TEXT UNIQUE,
    -- AI: Add other profile fields as needed, e.g.:
    -- full_name TEXT,
    -- avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Optional: Trigger to update `updated_at` timestamp automatically
CREATE OR REPLACE FUNCTION public.handle_updated_at() 
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER on_profile_updated
  BEFORE UPDATE ON public.profiles
  FOR EACH ROW
  EXECUTE PROCEDURE public.handle_updated_at();

-- Enable Row Level Security (RLS) on the profiles table.
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;

-- Policies for RLS:
-- 1. Users can view their own profile.
CREATE POLICY "Users can view their own profile" ON public.profiles
  FOR SELECT
  USING (auth.uid() = id);

-- 2. Users can insert their own profile.
CREATE POLICY "Users can insert their own profile" ON public.profiles
  FOR INSERT
  WITH CHECK (auth.uid() = id);

-- 3. Users can update their own profile.
CREATE POLICY "Users can update their own profile" ON public.profiles
  FOR UPDATE
  USING (auth.uid() = id)
  WITH CHECK (auth.uid() = id);

-- 4. Users can delete their own profile (optional, consider if this is desired).
CREATE POLICY "Users can delete their own profile" ON public.profiles
  FOR DELETE
  USING (auth.uid() = id);
*/ 