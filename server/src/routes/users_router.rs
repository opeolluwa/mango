use axum::{
    Router,
    routing::{get, post, put},
};

use crate::{
    handlers::users_handlers::{
        retrieve_information, update_password, update_profile, update_profile_picture,
    },
    states::services_state::ServicesState,
};

pub(super) fn user_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .route("/profile", put(update_profile))
        .route("/avatar", post(update_profile_picture))
        .route("/password", put(update_password))
        .with_state(state)
}
