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

pub fn transcribe(
    ctx: &WhisperContext,
    audio: &[f32],
    language: &str,
    verbatim_mode: bool,
) -> AppResult<String> {
    if audio.is_empty() {
        return Ok(String::new());
    }

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    let lang = if language == "auto" { None } else { Some(language) };
    params.set_language(lang);
    params.set_translate(false);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_no_timestamps(true);
    params.set_single_segment(false);
    params.set_suppress_blank(true);
    params.set_suppress_non_speech_tokens(true);

    if verbatim_mode {
        // Verbatim mode: force deterministic decoding, no fallback, no context
        // temperature_inc=0 disables the fallback that causes summarization
        params.set_temperature(0.0);
        params.set_temperature_inc(0.0);
        params.set_entropy_thold(2.8);
        params.set_logprob_thold(-1.5);
        params.set_no_context(true);
        // No initial_prompt: Whisper treats it as prior text, not as an instruction,
        // which can confuse the model into summarizing instead of transcribing
    }

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

/// Transcribe long audio by splitting into chunks to prevent summarization.
/// Each chunk is transcribed independently, preventing Whisper from condensing
/// long recordings into summaries.
pub fn transcribe_long(
    ctx: &WhisperContext,
    audio: &[f32],
    language: &str,
    verbatim_mode: bool,
) -> AppResult<String> {
    // 20 seconds at 16kHz — stays well under Whisper's 30s window
    const CHUNK_SAMPLES: usize = 16000 * 20;

    if audio.len() <= CHUNK_SAMPLES {
        return transcribe(ctx, audio, language, verbatim_mode);
    }

    let mut full_text = String::new();
    let mut offset = 0;

    while offset < audio.len() {
        let end = (offset + CHUNK_SAMPLES).min(audio.len());
        let chunk = &audio[offset..end];

        let text = transcribe(ctx, chunk, language, verbatim_mode)?;
        if !text.is_empty() {
            if !full_text.is_empty() {
                full_text.push(' ');
            }
            full_text.push_str(&text);
        }

        offset = end;
    }

    Ok(full_text)
}
