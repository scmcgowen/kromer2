use actix_web::{HttpResponse, get, web};
use uuid::Uuid;

use crate::database::ModelExt;
use crate::database::player::Model as Player;

use crate::errors::player::PlayerError;
use crate::models::kromer::responses::ApiResponse;
use crate::models::kromer::wallets::Wallet as WalletResponse;
use crate::{AppState, errors::KromerError};

#[get("/by-player/{uuid}")]
async fn wallet_get_by_uuid(
    state: web::Data<AppState>,
    uuid: web::Path<Uuid>,
) -> Result<HttpResponse, KromerError> {
    let uuid = uuid.into_inner();
    let pool = &state.pool;

    let mut tx = pool.begin().await?;

    let player = Player::fetch_by_id(&mut *tx, &uuid)
        .await?
        .ok_or_else(|| KromerError::Player(PlayerError::NotFound))?;
    let owned_wallets = player.owned_wallets(&mut *tx).await?;

    tx.commit().await?;

    let sanitized_wallets: Vec<WalletResponse> = owned_wallets
        .into_iter()
        .map(|wallet| wallet.into())
        .collect();

    let response = ApiResponse {
        data: Some(sanitized_wallets),
        ..Default::default()
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/by-name/{name}")]
async fn wallet_get_by_name(
    state: web::Data<AppState>,
    name: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let name = name.into_inner();
    let pool = &state.pool;

    let mut tx = pool.begin().await?;

    let player = Player::fetch_by_name(&mut *tx, name)
        .await?
        .ok_or_else(|| KromerError::Player(PlayerError::NotFound))?;
    let owned_wallets = player.owned_wallets(&mut *tx).await?;

    tx.commit().await?;

    let sanitized_wallets: Vec<WalletResponse> = owned_wallets
        .into_iter()
        .map(|wallet| wallet.into())
        .collect();

    let response = ApiResponse {
        data: Some(sanitized_wallets),
        ..Default::default()
    };

    Ok(HttpResponse::Ok().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            .service(wallet_get_by_name)
            .service(wallet_get_by_uuid),
    );
}
