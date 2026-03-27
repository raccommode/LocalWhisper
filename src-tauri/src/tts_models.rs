use futures_util::StreamExt;
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

use crate::errors::{AppError, AppResult};
use crate::models::DownloadProgress;

const HF_VOICES: &str = "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0";

const PIPER_RELEASE: &str = "https://github.com/rhasspy/piper/releases/download/2023.11.14-2";

struct VoiceDef {
    id: &'static str,
    name: &'static str,
    language: &'static str,
    size_label: &'static str,
    size_bytes: u64,
    /// Path components after HF_VOICES: lang/locale/name/quality
    hf_path: &'static str,
}

const VOICE_CATALOG: &[VoiceDef] = &[
    VoiceDef {
        id: "fr_FR-siwis-medium",
        name: "Siwis (Femme)",
        language: "Francais",
        size_label: "63 Mo",
        size_bytes: 63_000_000,
        hf_path: "fr/fr_FR/siwis/medium",
    },
    VoiceDef {
        id: "fr_FR-upmc-medium",
        name: "UPMC (Homme)",
        language: "Francais",
        size_label: "63 Mo",
        size_bytes: 63_000_000,
        hf_path: "fr/fr_FR/upmc/medium",
    },
    VoiceDef {
        id: "fr_FR-tom-medium",
        name: "Tom (Homme)",
        language: "Francais",
        size_label: "63 Mo",
        size_bytes: 63_000_000,
        hf_path: "fr/fr_FR/tom/medium",
    },
];

#[derive(Debug, Clone, Serialize)]
pub struct TtsModelInfo {
    pub id: String,
    pub name: String,
    pub language: String,
    pub size_label: String,
    pub size_bytes: u64,
    pub is_downloaded: bool,
}

fn tts_dir(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("tts")
}

fn voices_dir(app_data_dir: &PathBuf) -> PathBuf {
    tts_dir(app_data_dir).join("voices")
}

fn piper_dir(app_data_dir: &PathBuf) -> PathBuf {
    tts_dir(app_data_dir).join("piper")
}

/// Find the piper binary: check common local paths and PATH first, then downloaded copy.
pub fn piper_binary(app_data_dir: &PathBuf) -> PathBuf {
    // 1. Check common local paths
    let home = std::env::var("HOME").unwrap_or_default();
    let local_paths = [
        format!("{}/.local/bin/piper", home),
        "/usr/local/bin/piper".to_string(),
        "/opt/homebrew/bin/piper".to_string(),
    ];
    for p in &local_paths {
        let path = PathBuf::from(p);
        if path.exists() {
            return path;
        }
    }

    // 2. Check PATH via `which`
    if let Ok(output) = std::process::Command::new("which")
        .arg("piper")
        .output()
    {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return PathBuf::from(path_str);
            }
        }
    }

    // 3. Fallback to downloaded binary in app data dir
    piper_dir(app_data_dir).join("piper")
}

pub fn voice_model_path(app_data_dir: &PathBuf, voice_id: &str) -> PathBuf {
    voices_dir(app_data_dir).join(format!("{}.onnx", voice_id))
}

pub fn voice_config_path(app_data_dir: &PathBuf, voice_id: &str) -> PathBuf {
    voices_dir(app_data_dir).join(format!("{}.onnx.json", voice_id))
}

pub fn is_piper_installed(app_data_dir: &PathBuf) -> bool {
    let bin = piper_binary(app_data_dir);
    bin.exists()
}

pub fn is_voice_downloaded(app_data_dir: &PathBuf, voice_id: &str) -> bool {
    voice_model_path(app_data_dir, voice_id).exists()
        && voice_config_path(app_data_dir, voice_id).exists()
}

pub fn list_tts_models(app_data_dir: &PathBuf) -> Vec<TtsModelInfo> {
    VOICE_CATALOG
        .iter()
        .map(|def| TtsModelInfo {
            id: def.id.to_string(),
            name: def.name.to_string(),
            language: def.language.to_string(),
            size_label: def.size_label.to_string(),
            size_bytes: def.size_bytes,
            is_downloaded: is_voice_downloaded(app_data_dir, def.id),
        })
        .collect()
}

