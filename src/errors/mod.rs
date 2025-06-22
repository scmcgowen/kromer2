pub mod krist;
pub mod name;
pub mod transaction;
pub mod wallet;
pub mod websocket;

use actix_web::{HttpResponse, body::BoxBody, error, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum KromerError {
    #[error("Resource not found")]
    NotFound,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Wallet error: {0}")]
    Wallet(#[from] wallet::WalletError),

    #[error("Name error: {0}")]
    Name(#[from] name::NameError),

    #[error("Transaction error: {0}")]
    Transaction(#[from] transaction::TransactionError),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] websocket::WebSocketError),

    #[error("Something went wrong: {0}")]
    Internal(&'static str),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<'a> {
    pub message: &'a str,
    pub description: String,
}

impl error::ResponseError for KromerError {
    fn status_code(&self) -> StatusCode {
        match self {
            KromerError::NotFound => StatusCode::NOT_FOUND,
            KromerError::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KromerError::Wallet(e) => e.status_code(),
            KromerError::Transaction(e) => e.status_code(),
            KromerError::Name(e) => e.status_code(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let response = ApiResponse {
            message: match self {
                KromerError::NotFound => "not_found",
                KromerError::Database(..) => "database",
                KromerError::Wallet(..) => "wallet",
                KromerError::Transaction(..) => "transaction",
                _ => "internal_server_error",
            },
            description: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(response)
    }
}
