use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use crate::errors::{AppError, AppResult};
use crate::tts_models;

/// Launch Piper TTS to speak the given text.
/// Returns the child process (afplay) so it can be killed later.
pub fn speak(app_data_dir: &PathBuf, text: &str, voice_id: &str, rate: u32) -> AppResult<Child> {
    let piper_bin = tts_models::piper_binary(app_data_dir);
    if !piper_bin.exists() {
        return Err(AppError::Tts(
            "Piper n'est pas installé. Installez-le via `pipx install piper-tts` ou depuis les réglages.".to_string(),
        ));
    }

    let model_path = tts_models::voice_model_path(app_data_dir, voice_id);
    if !model_path.exists() {
        return Err(AppError::Tts(format!(
            "Voix '{}' non téléchargée.",
            voice_id
        )));
    }

    let config_path = tts_models::voice_config_path(app_data_dir, voice_id);

    // Piper length_scale: <1 = faster, >1 = slower.
    // 180 wpm is "normal" (scale 1.0), 300 wpm → ~0.6, 80 wpm → ~2.25
    let length_scale = 180.0 / rate as f64;

    // Generate WAV to a temp file, then play with afplay (macOS built-in)
    let temp_dir = std::env::temp_dir();
    let wav_path = temp_dir.join("localwhisper_tts.wav");

    // Build piper command: echo text | piper --model X --config X --output_file X
    let mut cmd = Command::new(&piper_bin);
    cmd.arg("--model")
        .arg(&model_path)
        .arg("--output_file")
        .arg(&wav_path)
        .arg("--length_scale")
        .arg(format!("{:.2}", length_scale))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Only pass --config if the file exists (Python piper-tts auto-detects it)
    if config_path.exists() {
        cmd.arg("--config").arg(&config_path);
    }

    let piper_child = cmd
        .spawn()
        .map_err(|e| AppError::Tts(format!("Impossible de lancer Piper : {}", e)))?;

    // Write text to piper's stdin, then wait for it to finish
    let mut child = piper_child;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(text.as_bytes());
        // stdin is dropped here, closing it so piper processes
    }

    // Wait for piper to generate the WAV
    let output = child
        .wait_with_output()
        .map_err(|e| AppError::Tts(format!("Piper a échoué : {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Tts(format!(
            "Piper a échoué (code {}) : {}",
            output.status.code().unwrap_or(-1),
            stderr.trim()
        )));
    }

    // Verify the WAV was created
    if !wav_path.exists() {
        return Err(AppError::Tts("Piper n'a pas généré de fichier audio.".to_string()));
    }

    // Play the generated WAV with afplay (macOS built-in, non-blocking)
    let player = Command::new("afplay")
        .arg(&wav_path)
        .spawn()
        .map_err(|e| AppError::Tts(format!("Impossible de lire l'audio : {}", e)))?;

    Ok(player)
}

/// Stop the current TTS playback process.
pub fn stop(process: &mut Child) {
    let _ = process.kill();
    let _ = process.wait();
}

/// Check if the TTS playback is still running.
pub fn is_speaking(process: &mut Child) -> bool {
    match process.try_wait() {
        Ok(Some(_)) => false,
        Ok(None) => true,
        Err(_) => false,
    }
}
