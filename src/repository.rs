use std::convert::Infallible;

use axum::extract::FromRequestParts;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{
    app::AppState,
    models::{Asset, OwnedAsset, UserRecord},
};

pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub async fn list_assets(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(
            Asset,
            "SELECT id, name, unit_value::numeric AS \"unit_value!\"
            FROM assets
            ORDER BY name ASC;"
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn create_asset(&self, name: String, unit_value: Decimal) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "INSERT INTO assets (name, unit_value)
            VALUES ($1, $2::numeric)
            RETURNING id, name, unit_value::numeric AS \"unit_value!\";",
            name,
            unit_value
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        id: i64,
        name: Option<String>,
        unit_value: Option<Decimal>,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            "UPDATE assets
            SET name = COALESCE($2, name),
                unit_value = COALESCE($3::numeric, unit_value)
            WHERE id = $1
            RETURNING id, name, unit_value::numeric AS \"unit_value!\";",
            id,
            name,
            unit_value
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn add_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> sqlx::Result<UserRecord> {
        sqlx::query_as::<_, UserRecord>(
            "INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash;",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as::<_, UserRecord>(
            "SELECT id, username, email, password_hash
            FROM users
            WHERE email = $1;",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await
    }

    pub async fn list_owned_assets(&self, user_id: i64) -> sqlx::Result<Vec<OwnedAsset>> {
        sqlx::query_as!(
            OwnedAsset,
            r#"
            SELECT
            a.id,
            a.name,
            a.unit_value::numeric AS "unit_value!",
            SUM((a.unit_value::numeric - o.bought_for::numeric) * o.quantity_owned::numeric) AS "value_delta!",
            SUM(o.quantity_owned::numeric) AS "quantity_owned!",
            JSON_AGG(
              JSON_BUILD_OBJECT(
              'bought_at', o.timestamp,
              'bought_for', o.bought_for::numeric,
              'quantity_bought', o.quantity_owned::numeric,
              'value_delta', (a.unit_value::numeric - o.bought_for::numeric) * o.quantity_owned::numeric
              )
              ORDER BY o.timestamp ASC
            ) AS "purchase_history!: _"
            FROM assets AS a
            JOIN owned_assets AS o
              ON o.asset_id = a.id
            WHERE o.user_id = $1
            GROUP BY a.id;
            "#,
            user_id
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: Decimal,
        unit_value: Decimal,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO owned_assets
            (user_id, asset_id, quantity_owned, bought_for)
            VALUES ($1, $2, $3::numeric, $4::numeric)",
            user_id,
            asset_id,
            quantity,
            unit_value,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
