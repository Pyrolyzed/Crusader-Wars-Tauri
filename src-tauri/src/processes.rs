use std::path::Path;
use std::ffi::OsStr;
use sysinfo::System;
use std::sync::Mutex;
use std::process::Command;
use crate::state;
use tauri::Emitter;

#[tauri::command]
pub fn is_ck_running() -> bool {
    let system = System::new_all();
    println!("is ck running: {}", (system.processes_by_exact_name("Main Thread".as_ref()).next().is_some() || system.processes_by_exact_name("ck3.exe".as_ref()).next().is_some()));
    system.processes_by_exact_name("Main Thread".as_ref()).next().is_some() || system.processes_by_exact_name("ck3.exe".as_ref()).next().is_some()
}

#[tauri::command]
pub fn is_attila_running() -> bool {
    let system = System::new_all();
    println!("is attila running: {}", (system.processes_by_exact_name("attila".as_ref()).next().is_some() || system.processes_by_exact_name("attila.exe".as_ref()).next().is_some()));
    system.processes_by_exact_name("attila".as_ref()).next().is_some() || system.processes_by_exact_name("attila.exe".as_ref()).next().is_some()
}

#[tauri::command]
pub fn launch_crusader_kings(app: tauri::AppHandle, state: tauri::State<'_, Mutex<state::AppState>>) {
    if cfg!(target_os = "windows") {
        println!("Launching Windows version of CK");
        Command::new("cmd")
            .args(["/C", &state.lock().unwrap().ck_path.clone()])
            .output()
            .expect("Failed to start CK3!");
    } else {
        println!("Launching linux version of CK");
        Command::new("sh")
            .arg("-c")
            .arg("steam steam://rungameid/1158310")
            .output()
            .expect("Failed to start CK3!");
    }
    app.emit("launching", "ck").unwrap();
}

#[tauri::command]
pub fn close_attila(app: tauri::AppHandle) -> bool {
    app.emit("closing", "attila").unwrap();
    let system = System::new_all();
    if let Some(linux_process) = system.processes_by_exact_name("attila".as_ref()).next() { linux_process.kill(); return true; }
    if let Some(windows_process) = system.processes_by_exact_name("attila.exe".as_ref()).next() { windows_process.kill(); return true; }
    false
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
    println!("valid attila: {}, valid ck3: {}", valid_attila, valid_ck3);
    valid_attila && valid_ck3
}
