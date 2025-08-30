use axum::{Router, routing::get};

use crate::{
    handlers::notification_handlers::{
        count_unread, fetch_notifications, listen_for_new_notifications,
    },
    states::services_state::ServicesState,
};

pub(super) fn notification_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", get(fetch_notifications))
        .route("/listen", get(listen_for_new_notifications))
        .route("/unread", get(count_unread))
        .with_state(state)
}
