use sqlx::{MySql, Pool};

pub mod database;
pub mod errors;
pub mod models;
pub mod routes;
pub mod utils;
pub mod websockets;

#[derive(Debug)]
pub struct AppState {
    pub pool: Pool<MySql>,
}
