use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub id: i32,
    pub last_transfered: Option<DateTime<Utc>>,
    pub last_updated: Option<DateTime<Utc>>,
    pub name: String,
    pub owner: String,
    pub original_owner: String,
    pub time_registered: DateTime<Utc>,
    pub unpaid: Decimal,
    pub metadata: Option<String>,
}

#[derive(Debug, Iden)]
pub enum Name {
    Table,
    Id,
    LastTransfered,
    LastUpdated,
    Name,
    Owner,
    OriginalOwner,
    TimeRegistered,
    Unpaid,
    Metadata,
}
