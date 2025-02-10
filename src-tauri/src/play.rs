use std::sync::Mutex;

use crate::{state, processes};
use tauri::Emitter;
use std::process::Command;

#[tauri::command]
pub fn play(app: tauri::AppHandle, state: tauri::State<'_, Mutex<state::AppState>>) -> bool {
    
    // -- Check if the game paths are valid
    // -- Check if attila is already running (bad)
    // -- Check if CK3 is already running (fine)
    // Boot CK3 if it's not running
    // Prompt to close attila if it's running
    // Wait for battle (different function)

    // The status of CK3 and attila will change with time, so it needs to be a mutable variable
    let mut is_ck_running: bool = processes::is_ck_running();
    let mut is_attila_running: bool = processes::is_attila_running();

    // Make sure the game paths are valid by checking state, fail if not
    let is_valid: bool = state.lock().unwrap().is_valid_paths;
    if (!is_valid) {
        app.emit("play-result", "invalid-paths").unwrap();
        false
    }

    if (!is_ck_running) {
        Command::new(state.lock.unwrap().ck_path).output().expect("Failed to execute CK")
    }


    // Return true if successful
    app.emit("play-result", "success").unwrap();
    true
}
