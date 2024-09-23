use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Authentication error")]
    AuthError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = match &self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AuthError => StatusCode::UNAUTHORIZED,
        };
        (code, self.to_string()).into_response()
    }
}
