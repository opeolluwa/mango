use axum::{
    Router,
    routing::{any, get},
};

use crate::{
    handlers::notification_handlers::listen_for_new_notifications,
    states::services_state::ServicesState,
};

pub(super) fn notification_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/listen", any(listen_for_new_notifications))
        .route("/", any(listen_for_new_notifications))
        .with_state(state)
}
