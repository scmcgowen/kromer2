use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use sqlx::{MySql, Pool};

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub address: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub locked: bool,
    pub total_in: Decimal,
    pub total_out: Decimal,
}

#[derive(Debug, Iden)]
pub enum Wallet {
    Table,
    Id,
    Address,
    Balance,
    CreatedAt,
    Locked,
    TotalIn,
    TotalOut,
}
