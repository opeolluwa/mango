use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, AddBookToPlaylistResponse, CreateAudioBookRequest,
    CreateAudioBookResponse, DeleteBookRequest, DeleteBookResponse, MarkFavouriteRequest,
    MarkFavouriteResponse, RemoveBookFromPlaylistRequest, RemoveBookFromPlaylistResponse,
    UpdateBookRequest, UpdateBookResponse,
};
use crate::errors::common_service_error::ServiceError;
use crate::repositories::audio_book_repository::{AudioBookRepository, AudioBookRepositoryExt};
use sqlx::Postgres;
use sqlx::pool::Pool;

#[derive(Clone)]
pub struct AudioBooksService {
    audio_book_repository: AudioBookRepository,
}

pub trait AudioBooksServiceExt {
    async fn create_new(
        &self,
        request: CreateAudioBookRequest,
    ) -> Result<CreateAudioBookResponse, ServiceError>;

    async fn add_to_playlist(
        &self,
        request: AddBookToPlaylistRequest,
    ) -> Result<AddBookToPlaylistResponse, ServiceError>;

    async fn remove_from_playlist(
        &self,
        request: RemoveBookFromPlaylistRequest,
    ) -> Result<RemoveBookFromPlaylistResponse, ServiceError>;

    async fn update_book(
        &self,
        request: UpdateBookRequest,
    ) -> Result<UpdateBookResponse, ServiceError>;

    async fn delete_book(
        &self,
        request: DeleteBookRequest,
    ) -> Result<DeleteBookResponse, ServiceError>;

    async fn mark_favourite(
        &self,
        request: MarkFavouriteRequest,
    ) -> Result<MarkFavouriteResponse, ServiceError>;
}

impl AudioBooksService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            audio_book_repository: AudioBookRepository::init(pool),
        }
    }
}

impl AudioBooksServiceExt for AudioBooksService {
    async fn create_new(
        &self,
        request: CreateAudioBookRequest,
    ) -> Result<CreateAudioBookResponse, ServiceError> {
        todo!()
    }

    async fn add_to_playlist(
        &self,
        request: AddBookToPlaylistRequest,
    ) -> Result<AddBookToPlaylistResponse, ServiceError> {
        todo!()
    }

    async fn remove_from_playlist(
        &self,
        request: RemoveBookFromPlaylistRequest,
    ) -> Result<RemoveBookFromPlaylistResponse, ServiceError> {
        todo!()
    }

    async fn update_book(
        &self,
        request: UpdateBookRequest,
    ) -> Result<UpdateBookResponse, ServiceError> {
        todo!()
    }

    async fn delete_book(
        &self,
        request: DeleteBookRequest,
    ) -> Result<DeleteBookResponse, ServiceError> {
        todo!()
    }

    async fn mark_favourite(
        &self,
        request: MarkFavouriteRequest,
    ) -> Result<MarkFavouriteResponse, ServiceError> {
        todo!()
    }
}
