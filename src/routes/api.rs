use axum::{Json, Router, routing::get};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    app::AppState, auth::admin::Admin, error::AppError, models::Asset, repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets).post(create_asset).patch(update_asset),
    )
}

#[tracing::instrument(skip_all)]
async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: Decimal,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<Decimal>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository
        .update_asset(request.id, request.name, request.unit_value)
        .await?
    {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::AssetDoesNotExist),
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn test_create_assets(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Dogecoin".to_string(),
            unit_value: Decimal::new(100, 1),
        };
        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("sucess");

        assert_eq!(new_asset.id, 11);
        assert_eq!(new_asset.name, "Dogecoin".to_string());
        assert_eq!(new_asset.unit_value, Decimal::new(100, 1));

        let serialized = serde_json::to_string_pretty(&new_asset).expect("serialize");
        insta::assert_snapshot!(serialized);
    }

    #[sqlx::test]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("sucess");

        assert_eq!(assets.len(), 10);
        assert!(assets.iter().any(|asset| asset.name == "Bitcoin"));

        let serialized = serde_json::to_string_pretty(&assets).expect("serialize");
        insta::assert_snapshot!(serialized);
    }

    #[sqlx::test]
    async fn test_update_assets(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Avalanche".to_string()),
            unit_value: Some(Decimal::new(200, 1)),
        };
        let Json(update_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("sucess");

        assert_eq!(update_asset.id, 1);
        assert_eq!(update_asset.name, "Avalanche".to_string());
        assert_eq!(update_asset.unit_value, Decimal::new(200, 1));

        let serialized = serde_json::to_string_pretty(&update_asset).expect("serialize");
        insta::assert_snapshot!(serialized);
    }
}
