use jsonwebtoken::jwk::JwkSet;
use tokio::sync::OnceCell;
use reqwest::Client;
use std::env;

// AI: Define a specific error type for JWKS fetching, or use a general AppError (Phase 4.1)
#[derive(Debug, thiserror::Error)] // AI: Placeholder, requires thiserror crate
pub enum JwksError {
    #[error("Network error fetching JWKS: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Invalid JWKS URL: {0}")]
    InvalidUrl(String),
    #[error("JWKS_URL not set in environment")]
    UrlNotSet,
    // AI: Add more error variants as needed (e.g., parsing error if not covered by reqwest::Error)
}

static CACHED_JWKS: OnceCell<JwkSet> = OnceCell::const_new();

async fn fetch_jwks_from_url(jwks_url: &str, client: &Client) -> Result<JwkSet, JwksError> {
    let jwks: JwkSet = client
        .get(jwks_url)
        .send()
        .await?
        .error_for_status()? // Ensure we have a success status
        .json()
        .await?;
    Ok(jwks)
}

pub async fn get_jwks() -> Result<&'static JwkSet, JwksError> {
    CACHED_JWKS.get_or_try_init(|| async {
        let jwks_url = env::var("SUPABASE_JWKS_URL")
            .map_err(|_| JwksError::UrlNotSet)?;
        // AI: Consider creating a single reqwest::Client and reusing it (e.g., via AppState or OnceCell)
        let client = Client::new(); 
        println!("Fetching JWKS from: {}", jwks_url);
        fetch_jwks_from_url(&jwks_url, &client).await
    }).await
}

// AI: Add logic for JWKS refresh if needed, based on cache-control headers or a fixed interval.

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // AI: This test requires a running mock server or a real JWKS endpoint.
    // For now, it checks if the function attempts to fetch and fails if URL is not set.
    #[tokio::test]
    async fn test_get_jwks_url_not_set() {
        env::remove_var("SUPABASE_JWKS_URL");
        let result = get_jwks().await;
        assert!(matches!(result, Err(JwksError::UrlNotSet)));
    }

    // AI: Add more tests: successful fetch, caching behavior, error handling for network issues.
    // Example for a successful fetch test (requires a mock server or a valid SUPABASE_JWKS_URL):
    // #[tokio::test]
    // async fn test_get_jwks_success() {
    //     // Mock the env var and potentially the HTTP request
    //     env::set_var("SUPABASE_JWKS_URL", "your_mock_jwks_url_or_real_one_for_testing");
    //     let jwks = get_jwks().await.expect("Failed to get JWKS");
    //     assert!(!jwks.keys.is_empty());
    //     // Test caching: second call should be faster and not print "Fetching JWKS..." again if logging is added to fetch_jwks_from_url
    //     let jwks_cached = get_jwks().await.expect("Failed to get JWKS from cache");
    //     assert_eq!(jwks.keys.len(), jwks_cached.keys.len());
    // }
} 