/// Download a Piper voice model (.onnx + .onnx.json) from Hugging Face.
pub async fn download_voice(
    app: AppHandle,
    app_data_dir: PathBuf,
    voice_id: String,
) -> AppResult<()> {
    let def = VOICE_CATALOG
        .iter()
        .find(|v| v.id == voice_id)
        .ok_or_else(|| AppError::Tts(format!("Voix inconnue : {}", voice_id)))?;

    let dir = voices_dir(&app_data_dir);
    std::fs::create_dir_all(&dir)?;

    // Download model .onnx
    let model_url = format!("{}/{}/{}.onnx", HF_VOICES, def.hf_path, voice_id);
    download_file(
        &app,
        &model_url,
        &dir.join(format!("{}.onnx.part", voice_id)),
        &dir.join(format!("{}.onnx", voice_id)),
        &voice_id,
    )
    .await?;

    // Download config .onnx.json
    let config_url = format!("{}/{}/{}.onnx.json", HF_VOICES, def.hf_path, voice_id);
    download_file(
        &app,
        &config_url,
        &dir.join(format!("{}.onnx.json.part", voice_id)),
        &dir.join(format!("{}.onnx.json", voice_id)),
        &voice_id,
    )
    .await?;

    let _ = app.emit("download-complete", &voice_id);
    Ok(())
}

/// Download the Piper binary from GitHub releases.
pub async fn download_piper(app: AppHandle, app_data_dir: PathBuf) -> AppResult<()> {
    let arch = if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x86_64"
    };

    let filename = format!("piper_macos_{}.tar.gz", arch);
    let url = format!("{}/{}", PIPER_RELEASE, filename);
    let dir = tts_dir(&app_data_dir);
    std::fs::create_dir_all(&dir)?;

    let tar_path = dir.join(&filename);

    // Download the tar.gz
    download_file(
        &app,
        &url,
        &dir.join(format!("{}.part", filename)),
        &tar_path,
        "piper",
    )
    .await?;

    // Extract with tar
    let status = std::process::Command::new("tar")
        .arg("-xzf")
        .arg(&tar_path)
        .current_dir(&dir)
        .status()
        .map_err(|e| AppError::Tts(format!("Extraction impossible : {}", e)))?;

    if !status.success() {
        return Err(AppError::Tts("Extraction du tar.gz échouée".to_string()));
    }

    // Make piper binary executable
    let piper_bin = piper_binary(&app_data_dir);
    if piper_bin.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&piper_bin)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&piper_bin, perms)?;
        }
    }

    // Clean up tar.gz
    let _ = std::fs::remove_file(&tar_path);

    let _ = app.emit("download-complete", "piper");
    Ok(())
}

pub fn delete_voice(app_data_dir: &PathBuf, voice_id: &str) -> AppResult<()> {
    let model = voice_model_path(app_data_dir, voice_id);
    let config = voice_config_path(app_data_dir, voice_id);
    if model.exists() {
        std::fs::remove_file(&model)?;
    }
    if config.exists() {
        std::fs::remove_file(&config)?;
    }
    Ok(())
}

async fn download_file(
    app: &AppHandle,
    url: &str,
    part_path: &PathBuf,
    final_path: &PathBuf,
    event_id: &str,
) -> AppResult<()> {
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .map_err(|e| AppError::Download(format!("Requête impossible : {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::Download(format!(
            "Téléchargement échoué (HTTP {})",
            response.status()
        )));
    }

    let total_bytes = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut file = std::fs::File::create(part_path)?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk =
            chunk.map_err(|e| AppError::Download(format!("Flux interrompu : {}", e)))?;
        std::io::Write::write_all(&mut file, &chunk)?;
        downloaded += chunk.len() as u64;

        let percent = if total_bytes > 0 {
            (downloaded as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let _ = app.emit(
            "download-progress",
            DownloadProgress {
                model_id: event_id.to_string(),
                downloaded_bytes: downloaded,
                total_bytes,
                percent,
            },
        );
    }

    drop(file);
    std::fs::rename(part_path, final_path)?;
    Ok(())
}
