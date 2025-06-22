use actix_web::{App, HttpServer, middleware, web};
use sqlx::mysql::MySqlPool;
use std::env;

use maria_kromer::{AppState, routes};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = MySqlPool::connect(&database_url).await?;

    let state = web::Data::new(AppState { pool });

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
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
