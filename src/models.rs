use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use time::OffsetDateTime;

#[derive(Serialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub unit_value: Decimal,
}

#[derive(sqlx::FromRow)]
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    #[allow(dead_code)]
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurchaseHistory {
    #[serde(with = "time::serde::iso8601")]
    pub bought_at: OffsetDateTime,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub bought_for: Decimal,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub quantity_bought: Decimal,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub value_delta: Decimal,
}

#[derive(Serialize)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub unit_value: Decimal,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub value_delta: Decimal,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub quantity_owned: Decimal,
    pub purchase_history: Json<Vec<PurchaseHistory>>,
}

impl OwnedAsset {
    pub fn purchase_history(&self) -> &[PurchaseHistory] {
        &self.purchase_history.0
    }

    pub fn has_non_negative_change(&self) -> bool {
        self.value_delta >= Decimal::ZERO
    }
}

impl PurchaseHistory {
    pub fn has_non_negative_change(&self) -> bool {
        self.value_delta >= Decimal::ZERO
    }
}
