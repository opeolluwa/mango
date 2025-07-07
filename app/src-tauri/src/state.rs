// use rodio::Sink;
use sqlx::{Pool, Sqlite};
use std::sync::{Arc, Mutex};
pub struct AppState {
    // pub current_audio_book: Mutex<Option<Arc<Sink>>>,
    pub db: Arc<Pool<Sqlite>>,
}
