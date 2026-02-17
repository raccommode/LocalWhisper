use std::path::Path;
use std::sync::Arc;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::errors::{AppError, AppResult};

pub fn load_model(path: &Path) -> AppResult<Arc<WhisperContext>> {
    let path_str = path
        .to_str()
        .ok_or_else(|| AppError::Transcription("Chemin du modèle invalide".into()))?;

    let ctx = WhisperContext::new_with_params(path_str, WhisperContextParameters::default())
        .map_err(|e| AppError::Transcription(format!("Chargement du modèle impossible : {}", e)))?;

    Ok(Arc::new(ctx))
}

pub fn transcribe(ctx: &WhisperContext, audio: &[f32], language: &str) -> AppResult<String> {
    if audio.is_empty() {
        return Ok(String::new());
    }

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    let lang = if language == "auto" { None } else { Some(language) };
    params.set_language(lang);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_no_timestamps(true);
    params.set_single_segment(false);

    let mut state = ctx
        .create_state()
        .map_err(|e| AppError::Transcription(format!("Création état impossible : {}", e)))?;

    state
        .full(params, audio)
        .map_err(|e| AppError::Transcription(format!("Transcription échouée : {}", e)))?;

    let n_segments = state
        .full_n_segments()
        .map_err(|e| AppError::Transcription(format!("Lecture segments impossible : {}", e)))?;

    let mut text = String::new();
    for i in 0..n_segments {
        let segment = state
            .full_get_segment_text(i)
            .map_err(|e| AppError::Transcription(format!("Lecture segment {} impossible : {}", i, e)))?;
        text.push_str(&segment);
    }

    Ok(text.trim().to_string())
}
