use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing Authorization Header")]
    MissingAuthorization,

    #[error("Invalid Credentials")]
    InvalidCredentials,

    #[error("Asset Does Not Exist")]
    AssetDoesNotExist,

    #[error("User Does Not Exist")]
    UserDoesNotExist,

    #[error("This email is already registered")]
    EmailTaken,

    #[error("The provided email is not valid")]
    InvalidEmailFormat,

    #[error("The provided username is not valid")]
    InvalidName,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Template(#[from] askama::Error),

    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingAuthorization => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::EmailTaken | Self::InvalidEmailFormat | Self::InvalidName => {
                StatusCode::BAD_REQUEST
            }
            Self::AssetDoesNotExist | Self::UserDoesNotExist => StatusCode::NOT_FOUND,
            Self::Database(_) | Self::Template(_) | Self::Session(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn user_facing_message(&self) -> String {
        match self {
            Self::Database(_) | Self::Template(_) | Self::Session(_) => {
                "Internal server error".to_string()
            }
            Self::MissingAuthorization => "Unauthorized".to_string(),
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.user_facing_message(),
        };

        (self.status_code(), Json(error_response)).into_response()
    }
}
