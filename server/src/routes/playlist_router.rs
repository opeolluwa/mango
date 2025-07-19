use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::playlist_handlers::{
        create_playlist, delete_playlist, retrieve_playlist, update_playlist,
    },
    states::services_state::ServicesState,
};

pub(super) fn playlist_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", post(create_playlist))
        .route("/{playlist_identifier}", get(retrieve_playlist))
        .route("/{playlist_identifier}", post(update_playlist))
        .route("/{playlist_identifier}", post(delete_playlist))
        .with_state(state)
}
