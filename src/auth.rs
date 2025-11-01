use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // Username
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);

        Self {
            sub: user_id.to_string(),
            username,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

/// Generate a JWT token
pub fn generate_token(user_id: Uuid, username: String, secret: &str) -> Result<String, Error> {
    let claims = Claims::new(user_id, username, 24); // 24 hours expiration
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ErrorUnauthorized(format!("Failed to generate token: {}", e)))
}

/// Validate a JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| ErrorUnauthorized(format!("Invalid token: {}", e)))
}

/// Validator function for actix-web-httpauth (simplified)
/// Note: This is a placeholder for future middleware integration
/// Currently, authentication can be added to specific handlers as needed
#[allow(dead_code)]
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Try to get JWT secret from app data
    let config = req.app_data::<actix_web::web::Data<crate::config::Config>>();

    if config.is_none() {
        return Err((ErrorUnauthorized("Configuration not found"), req));
    }

    let secret = &config.unwrap().jwt_secret;

    // Validate token
    match validate_token(credentials.token(), secret) {
        Ok(claims) => {
            // Attach claims to request extensions
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => Err((e, req)),
    }
}

/// Hash password using bcrypt
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_token_generation_and_validation() {
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let secret = "test_secret_key_123456";

        let token = generate_token(user_id, username.clone(), secret).unwrap();
        let claims = validate_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_invalid_token() {
        let secret = "test_secret_key_123456";
        let result = validate_token("invalid.token.here", secret);
        assert!(result.is_err());
    }
}
