use askama::Template;
use axum::{
    Router,
    extract::Form,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
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
        .route("/logout", get(logout))
        .route("/assets", get(assets).post(purchase_asset))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

#[tracing::instrument(skip_all)]
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
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);

    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
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
    unit_value: f64,
    quantity: f64,
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
    use time::{
        OffsetDateTime, format_description::StaticFormatDescription, macros::format_description,
    };

    #[askama::filter_fn]
    pub fn pretty_number(value: &f64, _env: &dyn askama::Values) -> askama::Result<String> {
        Ok(format_number(*value))
    }

    #[askama::filter_fn]
    pub fn currency(value: &f64, _env: &dyn askama::Values) -> askama::Result<String> {
        Ok(format!("$ {}", format_number(*value)))
    }

    #[askama::filter_fn]
    pub fn signed_amount(value: &f64, _env: &dyn askama::Values) -> askama::Result<String> {
        let value = round_to_two(*value);

        if value == 0.0 {
            return Ok(String::from("0"));
        }

        let sign = if value.is_sign_negative() { "-" } else { "+" };
        Ok(format!("{sign}{}", format_number(value.abs())))
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

    fn round_to_two(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    fn format_number(value: f64) -> String {
        let mut formatted = format!("{:.2}", round_to_two(value));

        while formatted.contains('.') && formatted.ends_with('0') {
            formatted.pop();
        }

        if formatted.ends_with('.') {
            formatted.pop();
        }

        if formatted.is_empty() {
            String::from("0")
        } else {
            formatted
        }
    }
}
