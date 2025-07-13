use actix_web::{HttpResponse, get, post, web};
use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::database::wallet::Model as Wallet;

use crate::database::ModelExt;
use crate::{
    AppState,
    errors::{KromerError, transaction::TransactionError},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MinecraftUser {
    pub name: String,
    pub mc_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GiveMoneyReq {
    pub address: String,
    pub amount: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Guh {
    pub name: String,
}

#[post("/create")]
async fn wallet_create(
    state: web::Data<AppState>,
    user: web::Json<MinecraftUser>,
) -> Result<HttpResponse, KromerError> {
    let pool = &state.pool;
    let user = user.into_inner();

    todo!()
}

#[post("/give-money")]
async fn wallet_give_money(
    state: web::Data<AppState>,
    data: web::Json<GiveMoneyReq>,
) -> Result<HttpResponse, KromerError> {
    let pool = &state.pool;
    let data = data.into_inner();

    if data.amount < dec!(0.0) {
        return Err(KromerError::Transaction(TransactionError::InvalidAmount));
    }

    todo!()
}

#[get("/by-player/{uuid}")]
async fn wallet_get_by_uuid(
    state: web::Data<AppState>,
    uuid: web::Path<Uuid>,
) -> Result<HttpResponse, KromerError> {
    let uuid = uuid.into_inner();
    let pool = &state.pool;

    let wallet = Wallet::fetch_by_id(pool, uuid).await?;

    // // Maybe not the best? maybe censor? idk.
    Ok(HttpResponse::Ok().json(json!({
        "wallet": wallet
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            .service(wallet_create)
            .service(wallet_give_money)
            .service(wallet_get_by_uuid),
    );
}
