use axum::{
    Router,
    routing::{get, post, put},
};

use crate::{
    handlers::audio_books_handlers::{create_new_book, fetch_book, update_book},
    states::services_state::ServicesState,
};

pub(super) fn audio_book_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", post(create_new_book))
        .route("/", put(update_book))
        .route("/", get(fetch_book))
        .with_state(state)
}
