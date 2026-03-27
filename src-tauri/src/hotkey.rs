use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
use crate::tts;

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

/// Register the TTS hotkey (press to read selection, press again to stop)
pub fn register_tts_hotkey(app: &AppHandle, hotkey_str: &str) -> AppResult<()> {
    if hotkey_str.is_empty() {
        return Ok(());
    }

    let normalized = normalize_hotkey(hotkey_str);
    let shortcut: Shortcut = normalized
        .parse()
        .map_err(|e| AppError::Hotkey(format!("Raccourci TTS invalide '{}' : {}", normalized, e)))?;

    let handle = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_tts(&handle);
            }
        })
        .map_err(|e| AppError::Hotkey(format!("Enregistrement raccourci TTS impossible : {}", e)))?;

    Ok(())
}

/// Register all hotkeys from the current config
pub fn register_all(app: &AppHandle) -> AppResult<()> {
    let state: tauri::State<AppState> = app.state();
    let (hotkey, hotkey_ptt, tts_hotkey, tts_enabled) = {
        let inner = state.inner.lock().unwrap();
        (
            inner.config.hotkey.clone(),
            inner.config.hotkey_ptt.clone(),
            inner.config.tts_hotkey.clone(),
            inner.config.tts_enabled,
        )
    };

    register_hotkey(app, &hotkey)?;
    register_ptt_hotkey(app, &hotkey_ptt)?;
    if tts_enabled {
        register_tts_hotkey(app, &tts_hotkey)?;
    }

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
    let (device, live_mode) = {
        let inner = state.inner.lock().unwrap();
        (inner.config.audio_device.clone(), inner.config.live_mode)
    };

    match audio::start_recording(state, rec, device) {
        Ok(()) => {
            // Play start sound in a background thread to not block
            std::thread::spawn(|| sounds::play_start_sound());

            tray::update_tray_icon(app, true);
            let _ = app.emit("recording-state-changed", true);
            log::info!("Enregistrement demarre");

            if live_mode {
                let shared_buffer = {
                    let inner = state.inner.lock().unwrap();
                    inner.shared_buffer.clone()
                };
                if let Some(shared_buffer) = shared_buffer {
                    let stop_signal = Arc::new(AtomicBool::new(false));
                    {
                        let mut inner = state.inner.lock().unwrap();
                        inner.live_stop_signal = Some(stop_signal.clone());
                    }
                    let handle = app.clone();
                    let inner_arc = state.inner.clone();
                    std::thread::spawn(move || {
                        run_live_transcription(handle, inner_arc, shared_buffer, stop_signal);
                    });
                }
            }
        }
        Err(e) => {
            log::error!("Demarrage enregistrement impossible : {}", e);
            let _ = app.emit("error", format!("Erreur d'enregistrement : {}", e));
        }
    }
}

