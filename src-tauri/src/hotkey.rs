use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState, Shortcut};

use crate::audio;
use crate::clipboard;
use crate::errors::{AppError, AppResult};
use crate::models;
use crate::sounds;
use crate::state::{AppState, RecordingStream};
use crate::transcription;
use crate::tray;

/// On Windows, replace "Super" modifier with "Ctrl" since the Win key
/// is intercepted by the OS for most key combinations.
fn normalize_hotkey(hotkey_str: &str) -> String {
    if cfg!(target_os = "windows") && hotkey_str.contains("Super") {
        hotkey_str.replace("Super", "Ctrl")
    } else {
        hotkey_str.to_string()
    }
}

/// Register the toggle hotkey (press to start, press again to stop)
pub fn register_hotkey(app: &AppHandle, hotkey_str: &str) -> AppResult<()> {
    if hotkey_str.is_empty() {
        return Ok(());
    }

    let normalized = normalize_hotkey(hotkey_str);
    let shortcut: Shortcut = normalized
        .parse()
        .map_err(|e| AppError::Hotkey(format!("Raccourci invalide '{}' : {}", normalized, e)))?;

    let handle = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_recording(&handle);
            }
        })
        .map_err(|e| AppError::Hotkey(format!("Enregistrement raccourci impossible : {}", e)))?;

    Ok(())
}

/// Register the push-to-talk hotkey (hold to record, release to stop)
pub fn register_ptt_hotkey(app: &AppHandle, hotkey_str: &str) -> AppResult<()> {
    if hotkey_str.is_empty() {
        return Ok(());
    }

    let normalized = normalize_hotkey(hotkey_str);
    let shortcut: Shortcut = normalized
        .parse()
        .map_err(|e| AppError::Hotkey(format!("Raccourci PTT invalide '{}' : {}", normalized, e)))?;

    let handle = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            match event.state {
                ShortcutState::Pressed => handle_press(&handle),
                ShortcutState::Released => handle_release(&handle),
            }
        })
        .map_err(|e| AppError::Hotkey(format!("Enregistrement raccourci PTT impossible : {}", e)))?;

    Ok(())
}

/// Register both hotkeys from the current config
pub fn register_all(app: &AppHandle) -> AppResult<()> {
    let state: tauri::State<AppState> = app.state();
    let (hotkey, hotkey_ptt) = {
        let inner = state.inner.lock().unwrap();
        (inner.config.hotkey.clone(), inner.config.hotkey_ptt.clone())
    };

    register_hotkey(app, &hotkey)?;
    register_ptt_hotkey(app, &hotkey_ptt)?;

    Ok(())
}

pub fn unregister_all(app: &AppHandle) -> AppResult<()> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| AppError::Hotkey(format!("Desenregistrement impossible : {}", e)))?;
    Ok(())
}

fn handle_press(app: &AppHandle) {
    let state: tauri::State<AppState> = app.state();
    let rec: tauri::State<RecordingStream> = app.state();
    let currently_recording = {
        let inner = state.inner.lock().unwrap();
        inner.is_recording
    };
    if !currently_recording {
        start(app, &state, &rec);
    }
}

fn handle_release(app: &AppHandle) {
    let state: tauri::State<AppState> = app.state();
    let rec: tauri::State<RecordingStream> = app.state();
    let currently_recording = {
        let inner = state.inner.lock().unwrap();
        inner.is_recording
    };
    if currently_recording {
        stop(app, &state, &rec);
    }
}

fn toggle_recording(app: &AppHandle) {
    let state: tauri::State<AppState> = app.state();
    let rec: tauri::State<RecordingStream> = app.state();

    let currently_recording = {
        let inner = state.inner.lock().unwrap();
        inner.is_recording
    };

    if !currently_recording {
        start(app, &state, &rec);
    } else {
        stop(app, &state, &rec);
    }
}

fn start(app: &AppHandle, state: &AppState, rec: &RecordingStream) {
    let device = {
        let inner = state.inner.lock().unwrap();
        inner.config.audio_device.clone()
    };

    match audio::start_recording(state, rec, device) {
        Ok(()) => {
            // Play start sound in a background thread to not block
            std::thread::spawn(|| sounds::play_start_sound());

            tray::update_tray_icon(app, true);
            let _ = app.emit("recording-state-changed", true);
            log::info!("Enregistrement demarre");
        }
        Err(e) => {
            log::error!("Demarrage enregistrement impossible : {}", e);
            let _ = app.emit("error", format!("Erreur d'enregistrement : {}", e));
        }
    }
}

