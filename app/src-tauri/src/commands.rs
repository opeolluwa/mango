use crate::{database, LAME_SIDECAR, MEDIA_DIR, MODEL_CONFIG_FILE};
use libaudify::core::Audify;
use libaudify::error::AudifyError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::thread;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager};
use tauri::{Runtime, State};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use ts_rs::TS;
use walkdir::WalkDir;

// use crate::adapters::AudioBook;
use crate::database::ModelTrait;
use crate::error::CommandError;
use crate::state::AppState;
use futures::future::join_all;
use rayon::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::sync::Arc;
use crate::adapters::AudioLibrary;

#[tauri::command]
pub async fn synthesize_audio<R: Runtime>(
    pdf_path: &str,
    app_handle: AppHandle,
    _window: tauri::Window<R>,
    state: State<'_, Arc<AppState>>,
) -> Result<(), CommandError> {
    println!("Received PDF path: {pdf_path}");

    let config_path = app_handle
        .path()
        .resolve(*MODEL_CONFIG_FILE, BaseDirectory::Resource)
        .map_err(|_| AudifyError::PathResolutionError)?;

    let config_path_str = config_path
        .to_str()
        .ok_or(AudifyError::PathResolutionError)?;

    let audify_rs = Audify::new(config_path_str);
    let canonical_file_path = PathBuf::from(pdf_path);
    let file_name = canonical_file_path
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or(AudifyError::FileNameError)
        .unwrap()
        .to_string();

    // app_handle
    //     .emit(
    //         "processing-audio",
    //         AudioSynthesisEvent {
    //             file_name: file_name.clone(),
    //             ..Default::default()
    //         },
    //     )
    //     .unwrap();

    //obtain the wav file
    let audio_output = format!("{}/{}.wav", MEDIA_DIR.as_str(), file_name);
    audify_rs
        .synthesize_pdf(pdf_path, &audio_output)
        .map_err(|e| AudifyError::SynthesisError(e.to_string()))?;

    //CONVERT TO MP3
    transcode_wav_to_mp3(app_handle.clone(), &audio_output).await;
    delete_file_if_exists(&audio_output).unwrap();

    // save the file to the database
    let book = database::AudioBook::new(Some(file_name.clone()));
    let pool = state.db.clone();
    book.save(&pool).await?;

    // app_handle
    //     .emit(
    //         "finished-processing-audio",
    //         AudioSynthesisEvent {
    //             file_name,
    //             audio_src: audio_output,
    //         },
    //     )
    //     .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn sync_playlist(state: State<'_, Arc<AppState>>) -> Result<(), CommandError> {
    let local_audio_books = WalkDir::new(&format!("{}/", MEDIA_DIR.as_str()))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().unwrap_or_default() == "mp3")
        .filter_map(|entry| {
            let file_name = entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            Option::from(file_name)
        })
        .collect::<Vec<String>>();

    println!("{:?}", local_audio_books);
    
    let pool = state.db.clone();
    let handler = database::AudioBook::default();
    let cached_audio_book = handler.find_all(&pool).await?;

    let cached_audio_book_names = cached_audio_book
        .into_iter()
        .map(|book| book.title.unwrap_or_default())
        .collect::<Vec<String>>();

    println!("{:?}", cached_audio_book_names);

    for file_name in &local_audio_books {
        if !cached_audio_book_names.contains(file_name) {
            if let Err(e) = database::AudioBook::new(Some(file_name.clone()))
                .save(&pool)
                .await
            {
                eprintln!("Error saving book: {}", e);
            }
        }
    }
    
    Ok(())
}
#[tauri::command]
pub async fn read_library(
    state: State<'_, Arc<AppState>>,
) -> Result<AudioLibrary, CommandError> {
    sync_playlist(state.clone()).await?;

    let pool = state.db.clone();
    let handler = database::AudioBook::default();
    let cached_audio_book = handler.find_all(&pool).await?;

    Ok(AudioLibrary::new(cached_audio_book))
}

#[tauri::command]
//todo: use the primary key from the database
pub async fn play_audio_book(
    book_title: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), CommandError> {
    log::info!("playing {}", book_title);
    let audio_book_canonical_path = format!("{}/{}", MEDIA_DIR.as_str(), book_title);
    let state = state.inner().clone();

    //play the file in a new thread
    thread::spawn(move || {
        let file = match File::open(audio_book_canonical_path) {
            Ok(file) => file,
            Err(err) => return Err(CommandError::from(err.to_string())),
        };

        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(err) => return Err(CommandError::from(err.to_string())),
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => Arc::new(sink),
            Err(err) => return Err(CommandError::from(err.to_string())),
        };

        match Decoder::new(BufReader::new(file)) {
            Ok(source) => sink.append(source),
            Err(err) => return Err(CommandError::from(err.to_string())),
        }

        {
            let mut current_audio_book = state.current_audio_book.lock().unwrap();
            if let Some(ref audio_book) = *current_audio_book {
                audio_book.pause();
            }
            *current_audio_book = Some(sink.clone());
        }

        sink.set_volume(1.0);
        sink.sleep_until_end();
        Ok(())
    });

    Ok(())
}

#[tauri::command]
pub async fn pause_audio_book(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let current_audio_book = state.current_audio_book.lock().unwrap();
    if let Some(ref audio_book) = *current_audio_book {
        audio_book.pause();
    }

    Ok(())
}

#[tauri::command]
pub async fn resume_playing_audio_book(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    println!("Resuming audio book");
    let current_audio_book = state.current_audio_book.lock().unwrap();
    if let Some(ref audio_book) = *current_audio_book {
        match audio_book.is_paused() {
            true => {
                println!("Resuming audio book");
                audio_book.play()
            }
            false => {
                println!("Audio book is already playing");
                return Ok(());
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn set_audio_book_volume(
    volume: f32,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let current_audio_book = state.current_audio_book.lock().unwrap();
    if let Some(ref audio_book) = *current_audio_book {
        audio_book.set_volume(volume);
    }
    Ok(())
}

#[tauri::command]
pub async fn seek_audio_book_to_position() -> Result<(), String> {
    todo!()
}

#[tauri::command]
pub async fn set_audio_book_playback_speed() -> Result<(), String> {
    todo!()
}

async fn transcode_wav_to_mp3(app: tauri::AppHandle, file_name: &str) {
    let window = app.get_webview_window("main").unwrap();

    let sidecar_command = app.shell().sidecar(LAME_SIDECAR).unwrap().args([file_name]);

    let (mut rx, mut child) = sidecar_command.spawn().unwrap();

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line_bytes) = event {
            let line = String::from_utf8_lossy(&line_bytes);

            // Now this will work
            window
                .emit("audio-encoded", Some(format!("{}", line)))
                .expect("failed to emit event");

            // Optional: write to stdin
            child.write(b"message from Rust\n").unwrap();
        }
    }
}

fn delete_file_if_exists(path: &str) -> io::Result<()> {
    let file_path = Path::new(path);
    if file_path.exists() {
        fs::remove_file(file_path)?;
    }
    Ok(())
}
