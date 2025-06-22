pub mod krist;
mod not_found;

use actix_web::{HttpResponse, get, web};

use crate::errors::krist::KristError;

#[get("/")]
pub async fn index_get() -> Result<HttpResponse, KristError> {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/api/v1").configure(v1::config));
    cfg.service(web::scope("/api/krist").configure(krist::config));
    // cfg.service(
    //     web::scope("/api/_internal")
    //         .guard(guards::internal_key_guard)
    //         .configure(internal::config),
    // );
    cfg.service(web::scope("").service(index_get));
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PaginationParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: Some(50),
            offset: Some(0),
        }
    }
}