fn stop(app: &AppHandle, state: &AppState, rec: &RecordingStream) {
    let live_mode = {
        let inner = state.inner.lock().unwrap();
        inner.config.live_mode
    };

    match audio::stop_recording(state, rec) {
        Ok(audio_data) => {
            // Play stop sound in a background thread
            std::thread::spawn(|| sounds::play_stop_sound());

            let _ = app.emit("recording-state-changed", false);
            log::info!("Enregistrement arrete. {} echantillons.", audio_data.len());

            if live_mode {
                // Signal the live transcription thread to process final chunk and finish.
                // The live thread handles tray icon reset and transcription-complete.
                let mut inner = state.inner.lock().unwrap();
                if let Some(signal) = &inner.live_stop_signal {
                    signal.store(true, Ordering::Relaxed);
                }
                inner.live_stop_signal = None;
            } else {
                tray::update_tray_icon(app, false);

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

    let (ctx, language, auto_paste, verbatim_mode, app_data_dir) = {
        let inner = inner_arc.lock().unwrap();
        (
            inner.whisper_ctx.clone(),
            inner.config.language.clone(),
            inner.config.auto_paste,
            inner.config.verbatim_mode,
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

    match transcription::transcribe_long(&ctx, &audio_data, &language, verbatim_mode) {
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

fn run_live_transcription(
    app: AppHandle,
    inner_arc: std::sync::Arc<std::sync::Mutex<crate::state::InnerState>>,
    shared_buffer: Arc<std::sync::Mutex<Vec<f32>>>,
    stop_signal: Arc<AtomicBool>,
) {
    let (ctx, language, verbatim_mode, sample_rate, app_data_dir) = {
        let inner = inner_arc.lock().unwrap();
        (
            inner.whisper_ctx.clone(),
            inner.config.language.clone(),
            inner.config.verbatim_mode,
            inner.sample_rate,
            inner.app_data_dir.clone(),
        )
    };

    // Resolve the Whisper context
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

    let mut last_processed: usize = 0;
    let mut accumulated_text = String::new();

    // Live loop: transcribe new audio every ~5 seconds
    loop {
        // Wait ~5 seconds, checking stop signal every 100ms
        let mut should_stop = false;
        for _ in 0..50 {
            if stop_signal.load(Ordering::Relaxed) {
                should_stop = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Read new audio from the shared buffer
        let raw_chunk = {
            let buf = shared_buffer.lock().unwrap();
            if buf.len() > last_processed {
                let chunk = buf[last_processed..].to_vec();
                chunk
            } else {
                Vec::new()
            }
        };

        // Only transcribe if we have at least ~2 seconds of new audio
        let min_samples = (sample_rate as usize) * 2;
        if raw_chunk.len() >= min_samples {
            last_processed += raw_chunk.len();

            // Resample to 16kHz if needed
            let chunk = if sample_rate != 16000 {
                audio::resample(&raw_chunk, sample_rate, 16000)
            } else {
                raw_chunk
            };

            match transcription::transcribe(&ctx, &chunk, &language, verbatim_mode) {
                Ok(text) => {
                    if !text.is_empty() {
                        if !accumulated_text.is_empty() {
                            accumulated_text.push(' ');
                        }
                        accumulated_text.push_str(&text);
                        let _ = app.emit("live-transcription-partial", &accumulated_text);
                        log::info!("Live partiel : {}", text);
                    }
                }
                Err(e) => {
                    log::error!("Erreur transcription live : {}", e);
                }
            }
        }

        if should_stop {
            break;
        }
    }

    // Process any remaining audio after stop
    let raw_remaining = {
        let buf = shared_buffer.lock().unwrap();
        if buf.len() > last_processed {
            buf[last_processed..].to_vec()
        } else {
            Vec::new()
        }
    };

    // Only transcribe remaining if it's at least ~0.5 seconds
    let min_remaining = (sample_rate as usize) / 2;
    if raw_remaining.len() >= min_remaining {
        let chunk = if sample_rate != 16000 {
            audio::resample(&raw_remaining, sample_rate, 16000)
        } else {
            raw_remaining
        };

        if let Ok(text) = transcription::transcribe(&ctx, &chunk, &language, verbatim_mode) {
            if !text.is_empty() {
                if !accumulated_text.is_empty() {
                    accumulated_text.push(' ');
                }
                accumulated_text.push_str(&text);
            }
        }
    }

    // Final result: copy to clipboard and emit completion
    if !accumulated_text.is_empty() {
        let auto_paste = {
            let inner = inner_arc.lock().unwrap();
            inner.config.auto_paste
        };

        log::info!("Live transcription complete : {}", accumulated_text);

        match clipboard::copy_and_paste(&app, &accumulated_text, auto_paste) {
            Ok(()) => {
                std::thread::spawn(|| sounds::play_complete_sound());
                let _ = app.emit("transcription-complete", &accumulated_text);
            }
            Err(e) => {
                log::error!("Erreur presse-papier : {}", e);
                let _ = app.emit("error", format!("Erreur presse-papier : {}", e));
            }
        }
    } else {
        let _ = app.emit("transcription-complete", "");
    }

    tray::update_tray_icon(&app, false);
}

/// Fallback for apps that don't expose AX: simulate Cmd+C via CGEvent,
/// then poll for a clipboard change. Adds a 100ms delay before simulating
/// to let the hotkey modifier keys be physically released first.
fn copy_selection_and_read_clipboard(app: &AppHandle) -> String {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    let old = app.clipboard().read_text().unwrap_or_default();

    // Wait for hotkey keys to be released before injecting Cmd+C
    std::thread::sleep(std::time::Duration::from_millis(100));

    #[cfg(target_os = "macos")]
    {
        use std::os::raw::c_void;
        use std::ptr;

        type CGEventRef = *mut c_void;

        #[link(name = "CoreGraphics", kind = "framework")]
        extern "C" {
            fn CGEventCreateKeyboardEvent(src: *const c_void, key: u16, down: bool) -> CGEventRef;
            fn CGEventSetFlags(event: CGEventRef, flags: u64);
            fn CGEventPost(tap: i32, event: CGEventRef);
            fn CFRelease(cf: *const c_void);
        }

        const KCG_SESSION_EVENT_TAP: i32 = 1;
        const KCG_FLAG_COMMAND: u64 = 0x0010_0000;
        const KVK_ANSI_C: u16 = 0x08;

        unsafe {
            let down = CGEventCreateKeyboardEvent(ptr::null(), KVK_ANSI_C, true);
            if !down.is_null() {
                CGEventSetFlags(down, KCG_FLAG_COMMAND);
                CGEventPost(KCG_SESSION_EVENT_TAP, down);
                CFRelease(down as *const c_void);
            }
            let up = CGEventCreateKeyboardEvent(ptr::null(), KVK_ANSI_C, false);
            if !up.is_null() {
                CGEventSetFlags(up, KCG_FLAG_COMMAND);
                CGEventPost(KCG_SESSION_EVENT_TAP, up);
                CFRelease(up as *const c_void);
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        if let Ok(mut enigo) = enigo::Enigo::new(&enigo::Settings::default()) {
            use enigo::{Direction, Key, Keyboard};
            let _ = enigo.key(Key::Control, Direction::Press);
            let _ = enigo.key(Key::Unicode('c'), Direction::Click);
            let _ = enigo.key(Key::Control, Direction::Release);
        }
    }

    // Poll for clipboard change (up to 600ms, every 60ms)
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(60));
        let current = app.clipboard().read_text().unwrap_or_default();
        if !current.is_empty() && current != old {
            return current;
        }
    }

    String::new()
}

/// Get the currently selected text from the frontmost app using macOS
/// Accessibility API via osascript. No clipboard simulation needed.
fn get_selected_text_ax() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let script = r#"tell application "System Events"
    try
        set frontProcess to first application process whose frontmost is true
        set focusedEl to value of attribute "AXFocusedUIElement" of frontProcess
        set sel to value of attribute "AXSelectedText" of focusedEl
        return sel
    on error
        return ""
    end try
end tell"#;

        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .ok()?;

        let text = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();

        if text.is_empty() { None } else { Some(text) }
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

fn toggle_tts(app: &AppHandle) {
    let state: tauri::State<AppState> = app.state();
    let mut inner = state.inner.lock().unwrap();

    // If currently speaking, stop
    if let Some(ref mut child) = inner.tts_process {
        if tts::is_speaking(child) {
            tts::stop(child);
            inner.tts_process = None;
            let _ = app.emit("tts-state-changed", false);
            tray::update_tray_tooltip(app, None);
            return;
        }
    }

    let voice_id = match inner.config.tts_model.clone() {
        Some(v) => v,
        None => {
            let _ = app.emit("error", "Aucune voix TTS sélectionnée. Téléchargez une voix dans les réglages.");
            return;
        }
    };
    let rate = inner.config.tts_rate;
    let data_dir = inner.app_data_dir.clone();
    let inner_arc = state.inner.clone();
    drop(inner); // Release the lock before reading selection

    let app_clone = app.clone();
    std::thread::spawn(move || {
        // 1. Try AX API (fast, works for most native apps)
        let text = get_selected_text_ax().unwrap_or_default();

        // 2. Fallback: simulate Cmd+C and poll clipboard (works for Electron,
        //    terminals, browsers that don't expose AX selected text)
        let text = if text.is_empty() {
            copy_selection_and_read_clipboard(&app_clone)
        } else {
            text
        };

        if text.is_empty() {
            let _ = app_clone.emit("error", "Aucun texte sélectionné. Sélectionnez du texte avant d'appuyer sur le raccourci.");
            return;
        }

        // Start TTS
        match tts::speak(&data_dir, &text, &voice_id, rate) {
            Ok(child) => {
                let mut inner = inner_arc.lock().unwrap();
                inner.tts_process = Some(child);
                let _ = app_clone.emit("tts-state-changed", true);
                let _ = app_clone.emit("tts-text", &text);
                tray::update_tray_tooltip(&app_clone, Some("LocalWhisper - Lecture..."));
                drop(inner);

                // Monitor the process
                let app_mon = app_clone.clone();
                let inner_mon = inner_arc.clone();
                std::thread::spawn(move || {
                    loop {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        let mut inner = inner_mon.lock().unwrap();
                        if let Some(ref mut child) = inner.tts_process {
                            if !tts::is_speaking(child) {
                                inner.tts_process = None;
                                let _ = app_mon.emit("tts-state-changed", false);
                                tray::update_tray_tooltip(&app_mon, None);
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                });
            }
            Err(e) => {
                log::error!("TTS : {}", e);
                let _ = app_clone.emit("error", e.to_string());
            }
        }
    });
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
