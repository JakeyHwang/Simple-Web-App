use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        match self {
            AppError::Database(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::Auth(_) => HttpResponse::Unauthorized().json(error_response),
            AppError::Validation(_) => HttpResponse::BadRequest().json(error_response),
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            AppError::Bcrypt(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::Jwt(_) => HttpResponse::Unauthorized().json(error_response),
        }
    }
}

impl From<AppError> for HttpResponse {
    fn from(error: AppError) -> Self {
        error.error_response()
    }
} 