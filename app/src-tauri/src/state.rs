use rodio::Sink;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub current_audio_book: Mutex<Option<Arc<Sink>>>,
}
