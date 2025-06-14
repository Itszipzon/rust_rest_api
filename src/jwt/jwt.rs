use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use chrono::{Utc, Duration};

use crate::jwt::claims::Claims;

pub struct JwtManager {
    secret: String,
}

impl JwtManager {
    pub fn new() -> Self {
        dotenv().ok();
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self { secret }
    }

    pub fn generate_token(&self, username: &str, id: u32) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: username.to_owned(),
            id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
