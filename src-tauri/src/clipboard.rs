use tauri::{AppHandle, Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::errors::{AppError, AppResult};

pub fn copy_and_paste(app: &AppHandle, text: &str, auto_paste: bool) -> AppResult<()> {
    app.clipboard()
        .write_text(text.to_string())
        .map_err(|e| AppError::Clipboard(format!("Copie impossible : {}", e)))?;

    if auto_paste {
        // Small delay so the clipboard content is ready before the paste keystroke
        std::thread::sleep(std::time::Duration::from_millis(100));

        // enigo must run on the main thread on macOS (HIToolbox / TSM APIs require it).
        // Use run_on_main_thread to dispatch the paste simulation safely.
        let app_handle = app.clone();
        app.run_on_main_thread(move || {
            if let Err(e) = simulate_paste() {
                log::error!("Simulation collage echouee : {}", e);
                let _ = app_handle.emit("error", format!("Erreur collage : {}", e));
            }
        })
        .map_err(|e| AppError::Clipboard(format!("Dispatch main thread impossible : {}", e)))?;
    }

    Ok(())
}

fn simulate_paste() -> AppResult<()> {
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| AppError::Clipboard(format!("Initialisation Enigo impossible : {}", e)))?;

    #[cfg(target_os = "macos")]
    {
        enigo
            .key(Key::Meta, Press)
            .map_err(|e| AppError::Clipboard(format!("Touche Meta press échouée : {}", e)))?;
        enigo
            .key(Key::Unicode('v'), Click)
            .map_err(|e| AppError::Clipboard(format!("Touche V click échouée : {}", e)))?;
        enigo
            .key(Key::Meta, Release)
            .map_err(|e| AppError::Clipboard(format!("Touche Meta release échouée : {}", e)))?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        enigo
            .key(Key::Control, Press)
            .map_err(|e| AppError::Clipboard(format!("Touche Ctrl press échouée : {}", e)))?;
        enigo
            .key(Key::Unicode('v'), Click)
            .map_err(|e| AppError::Clipboard(format!("Touche V click échouée : {}", e)))?;
        enigo
            .key(Key::Control, Release)
            .map_err(|e| AppError::Clipboard(format!("Touche Ctrl release échouée : {}", e)))?;
    }

    Ok(())
}
