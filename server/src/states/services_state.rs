use axum::extract::FromRef;

use crate::services::{
    audio_book_service::AudioBooksService, authentication_service::AuthenticationService,
    playlist_service::PlaylistService, root_serice::RootService, user_service::UserService,
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
    pub playlist_service: PlaylistService,
    pub audio_book_service: AudioBooksService,
}

impl FromRef<ServicesState> for UserService {
    fn from_ref(input: &ServicesState) -> UserService {
        input.user_service.clone()
    }
}

impl FromRef<ServicesState> for RootService {
    fn from_ref(input: &ServicesState) -> RootService {
        input.root_service.clone()
    }
}

impl FromRef<ServicesState> for AuthenticationService {
    fn from_ref(input: &ServicesState) -> AuthenticationService {
        input.auth_service.clone()
    }
}

impl FromRef<ServicesState> for PlaylistService {
    fn from_ref(input: &ServicesState) -> PlaylistService {
        input.playlist_service.clone()
    }
}

impl FromRef<ServicesState> for AudioBooksService {
    fn from_ref(input: &ServicesState) -> AudioBooksService {
        input.audio_book_service.clone()
    }
}
