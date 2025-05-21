use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::header,
};
use jsonwebtoken::{decode, decode_header, Validation, jwk::AlgorithmParameters, DecodingKey, Algorithm};
use std::env;

use super::jwks::get_jwks;
use super::error::AuthError;
// AI: AuthUser will be defined in Phase 2.3. For now, we just validate the token.
// use super::user_context::AuthUser; 

async fn validate_token(token_str: &str) -> Result<(), AuthError> { 
    let token_header = decode_header(token_str).map_err(|e| AuthError::InvalidToken(format!("Invalid token header: {}", e)))?;
    let kid = token_header.kid.ok_or_else(|| AuthError::InvalidToken("Token header missing 'kid' (key ID)".to_string()))?;
    let alg_from_header = token_header.alg;
    
    let jwks = get_jwks().await.map_err(AuthError::JwksProcessingError)?;
    let jwk = jwks.find(&kid).ok_or_else(|| AuthError::JwkKidNotFound { kid: kid.clone() })?;

    // The `decode` function later will use the alg_from_header from Validation 
    // and the key material from jwk. It should internally handle algorithm compatibility.

    let required_iss = env::var("SUPABASE_JWT_ISS")
        .map_err(|_| AuthError::MissingEnvVar("SUPABASE_JWT_ISS".to_string()))?;
    let required_aud = env::var("SUPABASE_JWT_AUD")
        .map_err(|_| AuthError::MissingEnvVar("SUPABASE_JWT_AUD".to_string()))?;

    let mut validation = Validation::new(alg_from_header);
    validation.set_issuer(&[required_iss]);
    validation.set_audience(&[required_aud]);

    let decoding_key = match &jwk.algorithm {
        AlgorithmParameters::RSA(rsa_params) => {
            if !matches!(alg_from_header, Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 | Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512) {
                return Err(AuthError::InvalidToken("JWK is RSA but token algorithm is not an RSA variant".to_string()));
            }
            DecodingKey::from_rsa_components(&rsa_params.n, &rsa_params.e)
                .map_err(|e| AuthError::InvalidToken(format!("Failed to create RSA decoding key from JWK: {}", e)))?
        }
        AlgorithmParameters::OctetKey(oct_params) => { 
            if !matches!(alg_from_header, Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512) {
                return Err(AuthError::InvalidToken("JWK is OctetKey but token algorithm is not an HMAC variant".to_string()));
            }
            DecodingKey::from_secret(oct_params.value.as_ref())
        }
        // AI: Add EllipticCurve (EC) support if needed, with similar type checking
        // AlgorithmParameters::EllipticCurve(ec_params) => { ... }
        _ => return Err(AuthError::InvalidToken(format!("Unsupported JWK algorithm parameters for creating decoding key: {:?}", jwk.algorithm))),
    };

    decode::<serde_json::Value>(token_str, &decoding_key, &validation)?;

    Ok(())
}

pub async fn jwt_auth_middleware(req: Request, next: Next) -> Result<Response, AuthError> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token_str = if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            auth_header.trim_start_matches("Bearer ").to_owned()
        } else {
            return Err(AuthError::InvalidTokenFormat);
        }
    } else {
        return Err(AuthError::MissingToken);
    };

    validate_token(&token_str).await?;
    // AI: In Phase 2.3, the validated AuthUser should be inserted into request extensions:
    // req.extensions_mut().insert(auth_user);

    Ok(next.run(req).await)
} 