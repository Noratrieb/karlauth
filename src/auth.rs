use crate::errors::ServiceError;
use crate::models::User;
use actix_web::dev::{Payload, ServiceRequest};
use actix_web::{FromRequest, HttpMessage, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub enum Role {
    None = 0,
    ReadAll = 1,
    WriteAll = 2,
    Admin = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub uid: i32,
    pub role: Role,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        std::future::ready(
            req.extensions()
                .get::<Claims>()
                .map(|claims| claims.clone())
                .ok_or(
                    HttpResponse::InternalServerError()
                        .json("Could not get claims")
                        .into(),
                ),
        )
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    match validate_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(err) => Err(err.into()),
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

    if decoded.exp < Utc::now().timestamp() as usize {
        Err(ServiceError::TokenExpiredError)
    } else {
        Ok(decoded)
    }
}

pub fn create_jwt(user: &User) -> Result<String, ServiceError> {
    create_jwt_role(user, Role::ReadAll)
}

pub fn create_jwt_role(user: &User, role: Role) -> Result<String, ServiceError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(10))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        exp: expiration as usize,
        uid: user.id,
        role,
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
