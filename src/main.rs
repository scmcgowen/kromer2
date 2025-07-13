use actix_web::{App, HttpServer, middleware, web};
use sqlx::postgres::PgPool;
use std::env;

use kromer::{AppState, routes, websockets::WebSocketServer};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&database_url).await?;

    let krist_ws_server = WebSocketServer::new();
    let state = web::Data::new(AppState { pool });

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::Data::new(krist_ws_server.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(routes::config)
        // .default_service(web::route().to(routes::not_found::not_found))
    })
    .bind(&server_url)?
    .run();

    http_server.await?;

    Ok(())
}
