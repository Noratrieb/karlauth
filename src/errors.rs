use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWT Creation Error")]
    JWTCreationError,

    #[display(fmt = "JWT Error")]
    JWTokenError,

    #[display(fmt = "No Permission Error")]
    NoPermissionError,

    #[display(fmt = "Token Expired Error")]
    TokenExpiredError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWTCreationError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
            ServiceError::JWTokenError => HttpResponse::BadRequest().json("Invalid JWT"),
            ServiceError::NoPermissionError => HttpResponse::Unauthorized().json("No permissions"),
            ServiceError::TokenExpiredError => HttpResponse::Unauthorized().json("Token expired"),
        }
    }
}
