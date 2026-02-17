use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use whisper_rs::WhisperContext;

use crate::config::AppConfig;

pub struct InnerState {
    pub config: AppConfig,
    pub app_data_dir: PathBuf,
    pub whisper_ctx: Option<Arc<WhisperContext>>,
    pub is_recording: bool,
    pub audio_buffer: Vec<f32>,
    pub sample_rate: u32,
    /// Live shared buffer written to by the audio stream callback
    pub shared_buffer: Option<Arc<Mutex<Vec<f32>>>>,
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<Mutex<InnerState>>,
}

impl AppState {
    pub fn new(config: AppConfig, app_data_dir: PathBuf) -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerState {
                config,
                app_data_dir,
                whisper_ctx: None,
                is_recording: false,
                audio_buffer: Vec::new(),
                sample_rate: 16000,
                shared_buffer: None,
            })),
        }
    }
}

// cpal::Stream is !Send+!Sync but we guard access through the Mutex
// and only touch the stream from the audio module.
#[allow(dead_code)]
pub struct StreamHandle(pub cpal::Stream);
unsafe impl Send for StreamHandle {}
unsafe impl Sync for StreamHandle {}

pub struct RecordingStream {
    pub stream: Mutex<Option<StreamHandle>>,
}

impl RecordingStream {
    pub fn new() -> Self {
        Self {
            stream: Mutex::new(None),
        }
    }
}
