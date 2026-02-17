use std::sync::atomic::{AtomicBool, Ordering};

use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

use crate::errors::{AppError, AppResult};

static PROCESSING_ACTIVE: AtomicBool = AtomicBool::new(false);

const PROCESSING_FRAMES: [&[u8]; 8] = [
    include_bytes!("../icons/tray-processing-0.png"),
    include_bytes!("../icons/tray-processing-1.png"),
    include_bytes!("../icons/tray-processing-2.png"),
    include_bytes!("../icons/tray-processing-3.png"),
    include_bytes!("../icons/tray-processing-4.png"),
    include_bytes!("../icons/tray-processing-5.png"),
    include_bytes!("../icons/tray-processing-6.png"),
    include_bytes!("../icons/tray-processing-7.png"),
];

/// Toggle macOS dock icon visibility.
/// Uses NSApplication.setActivationPolicy:
///   0 = Regular (dock icon visible)
///   1 = Accessory (no dock icon, menu bar only)
#[cfg(target_os = "macos")]
pub fn set_dock_visible(visible: bool) {
    use objc::{class, msg_send, sel, sel_impl};
    unsafe {
        let app: *mut objc::runtime::Object = msg_send![class!(NSApplication), sharedApplication];
        let policy: isize = if visible { 0 } else { 1 };
        let _: () = msg_send![app, setActivationPolicy: policy];
    }
}

pub fn setup_tray(app: &AppHandle) -> AppResult<()> {
    let settings_item = MenuItem::with_id(app, "settings", "Parametres", true, None::<&str>)
        .map_err(|e| AppError::Config(format!("Creation menu impossible : {}", e)))?;

    let quit_item = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)
        .map_err(|e| AppError::Config(format!("Creation menu impossible : {}", e)))?;

    let menu = Menu::with_items(app, &[&settings_item, &quit_item])
        .map_err(|e| AppError::Config(format!("Creation menu impossible : {}", e)))?;

    let icon = Image::from_bytes(include_bytes!("../icons/tray-idle.png"))
        .map_err(|e| AppError::Config(format!("Icone tray impossible : {}", e)))?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .tooltip("LocalWhisper")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                if let Some(win) = app.get_webview_window("main") {
                    // Show dock icon when window is open
                    #[cfg(target_os = "macos")]
                    set_dock_visible(true);

                    let _ = win.show();
                    let _ = win.set_focus();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)
        .map_err(|e| AppError::Config(format!("Construction tray impossible : {}", e)))?;

    Ok(())
}

pub fn update_tray_icon(app: &AppHandle, recording: bool) {
    // Stop any running processing animation
    stop_processing_animation();

    let Some(tray) = app.tray_by_id("main") else {
        return;
    };

    let bytes: &[u8] = if recording {
        include_bytes!("../icons/tray-recording.png")
    } else {
        include_bytes!("../icons/tray-idle.png")
    };

    if let Ok(icon) = Image::from_bytes(bytes) {
        let _ = tray.set_icon(Some(icon));
    }

    let tooltip = if recording {
        "LocalWhisper - Enregistrement..."
    } else {
        "LocalWhisper"
    };
    let _ = tray.set_tooltip(Some(tooltip));
}

/// Start the animated processing spinner in the tray icon.
/// Spawns a background thread that cycles through frames until stopped.
pub fn start_processing_animation(app: &AppHandle) {
    // Mark animation as active
    PROCESSING_ACTIVE.store(true, Ordering::SeqCst);

    // Set tooltip immediately
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some("LocalWhisper - Transcription..."));
    }

    let handle = app.clone();
    std::thread::spawn(move || {
        let mut frame_idx = 0usize;
        while PROCESSING_ACTIVE.load(Ordering::SeqCst) {
            if let Some(tray) = handle.tray_by_id("main") {
                if let Ok(icon) = Image::from_bytes(PROCESSING_FRAMES[frame_idx]) {
                    let _ = tray.set_icon(Some(icon));
                }
            }
            frame_idx = (frame_idx + 1) % PROCESSING_FRAMES.len();
            std::thread::sleep(std::time::Duration::from_millis(120));
        }
    });
}

/// Stop the processing animation and reset to the idle icon.
pub fn stop_processing_animation() {
    PROCESSING_ACTIVE.store(false, Ordering::SeqCst);
}
