use actix_web::error;

#[derive(Debug, thiserror::Error)]
pub enum PlayerError {
    #[error("Player was not found")]
    NotFound,
}

impl error::ResponseError for PlayerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            PlayerError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}
