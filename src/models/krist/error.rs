use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ErrorResponse {
    //pub ok: bool,
    pub error: String,
    pub message: Option<String>,
}
