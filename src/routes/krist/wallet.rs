use actix_web::{HttpResponse, get, web};

use crate::AppState;
use crate::database::ModelExt;
use crate::database::wallet::Model as Wallet;
use crate::errors::krist::KristError;
use crate::errors::krist::address::AddressError;
use crate::models::addresses::{AddressJson, AddressListResponse, AddressResponse};
use crate::models::transactions::AddressTransactionQuery;
use crate::routes::PaginationParams;

#[get("")]
async fn wallet_list(
    state: web::Data<AppState>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let pagination = pagination.into_inner();
    let limit = pagination.limit.unwrap_or(50);
    let offset = pagination.offset.unwrap_or(0);

    let count = Wallet::total_count(&state.pool)
        .await
        .map_err(KristError::Database)?;
    let wallets = Wallet::fetch_all(&state.pool, limit, offset)
        .await
        .map_err(KristError::Database)?;

    let addresses: Vec<AddressJson> = wallets.into_iter().map(|wallet| wallet.into()).collect();

    let list_response = AddressListResponse {
        ok: true,
        count,
        total: addresses.len(),
        addresses,
    };

    Ok(HttpResponse::Ok().json(list_response))
}

#[get("/{address}")]
async fn wallet_get(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, KristError> {
    let address = address.into_inner();

    let wallet = Wallet::fetch_by_address(&state.pool, &address)
        .await
        .map_err(KristError::Database)?;

    wallet
        .map(|addr| AddressResponse {
            ok: true,
            address: addr.into(),
        })
        .map(|response| HttpResponse::Ok().json(response))
        .ok_or_else(|| KristError::Address(AddressError::NotFound(address)))
}

#[get("/rich")]
async fn wallet_richest(
    state: web::Data<AppState>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let pagination = pagination.into_inner();
    let limit = pagination.limit.unwrap_or(50);
    let offset = pagination.offset.unwrap_or(0);

    let total = Wallet::total_count(&state.pool)
        .await
        .map_err(KristError::Database)?;
    let ordered_wallets = Wallet::fetch_richest(&state.pool, limit, offset)
        .await
        .map_err(KristError::Database)?;
    let addresses: Vec<_> = ordered_wallets
        .into_iter()
        .map(|wallet| wallet.into())
        .collect();

    let response = AddressListResponse {
        ok: true,
        count: addresses.len(),
        total,
        addresses: addresses,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{address}/transactions")]
async fn wallet_get_transactions(
    _state: web::Data<AppState>,
    _address: web::Path<String>,
    _params: web::Query<AddressTransactionQuery>,
) -> Result<HttpResponse, KristError> {
    todo!()
}

#[get("/{address}/names")]
async fn wallet_get_names(
    _state: web::Data<AppState>,
    _address: web::Path<String>,
    _params: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    todo!()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/addresses")
            .service(wallet_richest)
            .service(wallet_get)
            .service(wallet_get_transactions)
            .service(wallet_get_names)
            .service(wallet_list),
    );
}
