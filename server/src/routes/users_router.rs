use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::{
    handlers::{
        authentication_handlers::change_password,
        users_handlers::{
            retrieve_information, update_password, update_profile, update_profile_picture,
        },
    },
    states::services_state::ServicesState,
};

pub(super) fn user_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .route("/profile", put(update_profile))
        .route("/avatar", post(update_profile_picture))
        .route("/password", put(update_password))
        .route("/password", patch(change_password))
        .with_state(state)
}
