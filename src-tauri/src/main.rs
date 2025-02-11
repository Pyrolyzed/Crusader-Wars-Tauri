// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::Mutex;

pub mod processes;
pub mod state;
pub mod battle;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Mutex::new(state::AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![processes::verify_game_paths, processes::is_ck_running, processes::is_attila_running, processes::launch_crusader_kings, processes::close_attila, battle::wait_for_battle_start, battle::wait_for_battle_end])
        .run(tauri::generate_context!())
        .unwrap()
}
