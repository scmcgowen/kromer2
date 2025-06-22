use actix_web::error;

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("Name not found")]
    NotFound,

    #[error("Failed to transfer name")]
    FailedTransfer,
}

impl error::ResponseError for NameError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            NameError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            NameError::FailedTransfer => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
