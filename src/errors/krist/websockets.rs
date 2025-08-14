use actix_web::{HttpResponse, error};
use thiserror::Error;

use super::{KristErrorExt, KristErrorResponse};

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("Invalid websocket token")]
    InvalidWebsocketToken,

    #[error("Failed to create a WebSocket handshake")]
    HandshakeError,

    #[error("WebSocket gateway token does not exist, go get a new one <3")]
    TokenDoesNotExist,
}

impl error::ResponseError for WebSocketError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::FORBIDDEN
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = KristErrorResponse {
            ok: false,
            error: self.error_type(),
            message: self.to_string(),
            info: None, // Unsure wheter or not this is right
        };

        HttpResponse::build(self.status_code()).json(message)
    }
}

impl KristErrorExt for WebSocketError {
    fn error_type(&self) -> &'static str {
        match self {
            WebSocketError::InvalidWebsocketToken => "invalid_websocket_token",
            WebSocketError::HandshakeError => "handshake_error",
            WebSocketError::TokenDoesNotExist => "gateway_token_not_found",
        }
    }
}
