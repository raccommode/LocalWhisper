use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: String,
    #[serde(default)]
    pub hotkey_ptt: String,
    pub auto_paste: bool,
    pub active_model: Option<String>,
    pub language: String,
    pub audio_device: Option<String>,
    #[serde(default = "default_ui_locale")]
    pub ui_locale: String,
    #[serde(default = "default_true")]
    pub verbatim_mode: bool,
    #[serde(default)]
    pub live_mode: bool,
    pub first_run_complete: bool,
    #[serde(default = "default_true")]
    pub tts_enabled: bool,
    #[serde(default)]
    pub tts_model: Option<String>,
    #[serde(default = "default_tts_rate")]
    pub tts_rate: u32,
    #[serde(default = "default_tts_hotkey")]
    pub tts_hotkey: String,
}

fn default_ui_locale() -> String {
    "en".to_string()
}

fn default_true() -> bool {
    true
}

fn default_tts_rate() -> u32 {
    180
}

fn default_tts_hotkey() -> String {
    "CmdOrCtrl+Shift+S".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey: if cfg!(target_os = "macos") {
                "Super+Insert".to_string()
            } else {
                "Ctrl+Shift+Space".to_string()
            },
            hotkey_ptt: "Insert".to_string(),
            auto_paste: true,
            active_model: None,
            language: "fr".to_string(),
            audio_device: None,
            ui_locale: "en".to_string(),
            verbatim_mode: true,
            live_mode: false,
            first_run_complete: false,
            tts_enabled: true,
            tts_model: None,
            tts_rate: 180,
            tts_hotkey: "CmdOrCtrl+Shift+S".to_string(),
        }
    }
}

impl AppConfig {
    fn config_path(app_data_dir: &PathBuf) -> PathBuf {
        app_data_dir.join("config.json")
    }

    pub fn load(app_data_dir: &PathBuf) -> AppResult<Self> {
        let path = Self::config_path(app_data_dir);

        if !path.exists() {
            let config = Self::default();
            config.save(app_data_dir)?;
            return Ok(config);
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| AppError::Config(format!("Lecture impossible : {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| AppError::Config(format!("Parsing impossible : {}", e)))
    }

    pub fn save(&self, app_data_dir: &PathBuf) -> AppResult<()> {
        std::fs::create_dir_all(app_data_dir)?;
        let path = Self::config_path(app_data_dir);
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Sérialisation impossible : {}", e)))?;
        std::fs::write(&path, json)?;
        Ok(())
    }
}
