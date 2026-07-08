use std::convert::Infallible;

use axum::extract::FromRequestParts;
use password_auth::VerifyError;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{
    app::AppState,
    error::AppError,
    repository::Repository,
};

pub struct UnauthenticatedUser {
    username: String,
    password: String,
}

impl UnauthenticatedUser {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub async fn authenticate(
        &self,
        repository: &Repository,
    ) -> Result<User, AppError> {
        let user_record = match repository.get_user_by_name(&self.username).await? {
            Some(user_record) => user_record,
            None => return Err(AppError::UserDoesNotExist),
        };

        match password_auth::verify_password(
            &self.password,
            &user_record.password_hash,
        ) {
            Ok(()) => Ok(User::new(
                user_record.id,
                user_record.username,
            )),

            Err(VerifyError::PasswordInvalid) => {
                Err(AppError::InvalidCredentials)
            }

            Err(VerifyError::Parse(err)) => {
                panic!("Hashing algorithm failed: {err}");
            }
        }
    }

    pub async fn register(
        self,
        repository: &Repository,
    ) -> Result<User, AppError> {
        let password_hash = password_auth::generate_hash(self.password);

        let user_record = match repository
            .add_user(&self.username, &password_hash)
            .await
        {
            Ok(user_record) => user_record,

            Err(sqlx::Error::Database(db_err))
                if db_err.is_unique_violation() =>
            {
                return Err(AppError::UsernameTaken);
            }

            Err(err) => return Err(AppError::Database(err)),
        };

        Ok(User::new(
            user_record.id,
            user_record.username,
        ))
    }
}

pub struct User {
    id: i64,
    username: String,
}

impl User {
    fn new(id: i64, username: String) -> Self {
        Self { id, username }
    }

    pub const fn username(&self) -> &String {
        &self.username
    }

    pub const fn id(&self) -> i64 {
        self.id
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {

        let session = Session::from_request_parts(parts, _state)
            .await
            .map_err(|_| AppError::MissingAuthorization)?;

        let user: UserSession = session
            .get("user")
            .await?
            .ok_or(AppError::MissingAuthorization)?;

        Ok(User::new(user.id, user.username))
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
       Ok(User::from_request_parts(parts, state).await.ok())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserSession {
   pub id: i64,
   pub username: String,
}