use actix_web::{HttpResponse, get, web};
use sea_query::{Asterisk, Expr, MysqlQueryBuilder, Query};

use crate::AppState;
use crate::database::wallet::{Model as WalletModel, Wallet};
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

    let count = Query::select()
        .expr(Expr::col((Wallet::Table, Wallet::Id)).count())
        .to_string(MysqlQueryBuilder);

    let q = Query::select()
        .column(Asterisk)
        .from(Wallet::Table)
        .limit(limit)
        .offset(offset)
        .to_string(MysqlQueryBuilder);
    let wallets: Vec<WalletModel> = sqlx::query_as(&q).fetch_all(&state.pool).await?;
    let wallets: Vec<AddressJson> = wallets.into_iter().map(|wallet| wallet.into()).collect();

    Ok(HttpResponse::Ok().json(wallets))
}

#[get("/{address}")]
async fn wallet_get(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, KristError> {
    let address = address.into_inner();

    let q = Query::select()
        .column(Asterisk)
        .from(Wallet::Table)
        .and_where(Expr::col(Wallet::Address).eq(&address))
        .to_string(MysqlQueryBuilder);
    let wallet: Option<WalletModel> = sqlx::query_as(&q).fetch_optional(&state.pool).await?;

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

    let q = Query::select()
        .column(Asterisk)
        .from(Wallet::Table)
        .order_by(Wallet::Balance, sea_query::Order::Desc)
        .limit(limit)
        .offset(offset)
        .to_string(MysqlQueryBuilder);
    let wallets: Vec<WalletModel> = sqlx::query_as(&q).fetch_all(&state.pool).await?;
    let wallets: Vec<AddressJson> = wallets.into_iter().map(|wallet| wallet.into()).collect();

    Ok(HttpResponse::Ok().json(wallets))
}

#[get("/{address}/transactions")]
async fn wallet_get_transactions(
    state: web::Data<AppState>,
    address: web::Path<String>,
    params: web::Query<AddressTransactionQuery>,
) -> Result<HttpResponse, KristError> {
    let mut conn = state.pool.acquire().await?;

    todo!()
}

#[get("/{address}/names")]
async fn wallet_get_names(
    state: web::Data<AppState>,
    address: web::Path<String>,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let mut conn = state.pool.acquire().await?;

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
