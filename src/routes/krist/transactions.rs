use actix_web::{HttpResponse, get, post, web};
use rust_decimal::dec;

use crate::database::ModelExt;
use crate::database::transaction::{Model as Transaction, TransactionCreateData, TransactionType};
use crate::database::wallet::Model as Wallet;

use crate::errors::krist::address::AddressError;
use crate::errors::krist::generic::GenericError;
use crate::errors::krist::transaction::TransactionError;
use crate::models::transactions::{
    TransactionDetails, TransactionJson, TransactionListResponse, TransactionResponse,
};
use crate::models::websockets::{WebSocketEvent, WebSocketMessage};
use crate::websockets::WebSocketServer;
use crate::{AppState, errors::krist::KristError, routes::PaginationParams};

#[get("")]
pub async fn transaction_list(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let params = query.into_inner();
    let pool = &state.pool;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let mut tx = pool.begin().await?;

    let total_transaction = Transaction::total_count(&mut *tx).await?;
    let transactions = Transaction::fetch_all(&mut *tx, limit, offset).await?;

    tx.commit().await?;

    let transactions: Vec<TransactionJson> =
        transactions.into_iter().map(|trans| trans.into()).collect();

    let response = TransactionListResponse {
        ok: true,
        count: transactions.len(),
        total: total_transaction,
        transactions,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[post("")]
async fn transaction_create(
    state: web::Data<AppState>,
    server: web::Data<WebSocketServer>,
    details: web::Json<TransactionDetails>,
) -> Result<HttpResponse, KristError> {
    let pool = &state.pool;
    let details = details.into_inner();
    let amount = details.amount.round_dp(2); // Do not allow more than 2 decimals after the dot.

    // Check on the server so DB doesnt throw.
    if amount <= dec!(0.00) {
        return Err(KristError::Generic(GenericError::InvalidParameter(
            "amount".to_string(),
        )));
    }

    let sender_verify_response = Wallet::verify_address(pool, details.private_key).await?;
    let sender = sender_verify_response.model;

    let recipient = Wallet::fetch_by_address(pool, &details.to)
        .await?
        .ok_or_else(|| KristError::Address(AddressError::NotFound(details.to)))?;

    // Make sure to check the request to see if the funds are available.
    if sender.balance < amount {
        return Err(KristError::Transaction(TransactionError::InsufficientFunds));
    }

    if sender.address == recipient.address {
        return Err(KristError::Transaction(
            TransactionError::SameWalletTransfer,
        ));
    }

    let creation_data = TransactionCreateData {
        from: sender.address,
        to: recipient.address,
        amount,
        metadata: details.metadata,
        transaction_type: TransactionType::Transfer,
    };
    let transaction = Transaction::create(pool, creation_data).await?;
    let transaction_json: TransactionJson = transaction.into();

    // TODO: WebSockets.
    let event = WebSocketMessage::new_event(WebSocketEvent::Transaction {
        transaction: transaction_json.clone(),
    });
    server.broadcast_event(event).await;

    let final_response = TransactionResponse {
        ok: true,
        transaction: transaction_json,
    };

    Ok(HttpResponse::Ok().json(final_response))
}

#[get("/latest")]
async fn transaction_latest(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let params = query.into_inner();
    let pool = &state.pool;

    let total = Transaction::total_count(pool).await?;
    let transactions = Transaction::sorted_by_date(pool, &params).await?;

    let transactions: Vec<TransactionJson> =
        transactions.into_iter().map(|trans| trans.into()).collect();

    let response = TransactionListResponse {
        ok: true,
        count: transactions.len(),
        total,
        transactions,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{id}")]
async fn transaction_get(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, KristError> {
    let id = id.into_inner();
    let pool = &state.pool;

    let slim = Transaction::fetch_by_id(pool, id).await?;

    slim.map(|trans| TransactionResponse {
        ok: true,
        transaction: trans.into(),
    })
    .map(|response| HttpResponse::Ok().json(response))
    .ok_or_else(|| KristError::Transaction(TransactionError::NotFound))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .service(transaction_create)
            .service(transaction_latest)
            .service(transaction_get)
            .service(transaction_list),
    );
}
