#![warn(unused_extern_crates)]

use aers_lib::{
    AERS_EXPORT_PATH, AERS_FILE_UPLOAD_PATH, errors, events::channels::RedisMessageChannel, routes,
    shared,
};
use axum::extract::DefaultBodyLimit;
use errors::app_error::AppError;
use redis::AsyncCommands;
use redis::aio::PubSub;
use routes::router::load_routes;
use shared::extract_env::extract_env;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::Path,
};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    initialize_file_systems()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = extract_env::<String>("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;
    log::info!("Database initialized");

    let migrator = Migrator::new(Path::new("migrations"))
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;
    migrator
        .run(&pool)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    let app = load_routes(pool)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            25 * 1024 * 1024, //25mb
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        ); //TODO: restrict to tauri url

    let port = extract_env::<u16>("PORT")?;
    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    log::info!("Application listening on http://{}", ip_address);

    let listener = tokio::net::TcpListener::bind(ip_address)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    tokio::spawn(async move {
        let redis_url = extract_env::<String>("REDIS_CONNECTION_URL").unwrap();
        let client = redis::Client::open(redis_url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        loop {
            let msg = pubsub
                .subscribe(RedisMessageChannel::FileConverted.to_string())
                // .await
                .map_err(|err| {
                    log::error!("failed to subscribe to Redis channel due to {}", err);
                    AppError::OperationFailed(err.to_string())
                });
        }
    });

    Ok(())
}

fn initialize_file_systems() -> Result<(), AppError> {
    std::fs::create_dir_all(AERS_FILE_UPLOAD_PATH).map_err(|err| {
        log::error!("failed to create AERS_FILE_UPLOAD_PATH due to {}", err);
        AppError::OperationFailed("failed to create AERS_FILE_UPLOAD_PATH".to_string())
    })?;
    std::fs::create_dir_all(AERS_EXPORT_PATH).map_err(|err| {
        log::error!("failed to create AERS_EXPORT_PATH due to {}", err);
        AppError::OperationFailed("failed to create AERS_EXPORT_PATH".to_string())
    })?;

    Ok(())
}
