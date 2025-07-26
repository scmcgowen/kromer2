use actix_web::error;

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Wallet {0} was not found")]
    NotFound(String),

    #[error("Authentication failed")]
    AuthFailed,
}

impl error::ResponseError for WalletError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            WalletError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            WalletError::AuthFailed => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
