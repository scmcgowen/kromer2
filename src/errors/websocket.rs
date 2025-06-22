use actix_web::Error as ActixError;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("Failed to read the WebSocket payload")]
    PayloadRead,

    #[error("Failed to deserialize JSON in request")]
    JsonParseRead,

    #[error("Failed to create a WebSocket room")]
    RoomCreation,

    #[error("Failed to create a WebSocket handshake")]
    HandshakeError,

    #[error("Failed to send a WebSocket message")]
    MessageSend,

    #[error("Failed to disconnect a WebSocket")]
    Disconnect,

    #[error("Failed to list sessions")]
    ListSessions,

    #[error("WebSocket Closed")]
    WebSocketClosed,

    #[error("Could not parse incoming message type")]
    InvalidMessageType,

    // Shouldn't really ever happen in production...
    #[error("Server configuration issue, contact an admin")]
    ServerConfigError,

    #[error("UUID was not found in server cache")]
    UuidNotFound,

    #[error("An invalid WebSocket Token was supplied")]
    InvalidUuid,

    #[error("Error parsing Kromer Address")]
    KromerAddressError,

    #[error("Conversion error, value must be a number or a string")]
    IdConversionError,
}

impl From<WebSocketError> for ActixError {
    fn from(err: WebSocketError) -> Self {
        InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR).into()
    }
}
