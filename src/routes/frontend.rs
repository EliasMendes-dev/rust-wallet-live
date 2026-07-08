use askama::Template;
use axum::{
    Router, extract::Form, response::{Html, IntoResponse, Redirect, Response}, routing::get,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    app::AppState, auth::user::{UnauthenticatedUser, User, UserSession}, error::AppError, repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    session: Session,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user =
        UnauthenticatedUser::new(request.username, request.password);

    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => {
            unauth_user.register(&repository).await?
        }
        Err(other_err) => return Err(other_err),
    };

    session
        .insert(
            "user",
            UserSession {
                id: user.id(),
                username: user.username().clone(),
            },
        )
        .await?;

    Ok(Redirect::to("/"))
}

async fn index(maybe_user: Option<User>) -> Result<Response, AppError> {
    match maybe_user {
        Some(user) => Ok(Html(format!("Hello, {}", user.username())).into_response()),
        None => Ok(Redirect::to("/login").into_response()),
    }
}