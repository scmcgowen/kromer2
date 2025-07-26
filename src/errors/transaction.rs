use actix_web::{error, http::StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Transaction not found")]
    NotFound,

    #[error("Transactions disabled")]
    Disabled,

    #[error("Same wallet transfer is not allowed")]
    SameWalletTransfer,

    #[error("Transaction conflict for parameter {0}")]
    Conflict(String),
}

impl error::ResponseError for TransactionError {
    fn status_code(&self) -> StatusCode {
        match self {
            TransactionError::NotFound => StatusCode::NOT_FOUND,
            TransactionError::InsufficientFunds => StatusCode::BAD_REQUEST,
            TransactionError::Disabled => StatusCode::FORBIDDEN,
            TransactionError::SameWalletTransfer => StatusCode::FORBIDDEN,
            TransactionError::Conflict(_) => StatusCode::CONFLICT,
        }
    }
}
