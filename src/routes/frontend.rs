use askama::Template;
use axum::{
    Router,
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::try_join;
use tower_sessions::Session;

use crate::{
    app::AppState,
    auth::user::{UnauthenticatedUser, User, UserSession},
    error::AppError,
    models::{Asset, OwnedAsset},
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/register", get(register_page).post(register))
        .route("/logout", get(logout))
        .route("/assets", get(assets).post(purchase_asset))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage {
    error: Option<String>,
    email: String,
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterPage {
    error: Option<String>,
    username: String,
    email: String,
}

#[tracing::instrument(skip_all)]
async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage {
        error: None,
        email: String::new(),
    }
    .render()?;
    Ok(Html(html))
}

#[tracing::instrument(skip_all)]
async fn register_page() -> Result<Html<String>, AppError> {
    let html = RegisterPage {
        error: None,
        username: String::new(),
        email: String::new(),
    }
    .render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

async fn login(
    repository: Repository,
    session: Session,
    Form(request): Form<LoginForm>,
) -> Result<Response, AppError> {
    let LoginForm { email, password } = request;
    let unauth_user = UnauthenticatedUser::new(email.clone(), password);

    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(err @ AppError::InvalidCredentials) => {
            return render_login_page(Some(err.to_string()), email, StatusCode::UNAUTHORIZED);
        }
        Err(err @ AppError::InvalidEmailFormat) => {
            return render_login_page(Some(err.to_string()), email, StatusCode::BAD_REQUEST);
        }
        Err(err) => return Err(err),
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

    Ok(Redirect::to("/").into_response())
}

async fn register(
    repository: Repository,
    session: Session,
    Form(request): Form<RegisterForm>,
) -> Result<Response, AppError> {
    let RegisterForm {
        username,
        email,
        password,
    } = request;

    let unauth_user = UnauthenticatedUser::new_register(username.clone(), email.clone(), password);

    let user = match unauth_user.register(&repository).await {
        Ok(user) => user,
        Err(
            err @ (AppError::InvalidName | AppError::InvalidEmailFormat | AppError::EmailTaken),
        ) => {
            return render_register_page(
                Some(err.to_string()),
                username,
                email,
                StatusCode::BAD_REQUEST,
            );
        }
        Err(err) => return Err(err),
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

    Ok(Redirect::to("/").into_response())
}

pub async fn logout(session: Session) -> Result<Redirect, AppError> {
    session.remove::<UserSession>("user").await?;

    Ok(Redirect::to("/login"))
}

async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login")),
    }
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    user: User,
}

pub async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
    }
    .render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    asset_id: i64,
    unit_value: Decimal,
    quantity: Decimal,
}

pub async fn purchase_asset(
    repository: Repository,
    user: User,
    Form(request): Form<PurchaseAssetForm>,
) -> Result<Redirect, AppError> {
    repository
        .insert_owned_asset(
            user.id(),
            request.asset_id,
            request.quantity,
            request.unit_value,
        )
        .await?;

    Ok(Redirect::to("/assets"))
}

pub mod filters {
    use askama;
    use rust_decimal::Decimal;
    use time::{
        OffsetDateTime, format_description::StaticFormatDescription, macros::format_description,
    };

    #[askama::filter_fn]
    pub fn pretty_number(value: &Decimal, _env: &dyn askama::Values) -> askama::Result<String> {
        Ok(format_decimal(*value))
    }

    #[askama::filter_fn]
    pub fn currency(value: &Decimal, _env: &dyn askama::Values) -> askama::Result<String> {
        Ok(format!("$ {}", format_decimal(*value)))
    }

    #[askama::filter_fn]
    pub fn signed_amount(value: &Decimal, _env: &dyn askama::Values) -> askama::Result<String> {
        let value = *value;

        if value.is_zero() {
            return Ok(String::from("0"));
        }

        let sign = if value.is_sign_negative() { "-" } else { "+" };
        Ok(format!("{sign}{}", format_decimal(value.abs())))
    }

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        const HUMAN_READABLE_FORMAT: StaticFormatDescription =
            format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]");

        datetime
            .format(HUMAN_READABLE_FORMAT)
            .map_err(askama::Error::custom)
    }

    fn format_decimal(value: Decimal) -> String {
        value.normalize().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_page_includes_error_message_and_email() {
        let html = LoginPage {
            error: Some(String::from("Invalid Credentials")),
            email: String::from("alice@example.com"),
        }
        .render()
        .expect("render login page");

        assert!(html.contains("Invalid Credentials"));
        assert!(html.contains(r#"value="alice@example.com""#));
    }

    #[test]
    fn register_page_includes_error_message_and_fields() {
        let html = RegisterPage {
            error: Some(String::from("The provided username is not valid")),
            username: String::from("alice"),
            email: String::from("alice@example.com"),
        }
        .render()
        .expect("render register page");

        assert!(html.contains("The provided username is not valid"));
        assert!(html.contains(r#"value="alice""#));
        assert!(html.contains(r#"value="alice@example.com""#));
    }
}

fn render_login_page(
    error: Option<String>,
    email: String,
    status: StatusCode,
) -> Result<Response, AppError> {
    let html = LoginPage { error, email }.render()?;
    Ok((status, Html(html)).into_response())
}

fn render_register_page(
    error: Option<String>,
    username: String,
    email: String,
    status: StatusCode,
) -> Result<Response, AppError> {
    let html = RegisterPage {
        error,
        username,
        email,
    }
    .render()?;
    Ok((status, Html(html)).into_response())
}
