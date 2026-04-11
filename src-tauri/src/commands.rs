use tauri::{AppHandle, Emitter, State};

use crate::audio::{self, AudioDevice};
use crate::clipboard;
use crate::config::AppConfig;
use crate::errors::AppResult;
use crate::hotkey;
use crate::models::{self, ModelInfo};
use crate::sounds;
use crate::tray;
use crate::permissions::{self, PermissionStatus};
use crate::state::AppState;
use crate::system_info::{self, SystemInfo};
use crate::transcription;
use crate::tts;
use crate::tts_models::{self, TtsModelInfo};

#[tauri::command]
pub fn get_config(state: State<AppState>) -> AppConfig {
    let inner = state.inner.lock().unwrap();
    inner.config.clone()
}

#[tauri::command]
pub fn save_config(state: State<AppState>, config: AppConfig) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    let dir = inner.app_data_dir.clone();
    config.save(&dir).map_err(|e| e.to_string())?;
    inner.config = config;
    Ok(())
}

#[tauri::command]
pub fn is_first_run(state: State<AppState>) -> bool {
    let inner = state.inner.lock().unwrap();
    !inner.config.first_run_complete
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    system_info::get_system_info()
}

#[tauri::command]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    audio::list_input_devices().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_models(state: State<AppState>) -> Vec<ModelInfo> {
    let inner = state.inner.lock().unwrap();
    models::list_models(&inner.app_data_dir)
}

