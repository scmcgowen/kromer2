use actix_web::{error, http::StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("Name {0} not found")]
    NameNotFound(String),

    #[error("Name {0} is already taken")]
    NameTaken(String),

    #[error("You are not the owner of name {0}")]
    NotNameOwner(String),

    #[error("Insufficient balance to purchase name")]
    InsufficientBalance,
}

impl error::ResponseError for NameError {
    fn status_code(&self) -> StatusCode {
        match self {
            NameError::NameNotFound(_) => StatusCode::NOT_FOUND,
            NameError::NotNameOwner(_) => StatusCode::UNAUTHORIZED,
            NameError::NameTaken(_) => StatusCode::CONFLICT,
            NameError::InsufficientBalance => StatusCode::PAYMENT_REQUIRED,
        }
    }
}
