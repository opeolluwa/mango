use aers_lib::{
    config::{
        AppConfig,
        app::{create_cors_layer, shutdown_signal},
        database::AppDatabase,
        filesystem::AppFileSystem,
    },
    errors,
    events::subscriber::{EventSubscriber, EventSubscriberExt},
    routes,
};
use axum::extract::DefaultBodyLimit;
use errors::app_error::AppError;
use routes::router::load_routes;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::from_env()?;
    AppFileSystem::init(&config)?;
    let db_pool = AppDatabase::init(&config).await?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .compact()
        .init();

    let db = std::sync::Arc::new(db_pool);

    let app = load_routes(db.clone())
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            config.body_limit_mb * 1024 * 1024,
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(create_cors_layer(&config)); // Use config-based CORS

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port));
    tracing::info!("Application listening on http://{ip_address}");

    let listener = tokio::net::TcpListener::bind(ip_address)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    tokio::spawn(async move {
        if let Err(err) = EventSubscriber::start_redis_listener(db).await {
            tracing::error!("Redis listener failed: {err}");
        }
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
