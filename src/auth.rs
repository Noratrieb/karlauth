use crate::errors::ServiceError;
use crate::models::User;
use actix_web::dev::{Payload, ServiceRequest};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    None,
    ReadAll,
    WriteAll,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    uid: i32,
    role: Role,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or(Default::default());

    match validate_token(credentials.token()) {
        Ok(claims) => {
            //req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(err) => Err(AuthenticationError::from(config).into()),
    }
}

fn validate_token(token: &str) -> Result<Claims, ServiceError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env var");

    let decoded = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| ServiceError::JWTokenError)?
    .claims;

    if decoded.exp > Utc::now().timestamp() as usize {
        Err(ServiceError::TokenExpiredError)
    } else {
        Ok(decoded)
    }
}

pub fn create_jwt(user: &User) -> Result<String, ServiceError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(10))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        exp: expiration as usize,
        uid: user.id,
        role: Role::ReadAll,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env var");

    let header = Header::new(Algorithm::HS512);
    jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| ServiceError::JWTCreationError)
}
