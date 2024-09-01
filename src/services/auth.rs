use bcrypt::{hash, verify, DEFAULT_COST};
use dotenvy::dotenv;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    sub: String,
    exp: usize,
}

impl JWTClaims {
    pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = 1720477487;

        let claims = JWTClaims {
            sub: user.id.to_string(),
            exp: expiration,
        };
        dotenv().ok();

        // Example secret key (replace with your secure secret)
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        // Encode the JWT token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(token)
    }
    pub fn decode_token(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
        dotenv().ok();

        // Example secret key (replace with your secure secret)
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        // Decode the JWT token
        let token_data = jsonwebtoken::decode::<JWTClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

pub fn hash_password(password: &str) -> String {
    let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
    password_hash
}
pub fn verify_password(hash: &str, password: &str) -> bool {
    let is_valid = verify(password, &hash).expect("Failed to verify password");
    is_valid
}