#[tauri::command]
pub async fn download_model(
    app: AppHandle,
    state: State<'_, AppState>,
    model_id: String,
) -> Result<(), String> {
    let dir = {
        let inner = state.inner.lock().unwrap();
        inner.app_data_dir.clone()
    };
    models::download_model(app, dir, model_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_model(state: State<AppState>, model_id: String) -> Result<(), String> {
    let inner = state.inner.lock().unwrap();
    models::delete_model(&inner.app_data_dir, &model_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_model(state: State<AppState>, model_id: String) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    let dir = inner.app_data_dir.clone();

    let path = models::get_model_path(&dir, &model_id)
        .ok_or_else(|| format!("Modèle introuvable : {}", model_id))?;

    let ctx = transcription::load_model(&path).map_err(|e| e.to_string())?;
    inner.whisper_ctx = Some(ctx);
    inner.config.active_model = Some(model_id);
    inner.config.save(&dir).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_hotkey(
    app: AppHandle,
    state: State<AppState>,
    new_hotkey: String,
) -> Result<(), String> {
    {
        let mut inner = state.inner.lock().unwrap();
        inner.config.hotkey = new_hotkey;
        let dir = inner.app_data_dir.clone();
        inner.config.save(&dir).map_err(|e| e.to_string())?;
    }

    // Re-register all hotkeys
    hotkey::unregister_all(&app).map_err(|e| e.to_string())?;
    hotkey::register_all(&app).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_hotkey_ptt(
    app: AppHandle,
    state: State<AppState>,
    new_hotkey: String,
) -> Result<(), String> {
    {
        let mut inner = state.inner.lock().unwrap();
        inner.config.hotkey_ptt = new_hotkey;
        let dir = inner.app_data_dir.clone();
        inner.config.save(&dir).map_err(|e| e.to_string())?;
    }

    // Re-register all hotkeys
    hotkey::unregister_all(&app).map_err(|e| e.to_string())?;
    hotkey::register_all(&app).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn suspend_hotkey(app: AppHandle) -> Result<(), String> {
    hotkey::unregister_all(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn resume_hotkey(app: AppHandle) -> Result<(), String> {
    hotkey::register_all(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_auto_paste(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.auto_paste = enabled;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_verbatim_mode(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.verbatim_mode = enabled;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_live_mode(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.live_mode = enabled;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_language(state: State<AppState>, language: String) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.language = language;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_ui_locale(state: State<AppState>, locale: String) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.ui_locale = locale;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_audio_device(state: State<AppState>, device_name: Option<String>) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.audio_device = device_name;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_setup_complete(state: State<AppState>) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.first_run_complete = true;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_recording_state(state: State<AppState>) -> AppResult<bool> {
    let inner = state.inner.lock().unwrap();
    Ok(inner.is_recording)
}

#[tauri::command]
pub async fn test_microphone(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let device_name = {
        let inner = state.inner.lock().unwrap();
        inner.config.audio_device.clone()
    };
    // Run on a blocking thread so we don't block the async runtime
    tokio::task::spawn_blocking(move || {
        audio::test_microphone(app, device_name)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn check_permissions() -> PermissionStatus {
    permissions::check_permissions()
}

#[tauri::command]
pub fn request_microphone_permission() {
    permissions::request_microphone_permission();
}

#[tauri::command]
pub fn open_accessibility_preferences() {
    permissions::open_accessibility_preferences();
}

// ── File Transcription ──

#[tauri::command]
pub async fn transcribe_file(
    app: AppHandle,
    state: State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    let path = std::path::PathBuf::from(&file_path);
    if !path.exists() {
        return Err("Fichier introuvable".into());
    }

    let (ctx, language, auto_paste, verbatim_mode, app_data_dir) = {
        let inner = state.inner.lock().unwrap();
        (
            inner.whisper_ctx.clone(),
            inner.config.language.clone(),
            inner.config.auto_paste,
            inner.config.verbatim_mode,
            inner.app_data_dir.clone(),
        )
    };

    let inner_arc = state.inner.clone();

    tokio::task::spawn_blocking(move || {
        let _ = app.emit("transcription-started", ());
        tray::start_processing_animation(&app);

        // Load audio file
        let audio_data = match audio::load_audio_file(&path) {
            Ok(data) => data,
            Err(e) => {
                tray::update_tray_icon(&app, false);
                let _ = app.emit("error", format!("{}", e));
                return Err(format!("{}", e));
            }
        };

        // Resolve whisper context
        let ctx = match ctx {
            Some(c) => c,
            None => {
                let model_id = {
                    let inner = inner_arc.lock().unwrap();
                    inner.config.active_model.clone()
                };
                let model_id = match model_id {
                    Some(id) => id,
                    None => {
                        tray::update_tray_icon(&app, false);
                        let msg = "Aucun modele selectionne";
                        let _ = app.emit("error", msg);
                        return Err(msg.into());
                    }
                };
                let model_path = match models::get_model_path(&app_data_dir, &model_id) {
                    Some(p) => p,
                    None => {
                        tray::update_tray_icon(&app, false);
                        let msg = "Modele non installe";
                        let _ = app.emit("error", msg);
                        return Err(msg.into());
                    }
                };
                match transcription::load_model(&model_path) {
                    Ok(c) => {
                        let mut inner = inner_arc.lock().unwrap();
                        inner.whisper_ctx = Some(c.clone());
                        c
                    }
                    Err(e) => {
                        tray::update_tray_icon(&app, false);
                        let msg = format!("Erreur chargement modele : {}", e);
                        let _ = app.emit("error", &msg);
                        return Err(msg);
                    }
                }
            }
        };

        // Transcribe
        match transcription::transcribe_long(&ctx, &audio_data, &language, verbatim_mode) {
            Ok(text) => {
                if text.is_empty() {
                    tray::update_tray_icon(&app, false);
                    let _ = app.emit("transcription-complete", "");
                    return Ok(String::new());
                }

                match clipboard::copy_and_paste(&app, &text, auto_paste) {
                    Ok(()) => {
                        std::thread::spawn(|| sounds::play_complete_sound());
                        let _ = app.emit("transcription-complete", &text);
                    }
                    Err(e) => {
                        let _ = app.emit("error", format!("Erreur presse-papier : {}", e));
                    }
                }

                tray::update_tray_icon(&app, false);
                Ok(text)
            }
            Err(e) => {
                tray::update_tray_icon(&app, false);
                let msg = format!("Erreur de transcription : {}", e);
                let _ = app.emit("error", &msg);
                Err(msg)
            }
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

// ── TTS Commands ──

#[tauri::command]
pub fn set_tts_enabled(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.tts_enabled = enabled;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_tts_model(state: State<AppState>, model_id: String) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.tts_model = Some(model_id);
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_tts_rate(state: State<AppState>, rate: u32) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    inner.config.tts_rate = rate;
    let dir = inner.app_data_dir.clone();
    inner.config.save(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_tts_hotkey(
    app: AppHandle,
    state: State<AppState>,
    new_hotkey: String,
) -> Result<(), String> {
    {
        let mut inner = state.inner.lock().unwrap();
        inner.config.tts_hotkey = new_hotkey;
        let dir = inner.app_data_dir.clone();
        inner.config.save(&dir).map_err(|e| e.to_string())?;
    }

    hotkey::unregister_all(&app).map_err(|e| e.to_string())?;
    hotkey::register_all(&app).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn speak_text(app: AppHandle, state: State<AppState>, text: String) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();

    // Stop any ongoing TTS first
    if let Some(ref mut child) = inner.tts_process {
        tts::stop(child);
    }

    let voice_id = inner.config.tts_model.clone().ok_or("Aucune voix sélectionnée")?;
    let rate = inner.config.tts_rate;
    let data_dir = inner.app_data_dir.clone();
    drop(inner);

    let child = tts::speak(&data_dir, &text, &voice_id, rate).map_err(|e| e.to_string())?;
    let mut inner = state.inner.lock().unwrap();
    inner.tts_process = Some(child);

    app.emit("tts-state-changed", true).ok();
    let app_clone = app.clone();
    let state_clone = state.inner.clone();
    std::thread::spawn(move || {
        // Wait for the process to finish
        loop {
            std::thread::sleep(std::time::Duration::from_millis(200));
            let mut inner = state_clone.lock().unwrap();
            if let Some(ref mut child) = inner.tts_process {
                if !tts::is_speaking(child) {
                    inner.tts_process = None;
                    app_clone.emit("tts-state-changed", false).ok();
                    break;
                }
            } else {
                break;
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_speaking(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    if let Some(ref mut child) = inner.tts_process {
        tts::stop(child);
        inner.tts_process = None;
        app.emit("tts-state-changed", false).ok();
    }
    Ok(())
}

#[tauri::command]
pub fn list_tts_models(state: State<AppState>) -> Vec<TtsModelInfo> {
    let inner = state.inner.lock().unwrap();
    tts_models::list_tts_models(&inner.app_data_dir)
}

#[tauri::command]
pub async fn download_tts_voice(
    app: AppHandle,
    state: State<'_, AppState>,
    voice_id: String,
) -> Result<(), String> {
    let dir = {
        let inner = state.inner.lock().unwrap();
        inner.app_data_dir.clone()
    };
    tts_models::download_voice(app, dir, voice_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_piper(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let dir = {
        let inner = state.inner.lock().unwrap();
        inner.app_data_dir.clone()
    };
    tts_models::download_piper(app, dir)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_piper_installed(state: State<AppState>) -> bool {
    let inner = state.inner.lock().unwrap();
    tts_models::is_piper_installed(&inner.app_data_dir)
}

#[tauri::command]
pub fn delete_tts_voice(state: State<AppState>, voice_id: String) -> Result<(), String> {
    let inner = state.inner.lock().unwrap();
    tts_models::delete_voice(&inner.app_data_dir, &voice_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_speaking(state: State<AppState>) -> bool {
    let mut inner = state.inner.lock().unwrap();
    if let Some(ref mut child) = inner.tts_process {
        tts::is_speaking(child)
    } else {
        false
    }
}
