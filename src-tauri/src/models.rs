use futures_util::StreamExt;
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

use crate::errors::{AppError, AppResult};

const HF_BASE: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";

#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size_bytes: u64,
    pub size_label: String,
    pub url: String,
    pub is_english_only: bool,
    pub is_quantized: bool,
    pub is_downloaded: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub model_id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: f64,
}

struct ModelDef {
    id: &'static str,
    name: &'static str,
    size_bytes: u64,
    size_label: &'static str,
    english_only: bool,
    quantized: bool,
}

const CATALOG: &[ModelDef] = &[
    ModelDef { id: "ggml-tiny",          name: "Tiny",             size_bytes: 77_700_000,    size_label: "75 Mo",   english_only: false, quantized: false },
    ModelDef { id: "ggml-tiny.en",       name: "Tiny (English)",   size_bytes: 77_700_000,    size_label: "75 Mo",   english_only: true,  quantized: false },
    ModelDef { id: "ggml-tiny-q5_1",     name: "Tiny Q5",          size_bytes: 44_000_000,    size_label: "42 Mo",   english_only: false, quantized: true  },
    ModelDef { id: "ggml-base",          name: "Base",             size_bytes: 147_000_000,   size_label: "142 Mo",  english_only: false, quantized: false },
    ModelDef { id: "ggml-base.en",       name: "Base (English)",   size_bytes: 147_000_000,   size_label: "142 Mo",  english_only: true,  quantized: false },
    ModelDef { id: "ggml-base-q5_1",     name: "Base Q5",          size_bytes: 90_000_000,    size_label: "87 Mo",   english_only: false, quantized: true  },
    ModelDef { id: "ggml-small",         name: "Small",            size_bytes: 488_000_000,   size_label: "466 Mo",  english_only: false, quantized: false },
    ModelDef { id: "ggml-small.en",      name: "Small (English)",  size_bytes: 488_000_000,   size_label: "466 Mo",  english_only: true,  quantized: false },
    ModelDef { id: "ggml-small-q5_1",    name: "Small Q5",         size_bytes: 190_000_000,   size_label: "181 Mo",  english_only: false, quantized: true  },
    ModelDef { id: "ggml-medium",        name: "Medium",           size_bytes: 1_533_000_000, size_label: "1.4 Go",  english_only: false, quantized: false },
    ModelDef { id: "ggml-medium.en",     name: "Medium (English)", size_bytes: 1_533_000_000, size_label: "1.4 Go",  english_only: true,  quantized: false },
    ModelDef { id: "ggml-medium-q5_0",   name: "Medium Q5",        size_bytes: 540_000_000,   size_label: "515 Mo",  english_only: false, quantized: true  },
    ModelDef { id: "ggml-large-v3",      name: "Large v3",         size_bytes: 3_094_000_000, size_label: "2.9 Go",  english_only: false, quantized: false },
    ModelDef { id: "ggml-large-v3-q5_0", name: "Large v3 Q5",      size_bytes: 1_100_000_000, size_label: "1.0 Go",  english_only: false, quantized: true  },
];

fn models_dir(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("models")
}

fn model_file(app_data_dir: &PathBuf, model_id: &str) -> PathBuf {
    models_dir(app_data_dir).join(format!("{}.bin", model_id))
}

pub fn get_model_path(app_data_dir: &PathBuf, model_id: &str) -> Option<PathBuf> {
    let path = model_file(app_data_dir, model_id);
    path.exists().then_some(path)
}

pub fn list_models(app_data_dir: &PathBuf) -> Vec<ModelInfo> {
    CATALOG
        .iter()
        .map(|def| ModelInfo {
            id: def.id.to_string(),
            name: def.name.to_string(),
            size_bytes: def.size_bytes,
            size_label: def.size_label.to_string(),
            url: format!("{}/{}.bin", HF_BASE, def.id),
            is_english_only: def.english_only,
            is_quantized: def.quantized,
            is_downloaded: model_file(app_data_dir, def.id).exists(),
        })
        .collect()
}

pub async fn download_model(
    app: AppHandle,
    app_data_dir: PathBuf,
    model_id: String,
) -> AppResult<()> {
    let dir = models_dir(&app_data_dir);
    std::fs::create_dir_all(&dir)?;

    let url = format!("{}/{}.bin", HF_BASE, model_id);
    let part_path = dir.join(format!("{}.bin.part", model_id));
    let final_path = dir.join(format!("{}.bin", model_id));

    let response = reqwest::Client::new()
        .get(&url)
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
    let mut file = std::fs::File::create(&part_path)?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| AppError::Download(format!("Flux interrompu : {}", e)))?;
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
                model_id: model_id.clone(),
                downloaded_bytes: downloaded,
                total_bytes,
                percent,
            },
        );
    }

    drop(file);
    std::fs::rename(&part_path, &final_path)?;
    let _ = app.emit("download-complete", model_id);

    Ok(())
}

pub fn delete_model(app_data_dir: &PathBuf, model_id: &str) -> AppResult<()> {
    let path = model_file(app_data_dir, model_id);
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}
