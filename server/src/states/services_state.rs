use axum::extract::FromRef;

use crate::services::{
    audio_book_service::AudioBooksService, authentication_service::AuthenticationService,
    notification_service::NotifiactionService, playlist_service::PlaylistService,
    root_serice::RootService, user_service::UserService,
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
    pub playlist_service: PlaylistService,
    pub audio_book_service: AudioBooksService,
    pub notification_service: NotifiactionService,
}

impl FromRef<ServicesState> for UserService {
    fn from_ref(services: &ServicesState) -> UserService {
        services.user_service.clone()
    }
}

impl FromRef<ServicesState> for RootService {
    fn from_ref(services: &ServicesState) -> RootService {
        services.root_service.clone()
    }
}

impl FromRef<ServicesState> for AuthenticationService {
    fn from_ref(services: &ServicesState) -> AuthenticationService {
        services.auth_service.clone()
    }
}

impl FromRef<ServicesState> for PlaylistService {
    fn from_ref(services: &ServicesState) -> PlaylistService {
        services.playlist_service.clone()
    }
}

impl FromRef<ServicesState> for AudioBooksService {
    fn from_ref(services: &ServicesState) -> AudioBooksService {
        services.audio_book_service.clone()
    }
}

impl FromRef<ServicesState> for NotifiactionService {
    fn from_ref(services: &ServicesState) -> NotifiactionService {
        services.notification_service.clone()
    }
}
