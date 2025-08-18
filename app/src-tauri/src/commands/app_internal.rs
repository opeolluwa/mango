use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager, State};

use crate::state::AppState;

#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    task: String,
) -> Result<(), ()> {
    // Lock the state without write access
    let mut setup_state = state.lock().unwrap();
    match task.as_str() {
        "frontend" => setup_state.setup.frontend_task = true,
        "backend" => setup_state.setup.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // Check if both tasks are completed
    if setup_state.setup.backend_task && setup_state.setup.frontend_task {
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        // splash_window.close().unwrap();
        // main_window.show().unwrap();
    }
    Ok(())
}

pub 