fn stop(app: &AppHandle, state: &AppState, rec: &RecordingStream) {
    match audio::stop_recording(state, rec) {
        Ok(audio_data) => {
            // Play stop sound in a background thread
            std::thread::spawn(|| sounds::play_stop_sound());

            tray::update_tray_icon(app, false);
            let _ = app.emit("recording-state-changed", false);
            log::info!("Enregistrement arrete. {} echantillons.", audio_data.len());

            if audio_data.is_empty() {
                log::warn!("Aucune donnee audio capturee");
                return;
            }

            let handle = app.clone();
            let inner_arc = state.inner.clone();

            std::thread::spawn(move || {
                run_transcription(handle, inner_arc, audio_data);
            });
        }
        Err(e) => {
            tray::update_tray_icon(app, false);
            let _ = app.emit("recording-state-changed", false);
            log::error!("Arret enregistrement impossible : {}", e);
            let _ = app.emit("error", format!("Erreur d'arret : {}", e));
        }
    }
}

fn run_transcription(
    app: AppHandle,
    inner_arc: std::sync::Arc<std::sync::Mutex<crate::state::InnerState>>,
    audio_data: Vec<f32>,
) {
    let _ = app.emit("transcription-started", ());
    tray::start_processing_animation(&app);

    let (ctx, language, auto_paste, app_data_dir) = {
        let inner = inner_arc.lock().unwrap();
        (
            inner.whisper_ctx.clone(),
            inner.config.language.clone(),
            inner.config.auto_paste,
            inner.app_data_dir.clone(),
        )
    };

    // Resolve the Whisper context: use cached or lazy-load from disk
    let ctx = match ctx {
        Some(c) => c,
        None => match resolve_model(&app, &inner_arc, &app_data_dir) {
            Some(c) => c,
            None => {
                tray::update_tray_icon(&app, false);
                return;
            }
        },
    };

    match transcription::transcribe(&ctx, &audio_data, &language) {
        Ok(text) => {
            if text.is_empty() {
                tray::update_tray_icon(&app, false);
                let _ = app.emit("transcription-complete", "");
                return;
            }

            log::info!("Transcription : {}", text);

            match clipboard::copy_and_paste(&app, &text, auto_paste) {
                Ok(()) => {
                    std::thread::spawn(|| sounds::play_complete_sound());
                    let _ = app.emit("transcription-complete", &text);
                }
                Err(e) => {
                    log::error!("Erreur presse-papier : {}", e);
                    let _ = app.emit("error", format!("Erreur presse-papier : {}", e));
                }
            }
        }
        Err(e) => {
            log::error!("Erreur de transcription : {}", e);
            let _ = app.emit("error", format!("Erreur de transcription : {}", e));
        }
    }

    tray::update_tray_icon(&app, false);
}

fn resolve_model(
    app: &AppHandle,
    inner_arc: &std::sync::Arc<std::sync::Mutex<crate::state::InnerState>>,
    app_data_dir: &std::path::PathBuf,
) -> Option<std::sync::Arc<whisper_rs::WhisperContext>> {
    let model_id = {
        let inner = inner_arc.lock().unwrap();
        inner.config.active_model.clone()
    };

    let model_id = match model_id {
        Some(id) => id,
        None => {
            let _ = app.emit(
                "error",
                "Aucun modele selectionne. Veuillez configurer un modele dans les parametres.",
            );
            return None;
        }
    };

    let path = match models::get_model_path(app_data_dir, &model_id) {
        Some(p) => p,
        None => {
            let _ = app.emit(
                "error",
                "Aucun modele installe. Veuillez telecharger un modele dans les parametres.",
            );
            return None;
        }
    };

    match transcription::load_model(&path) {
        Ok(ctx) => {
            let mut inner = inner_arc.lock().unwrap();
            inner.whisper_ctx = Some(ctx.clone());
            Some(ctx)
        }
        Err(e) => {
            log::error!("Chargement modele impossible : {}", e);
            let _ = app.emit("error", format!("Erreur de chargement du modele : {}", e));
            None
        }
    }
}
