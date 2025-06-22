use actix_web::error;

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Wallet not found")]
    NotFound,

    #[error("Failed to create wallet for unknown reasons")]
    FailedCreate,

    #[error("Failed to transfer funds")]
    FailedTransfer,

    #[error("Invalid password")]
    InvalidPassword,
}

impl error::ResponseError for WalletError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            WalletError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            WalletError::FailedCreate => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            WalletError::InvalidPassword => actix_web::http::StatusCode::BAD_REQUEST,
            WalletError::FailedTransfer => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
