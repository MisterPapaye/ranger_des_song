pub mod audio;
pub mod models;
pub mod api;
pub mod commands;

pub use tauri;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::select_source_folder,
            commands::select_destination_folder,
            commands::start_organization,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
