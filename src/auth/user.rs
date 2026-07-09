use std::convert::Infallible;

use axum::extract::FromRequestParts;
use once_cell::sync::Lazy;
use password_auth::VerifyError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{app::AppState, error::AppError, repository::Repository};

static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?xi)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap());

pub struct UnauthenticatedUser {
    username: Option<String>,
    email: String,
    password: String,
}

impl UnauthenticatedUser {
    pub fn new(email: String, password: String) -> Self {
        Self {
            username: None,
            email,
            password,
        }
    }

    pub fn new_register(username: String, email: String, password: String) -> Self {
        Self {
            username: Some(username),
            email,
            password,
        }
    }

    pub async fn authenticate(&self, repository: &Repository) -> Result<User, AppError> {
        let email = self.email.trim();

        if !EMAIL_REGEX.is_match(email) {
            return Err(AppError::InvalidEmailFormat);
        }

        let user_record = match repository.get_user_by_email(email).await? {
            Some(user_record) => user_record,
            None => return Err(AppError::InvalidCredentials),
        };

        match password_auth::verify_password(&self.password, &user_record.password_hash) {
            Ok(()) => Ok(User::new(user_record.id, user_record.username)),

            Err(VerifyError::PasswordInvalid) => Err(AppError::InvalidCredentials),

            Err(VerifyError::Parse(err)) => {
                panic!("Hashing algorithm failed: {err}");
            }
        }
    }

    pub async fn register(self, repository: &Repository) -> Result<User, AppError> {
        let UnauthenticatedUser {
            username,
            email,
            password,
        } = self;

        let username = username.ok_or(AppError::InvalidName)?.trim().to_string();

        if username.is_empty() {
            return Err(AppError::InvalidName);
        }

        let email = email.trim();

        if !EMAIL_REGEX.is_match(email) {
            return Err(AppError::InvalidEmailFormat);
        }

        let password_hash = password_auth::generate_hash(password);

        let user_record = match repository.add_user(&username, email, &password_hash).await {
            Ok(user_record) => user_record,

            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppError::EmailTaken);
            }

            Err(err) => return Err(AppError::Database(err)),
        };

        Ok(User::new(user_record.id, user_record.username))
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    fn register_user(username: &str, email: &str, password: &str) -> UnauthenticatedUser {
        UnauthenticatedUser::new_register(
            username.to_string(),
            email.to_string(),
            password.to_string(),
        )
    }

    fn login_user(email: &str, password: &str) -> UnauthenticatedUser {
        UnauthenticatedUser::new(email.to_string(), password.to_string())
    }

    #[sqlx::test]
    async fn register_trims_username_and_email(db: PgPool) {
        let repository: Repository = db.into();

        let created = register_user("  Alice  ", "  alice@example.com  ", "secret-password")
            .register(&repository)
            .await
            .expect("register");

        assert_eq!(created.username(), "Alice");

        let stored = repository
            .get_user_by_email("alice@example.com")
            .await
            .expect("query")
            .expect("stored user");

        assert_eq!(stored.username, "Alice");
        assert_ne!(stored.password_hash, "secret-password");
    }

    #[sqlx::test]
    async fn register_rejects_invalid_email_format(db: PgPool) {
        let repository: Repository = db.into();

        let err = register_user("Alice", "not-an-email", "secret-password")
            .register(&repository)
            .await
            .expect_err("invalid email should fail");

        assert!(matches!(err, AppError::InvalidEmailFormat));
    }

    #[sqlx::test]
    async fn authenticate_rejects_invalid_email_format(db: PgPool) {
        let repository: Repository = db.into();

        let err = login_user("not-an-email", "secret-password")
            .authenticate(&repository)
            .await
            .expect_err("invalid email should fail");

        assert!(matches!(err, AppError::InvalidEmailFormat));
    }

    #[sqlx::test]
    async fn authenticate_rejects_unknown_email(db: PgPool) {
        let repository: Repository = db.into();

        let err = login_user("missing@example.com", "secret-password")
            .authenticate(&repository)
            .await
            .expect_err("unknown email should fail");

        assert!(matches!(err, AppError::InvalidCredentials));
        assert_eq!(err.to_string(), "Invalid Credentials");
    }

    #[sqlx::test]
    async fn authenticate_rejects_wrong_password(db: PgPool) {
        let repository: Repository = db.into();

        register_user("Alice", "alice@example.com", "correct-password")
            .register(&repository)
            .await
            .expect("register");

        let err = login_user("alice@example.com", "wrong-password")
            .authenticate(&repository)
            .await
            .expect_err("wrong password should fail");

        assert!(matches!(err, AppError::InvalidCredentials));
        assert_eq!(err.to_string(), "Invalid Credentials");
    }

    #[sqlx::test]
    async fn authenticate_returns_user_for_valid_credentials(db: PgPool) {
        let repository: Repository = db.into();

        register_user("Alice", "alice@example.com", "correct-password")
            .register(&repository)
            .await
            .expect("register");

        let user = login_user("alice@example.com", "correct-password")
            .authenticate(&repository)
            .await
            .expect("authenticate");

        assert_eq!(user.username(), "Alice");
        assert!(user.id() > 0);
    }
}
