mod audio;
mod clipboard;
mod commands;
mod config;
mod errors;
mod hotkey;
mod models;
mod permissions;
mod sounds;
mod state;
mod system_info;
mod transcription;
mod tray;

use config::AppConfig;
use state::{AppState, RecordingStream};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_local_data_dir()
                .expect("Impossible d'obtenir le repertoire de donnees");

            std::fs::create_dir_all(&data_dir)
                .expect("Impossible de creer le repertoire de donnees");

            let config = AppConfig::load(&data_dir).unwrap_or_default();
            let state = AppState::new(config, data_dir);

            app.manage(state);
            app.manage(RecordingStream::new());

            tray::setup_tray(app.handle()).expect("Impossible de creer le tray");

            if let Err(e) = hotkey::register_all(app.handle()) {
                log::error!("Enregistrement raccourcis impossible : {}", e);
            }

            // Start hidden from dock (tray-only)
            #[cfg(target_os = "macos")]
            tray::set_dock_visible(false);

            // Hide window on close instead of quitting — the app lives in the tray.
            // Also toggle dock visibility: show in dock when window is open, hide when closed.
            let window = app.get_webview_window("main").unwrap();
            let w = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = w.hide();
                    #[cfg(target_os = "macos")]
                    tray::set_dock_visible(false);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::is_first_run,
            commands::get_system_info,
            commands::list_audio_devices,
            commands::list_models,
            commands::download_model,
            commands::delete_model,
            commands::load_model,
            commands::update_hotkey,
            commands::suspend_hotkey,
            commands::resume_hotkey,
            commands::update_hotkey_ptt,
            commands::set_auto_paste,
            commands::set_language,
            commands::set_ui_locale,
            commands::set_audio_device,
            commands::test_microphone,
            commands::mark_setup_complete,
            commands::get_recording_state,
            commands::check_permissions,
            commands::request_microphone_permission,
            commands::open_accessibility_preferences,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur fatale lors du lancement de l'application");
}
