use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::audio_books_handlers::{
        create_new_book, delete_book, fetch_book, fetch_favourites, find_many_books,
        mark_favourite, unmark_favourite, update_book,
    },
    states::services_state::ServicesState,
};

pub(super) fn audio_book_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", post(create_new_book))
        .route("/", get(find_many_books))
        .route("/{book_identifier}", put(update_book))
        .route("/{book_identifier}", get(fetch_book))
        .route("/{book_identifier}", delete(delete_book))
        .route("/{book_identifier}/favourite", post(mark_favourite))
        .route("/{book_identifier}/favourites", put(unmark_favourite))
        .route("/favourites", get(fetch_favourites))
        .with_state(state)
}
