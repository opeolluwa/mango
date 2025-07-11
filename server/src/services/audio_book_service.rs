use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, AddBookToPlaylistResponse, CreateAudioBookRequest,
    CreateAudioBookResponse, DeleteBookRequest, DeleteBookResponse, MarkFavouriteRequest,
    MarkFavouriteResponse, RemoveBookFromPlaylistRequest, RemoveBookFromPlaylistResponse,
    UpdateBookRequest, UpdateBookResponse,
};
use crate::errors::common_service_error::ServiceError;
use crate::repositories::audio_book_repository::AudioBookRepository;
use sqlx::Postgres;
use sqlx::pool::Pool;

#[derive(Clone)]
pub struct AudioBooksService {
    audio_book_repository: AudioBookRepository,
}

pub trait AudioBooksServiceExt {
    fn create_new(
        &self,
        request: CreateAudioBookRequest,
    ) -> impl std::future::Future<Output = Result<CreateAudioBookResponse, ServiceError>> + Send;

    fn add_to_playlist(
        &self,
        request: AddBookToPlaylistRequest,
    ) -> impl std::future::Future<Output = Result<AddBookToPlaylistResponse, ServiceError>> + Send;

    fn remove_from_playlist(
        &self,
        request: RemoveBookFromPlaylistRequest,
    ) -> impl std::future::Future<Output = Result<RemoveBookFromPlaylistResponse, ServiceError>> + Send;

    fn update_book(
        &self,
        request: UpdateBookRequest,
    ) -> impl std::future::Future<Output = Result<UpdateBookResponse, ServiceError>> + Send;

    fn delete_book(
        &self,
        request: DeleteBookRequest,
    ) -> impl std::future::Future<Output = Result<DeleteBookResponse, ServiceError>> + Send;

    fn mark_favourite(
        &self,
        request: MarkFavouriteRequest,
    ) -> impl std::future::Future<Output = Result<MarkFavouriteResponse, ServiceError>> + Send;
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
