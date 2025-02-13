use std::fs::File;
use std::io::SeekFrom;
use std::time;
use std::path::PathBuf;
use tauri::Manager;
use std::sync::Mutex;
use crate::state;
use std::io::Read;
use std::io::Seek;
fn get_log_file_path_windows(ck_file: String) -> String {
    if let Some(mut documents) = dirs_next::document_dir() {
        documents.push(PathBuf::from("Paradox Interactive\\Crusader Kings III\\console_history.txt"));
        return documents.into_os_string().into_string().unwrap();
    }
    String::from("Failed")
}

fn get_log_file_path_proton(ck_path: String, ck_file: String) -> String {
    ck_path.replace("common/Crusader Kings III/", "compatdata/1158310/pfx/drive_c/users/steamuser/Documents/Paradox Interactive/Crusader Kings III/console_history.txt")
}

fn get_log_file_path_native(ck_file: String) -> String {
    if let Some(mut share) = dirs_next::data_dir() {
        share.push(PathBuf::from("Paradox Interactive/Crusader Kings III/console_history.txt"));
        return share.into_os_string().into_string().unwrap();
    }
    String::from("Failed")
}

fn get_log_file_path(state: tauri::State<'_, Mutex<state::AppState>>) -> String { 
    if let Some(ck) = state.lock().unwrap().ck_path.split("/").last() {
        let ck_file = String::from(ck);

        if cfg!(target_os = "windows") {
            return get_log_file_path_windows(ck_file);
        }

        if ck_file.clone() == "ck3.exe" {
            return get_log_file_path_proton(state.lock().unwrap().ck_path.clone(), ck_file);
        } else {
            return get_log_file_path_native(ck_file);
        }
    }
    String::from("Failed to get log file!")
}

#[tauri::command]
pub fn wait_for_battle_start(app: tauri::AppHandle, state: tauri::State<'_, Mutex<state::AppState>>) {

    let path = get_log_file_path(state.clone());
    let mut is_error = false;
    match File::open(path) {
        Ok(file) => println!("File found."),
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            &mut is_error = true;
        }
    }

    if is_error {
        // Loop function until file found
        wait_for_battle_start(app.clone(), state.clone());
        return;
    }

    let mut file = File::open(path);
    let mut contents = vec![];
    let mut position = 0;

    loop {
        contents.truncate(0);
        file.seek(SeekFrom::Start(position as u64));
        position += file.read_to_end(&mut contents).unwrap();
        dbg!("{}", &contents);
        // if(is_battle_over) {
        //   battle_end(app.clone(), state.clone());
        // }
    }
    // Continuously read file waiting for line "CRUSADERWARS3" to appear
    // Search file for data
        // Get army proportions (battle scale)
        // Search for Date (year)
        // Search for the name of the battle (location)
        // Get modifiers
        // Get participants
        // Get player
        // Get commanders
        // Get Terrain
        // Get Player army
        // get player commander system
        // get player knight system
        // get enemy army
        // get enemy commander system
        // get enemy knight system
        // get army names
    // Read Installed Mods
        // Get data folder path of attila (Attila path + "data/")
        // Get workshop folder path (attila_path/../../workshop/content/325610/)
        // Make list of mods
        // Get all directories in data folder (skip attila packs)
        // Get all directories in workshop folder
        // Set required mods (and optional) to active
    // SetPlaythrough
        // Load unit mappers
        // Find "True" tag and load required mods for it
    // Create User Mods File (Attila side)
        // Create steam_appid.txt file to open attila automatically with correct mods
        // Get all used mods
        // Write to steam_appid.txt file with all required/optional mods.
    // Launch attila
    // Wait for battle to end

    // Log file is in "documents", so a dotfile on linux or in Users/USER/Documents on windows (or
    // in compatdata)
    // let log_file = get_log_file();
}


#[tauri::command]
pub fn wait_for_battle_end(app: tauri::AppHandle, state: tauri::State<'_, Mutex<state::AppState>>) {

}
