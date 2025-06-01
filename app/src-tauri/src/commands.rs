use std::path::{Path, PathBuf};

use libaudify::core::Audify;
use libaudify::error::AudifyError;
use serde::{Deserialize, Serialize};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use ts_rs::TS;
use walkdir::WalkDir;
use crate::{MEDIA_DIR, MODEL_CONFIG_FILE, LAME_SIDECAR};
use tauri::Runtime;

use std::fs;
use std::io;



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioBook {
    pub file_name: String,
    pub play_back_duration: u64,
    pub audio_src: String,
}

impl AudioBook {
    pub fn from_path(path: &Path) -> Option<Self> {
        let path_buf = path.canonicalize().ok()?;
        let file_name = path_buf.file_name()?.to_str()?.to_string();

        //TODO: Remove ".wav" if it exists
        let display_name = file_name.strip_suffix(".wav").unwrap_or(&file_name);

        Some(Self {
            file_name: display_name.to_string(),
            play_back_duration: 45, // Consider computing real duration later
            audio_src:path.canonicalize().ok()?.to_str().unwrap().to_string() ,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioLibrary {
    pub audio_books: Vec<AudioBook>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioSynthesisEvent {
    pub file_name: String,
    pub audio_src: String,
}

#[tauri::command]
pub async fn synthesize_audio<R: Runtime>(pdf_path: &str, app_handle: AppHandle, _window: tauri::Window<R>) -> Result<(), AudifyError> {
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

    app_handle
        .emit(
            "processing-audio",
            AudioSynthesisEvent {
                file_name: file_name.clone(),
                ..Default::default()
            },
        )
        .unwrap();

    //obtain the wav file
    let audio_output = format!("{}/{}.wav", MEDIA_DIR.as_str(), file_name);
    audify_rs
        .synthesize_pdf(pdf_path, &audio_output)
        .map_err(|e| AudifyError::SynthesisError(e.to_string()))?;

    //CONVERT TO MP3
 transcode_wav_to_mp3(app_handle.clone(), &audio_output).await;
 delete_file_if_exists(&audio_output).unwrap();


    app_handle
        .emit(
            "finished-processing-audio",
            AudioSynthesisEvent {
                file_name,
                audio_src: audio_output,
            },
        )
        .unwrap();


    Ok(())
}

#[tauri::command]
pub fn read_library() -> AudioLibrary {
    let mut audio_books = WalkDir::new(&format!("{}/", MEDIA_DIR.as_str()))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| AudioBook::from_path(entry.path()))
        .collect::<Vec<AudioBook>>();

    audio_books.remove(0);
    let library = AudioLibrary { audio_books };
    library
}


async fn transcode_wav_to_mp3(app: tauri::AppHandle, file_name: &str) {

    let window = app.get_webview_window("main").unwrap();

    let sidecar_command = app
        .shell()
        .sidecar(LAME_SIDECAR)
        .unwrap()
        .args([ file_name]);

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