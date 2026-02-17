use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

use crate::errors::{AppError, AppResult};
use crate::state::{AppState, RecordingStream, StreamHandle};

#[derive(Debug, Clone, Serialize)]
pub struct AudioDevice {
    pub name: String,
    pub is_default: bool,
}

pub fn list_input_devices() -> AppResult<Vec<AudioDevice>> {
    let host = cpal::default_host();

    let default_name = host
        .default_input_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    let devices = host
        .input_devices()
        .map_err(|e| AppError::Audio(format!("Enumeration des peripheriques impossible : {}", e)))?
        .filter_map(|d| {
            let name = d.name().ok()?;
            Some(AudioDevice {
                is_default: name == default_name,
                name,
            })
        })
        .collect();

    Ok(devices)
}

pub fn start_recording(
    state: &AppState,
    recording_stream: &RecordingStream,
    device_name: Option<String>,
) -> AppResult<()> {
    let host = cpal::default_host();

    let device = match device_name {
        Some(ref name) => host
            .input_devices()
            .map_err(|e| AppError::Audio(format!("Enumeration impossible : {}", e)))?
            .find(|d| d.name().ok().as_deref() == Some(name.as_str()))
            .ok_or_else(|| AppError::Audio(format!("Peripherique introuvable : {}", name)))?,
        None => host
            .default_input_device()
            .ok_or_else(|| AppError::Audio("Aucun micro par defaut detecte".into()))?,
    };

    let supported = device
        .default_input_config()
        .map_err(|e| AppError::Audio(format!("Config audio impossible : {}", e)))?;

    let native_rate = supported.sample_rate().0;
    let channels = supported.channels() as usize;

    let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let buf_writer = buffer.clone();

    let stream_config = cpal::StreamConfig {
        channels: supported.channels(),
        sample_rate: supported.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    };

    let stream = device
        .build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = buf_writer.lock().unwrap();
                if channels == 1 {
                    buf.extend_from_slice(data);
                } else {
                    // Mix multi-channel down to mono
                    for chunk in data.chunks(channels) {
                        let mono: f32 = chunk.iter().sum::<f32>() / channels as f32;
                        buf.push(mono);
                    }
                }
            },
            |err| log::error!("Erreur flux audio : {}", err),
            None,
        )
        .map_err(|e| AppError::Audio(format!("Creation du flux impossible : {}", e)))?;

    stream
        .play()
        .map_err(|e| AppError::Audio(format!("Demarrage du flux impossible : {}", e)))?;

    // Store the stream so it stays alive
    {
        let mut lock = recording_stream.stream.lock().unwrap();
        *lock = Some(StreamHandle(stream));
    }

    // Store the shared buffer reference in state so we can read it directly on stop
    {
        let mut inner = state.inner.lock().unwrap();
        inner.is_recording = true;
        inner.audio_buffer = Vec::new();
        inner.sample_rate = native_rate;
        inner.shared_buffer = Some(buffer);
    }

    Ok(())
}

pub fn stop_recording(
    state: &AppState,
    recording_stream: &RecordingStream,
) -> AppResult<Vec<f32>> {
    // Mark as no longer recording and take the shared buffer reference
    let shared_buf = {
        let mut inner = state.inner.lock().unwrap();
        inner.is_recording = false;
        inner.shared_buffer.take()
    };

    // Drop the stream to stop the audio hardware — no more callbacks after this
    {
        let mut lock = recording_stream.stream.lock().unwrap();
        *lock = None;
    }

    // Now safely read the captured audio directly from the shared buffer.
    // The stream is stopped so no more writes are happening.
    let (raw, rate) = {
        let captured = match shared_buf {
            Some(buf) => {
                let locked = buf.lock().unwrap();
                locked.clone()
            }
            None => Vec::new(),
        };
        let inner = state.inner.lock().unwrap();
        (captured, inner.sample_rate)
    };

    log::info!("Audio capture: {} echantillons a {} Hz", raw.len(), rate);

    // Whisper expects 16 kHz mono
    if rate != 16000 && !raw.is_empty() {
        Ok(resample(&raw, rate, 16000))
    } else {
        Ok(raw)
    }
}

pub fn test_microphone(app: AppHandle, device_name: Option<String>) -> AppResult<()> {
    let host = cpal::default_host();

    let device = match device_name {
        Some(ref name) => host
            .input_devices()
            .map_err(|e| AppError::Audio(format!("Enumeration impossible : {}", e)))?
            .find(|d| d.name().ok().as_deref() == Some(name.as_str()))
            .ok_or_else(|| AppError::Audio(format!("Peripherique introuvable : {}", name)))?,
        None => host
            .default_input_device()
            .ok_or_else(|| AppError::Audio("Aucun micro par defaut detecte".into()))?,
    };

    let supported = device
        .default_input_config()
        .map_err(|e| AppError::Audio(format!("Config audio impossible : {}", e)))?;

    let channels = supported.channels() as usize;

    let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let buf_writer = buffer.clone();

    let stream_config = cpal::StreamConfig {
        channels: supported.channels(),
        sample_rate: supported.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    };

    let stream = device
        .build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = buf_writer.lock().unwrap();
                if channels == 1 {
                    buf.extend_from_slice(data);
                } else {
                    for chunk in data.chunks(channels) {
                        let mono: f32 = chunk.iter().sum::<f32>() / channels as f32;
                        buf.push(mono);
                    }
                }
            },
            |err| log::error!("Erreur flux audio (test micro) : {}", err),
            None,
        )
        .map_err(|e| AppError::Audio(format!("Creation du flux impossible : {}", e)))?;

    stream
        .play()
        .map_err(|e| AppError::Audio(format!("Demarrage du flux impossible : {}", e)))?;

    // Record for 3 seconds, emitting RMS level every 100ms
    for _ in 0..30 {
        std::thread::sleep(std::time::Duration::from_millis(100));

        let rms = {
            let mut buf = buffer.lock().unwrap();
            let level = if buf.is_empty() {
                0.0
            } else {
                let sum_sq: f64 = buf.iter().map(|s| (*s as f64) * (*s as f64)).sum();
                (sum_sq / buf.len() as f64).sqrt()
            };
            buf.clear();
            level
        };

        // Clamp to 0.0..1.0 (RMS of normal speech is typically 0.01-0.1)
        let normalized = (rms * 10.0).min(1.0);
        let _ = app.emit("mic-test-level", normalized);
    }

    drop(stream);
    Ok(())
}

/// Linear interpolation resampler.
fn resample(input: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if input.is_empty() {
        return Vec::new();
    }

    let ratio = to_rate as f64 / from_rate as f64;
    let out_len = (input.len() as f64 * ratio) as usize;
    let mut output = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src = i as f64 / ratio;
        let lo = src.floor() as usize;
        let hi = (lo + 1).min(input.len() - 1);
        let frac = src - lo as f64;
        let sample = input[lo] as f64 * (1.0 - frac) + input[hi] as f64 * frac;
        output.push(sample as f32);
    }

    output
}
