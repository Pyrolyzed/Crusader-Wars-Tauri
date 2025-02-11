#[tauri::command]
pub fn wait_for_battle_start(app: tauri::AppHandle, state: tauri::State<'_, Mutex<state::AppState>>) {
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
    wait_for_battle_end(app.clone(), state.clone());
}


#[tauri::command]
pub fn wait_for_battle_end(app: tauri::AppHandle, state: tauri::State<'_, Mutex<state::AppState>>) {

}
