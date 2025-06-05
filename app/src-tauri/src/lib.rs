// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use dirs;
use lazy_static::lazy_static;
mod commands;

mod state;

lazy_static! {
    pub static ref MODEL_CONFIG_FILE: &'static str = "resources/en_US-libritts_r-medium.onnx.json";
    pub static ref MEDIA_DIR: String = {
        let Some(os_audio_dir) = dirs::audio_dir() else {
            todo!()
        };

        let os_audio_dir = os_audio_dir.as_path().to_str().unwrap();
        let audify_folder = "audify";

        let media_dir = format!("{os_audio_dir}/{audify_folder}");
        let _ = std::fs::create_dir(&media_dir);
        media_dir.clone()
    };
}

pub const LAME_SIDECAR: &str = "lame";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(desktop)]
            let _ = app
                .handle()
                .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::synthesize_audio,
            commands::read_library
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
