use axum::{http::StatusCode, response::IntoResponse, Json};
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

    #[error("This username is already registered")]
    UsernameTaken,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Template(#[from] askama::Error),

    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String, // fazer match aqui
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        let status = match self {
            Self::UsernameTaken | Self::MissingAuthorization => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::AssetDoesNotExist | Self::UserDoesNotExist => StatusCode::NOT_FOUND,
            Self::Database(_) | Self::Template(_) | Self::Session(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, Json(error_response)).into_response()
    }
}
