use std::path::Path;
use std::ffi::OsStr;
use sysinfo::System;
use std::sync::Mutex;

use crate::state;

#[tauri::command]
pub fn is_ck_running() -> bool {
    let system = System::new_all();
    system.processes_by_exact_name("Main Thread".as_ref()).next().is_some() || system.processes_by_exact_name("ck3.exe".as_ref()).next().is_some()
}

#[tauri::command]
pub fn is_attila_running() -> bool {
    let system = System::new_all();
    system.processes_by_exact_name("attila".as_ref()).next().is_some() || system.processes_by_exact_name("attila.exe".as_ref()).next().is_some()
}

#[tauri::command]
pub fn verify_game_paths(ck_str: String, attila_str: String, state: tauri::State<'_, Mutex<state::AppState>>) -> bool {
    let ck_path: &Path = Path::new(&ck_str);
    let attila_path: &Path = Path::new(&attila_str);
    let valid_attila: bool = attila_path.file_stem() == Some(OsStr::new("attila")) || attila_path.extension() == Some(OsStr::new("exe"));
    let valid_ck3: bool = ck_path.extension() == Some(OsStr::new("exe")) || ck_path.file_stem() == Some(OsStr::new("ck3"));

    if valid_attila && valid_ck3 {
        let mut app_state = state.lock().unwrap();
        app_state.attila_path = attila_str;
        app_state.ck_path = ck_str;
        app_state.is_valid_paths = true;
    }

    valid_attila && valid_ck3
}
