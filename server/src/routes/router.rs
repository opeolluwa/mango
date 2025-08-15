use std::sync::Arc;

use axum::{Router, http::StatusCode, response::IntoResponse};
use sqlx::{Pool, Postgres};

use crate::{
    adapters::api_response::ApiResponseBuilder,
    routes::{
        audio_books_router::audio_book_routes, authentication_router::authentication_routes,
        playlist_router::playlist_routes, public_router::public_routes, users_router::user_routes,
    },
    services::{
        audio_book_service::AudioBooksService, authentication_service::AuthenticationService,
        playlist_service::PlaylistService, root_serice::RootService, user_service::UserService,
    },
    states::services_state::ServicesState,
};

pub fn load_routes(pool: Arc<Pool<Postgres>>) -> Router {
    let state = ServicesState {
        user_service: UserService::init(&pool),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(&pool),
        playlist_service: PlaylistService::init(&pool),
        audio_book_service: AudioBooksService::init(&pool),
    };

    Router::new()
        .merge(public_routes(state.clone()))
        .nest("/auth", authentication_routes(state.clone()))
        .nest("/user", user_routes(state.clone()))
        .nest("/books", audio_book_routes(state.clone()))
        .nest("/playlist", playlist_routes(state.clone()))
        .fallback(async || {
            ApiResponseBuilder::<()>::new()
                .message(
                    "the resource you're looking does not exist or it has been permanently moved",
                )
                .status_code(StatusCode::NOT_FOUND)
                .build()
                .into_response()
        })
}
