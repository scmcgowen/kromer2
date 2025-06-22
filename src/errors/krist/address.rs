use actix_web::{HttpResponse, error, http::StatusCode};
use thiserror::Error;

use super::{KristErrorExt, KristErrorResponse};

#[derive(Error, Debug)]
pub enum AddressError {
    #[error("Address {0} not found")]
    NotFound(String),

    #[error("Authentication failed")]
    AuthFailed,
}

impl KristErrorExt for AddressError {
    fn error_type(&self) -> &'static str {
        match self {
            AddressError::NotFound(_) => "address_not_found",
            AddressError::AuthFailed => "auth_failed",
        }
    }
}

impl error::ResponseError for AddressError {
    fn status_code(&self) -> StatusCode {
        // TODO: Evaluate whether or not programs might break when returning the correct error code or not.
        //       In Krist, responses are always error code 200 because of a long standing bug.
        //       For some reason, that bug was never fixed and is just set there for forever, pretty stupid if you ask me.
        match self {
            AddressError::NotFound(_) => StatusCode::NOT_FOUND,
            AddressError::AuthFailed => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let message = KristErrorResponse {
            ok: false,
            error: self.error_type(),
            message: self.to_string(),
            info: None, // Unsure wheter or not this is right
        };

        HttpResponse::build(self.status_code()).json(message)
    }
}
