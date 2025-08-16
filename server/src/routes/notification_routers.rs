use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::notification_handlers::listen_for_new_notifications,
    states::services_state::ServicesState,
};

pub(super) fn notification_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/listen", get(listen_for_new_notifications))
        .with_state(state)
}
