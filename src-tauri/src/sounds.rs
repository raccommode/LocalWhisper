use std::io::Cursor;

/// Generate a sine wave tone as WAV bytes
fn generate_tone(frequency: f32, duration_ms: u32, volume: f32) -> Vec<u8> {
    let sample_rate: u32 = 44100;
    let num_samples = (sample_rate * duration_ms / 1000) as usize;
    let mut samples: Vec<i16> = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        // Apply a fade-in/fade-out envelope to avoid clicks
        let envelope = {
            let fade_samples = (sample_rate as f32 * 0.01) as usize; // 10ms fade
            if i < fade_samples {
                i as f32 / fade_samples as f32
            } else if i > num_samples - fade_samples {
                (num_samples - i) as f32 / fade_samples as f32
            } else {
                1.0
            }
        };
        let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * volume * envelope;
        samples.push((sample * i16::MAX as f32) as i16);
    }

    // Build a minimal WAV file in memory
    let data_size = (num_samples * 2) as u32;
    let file_size = 36 + data_size;
    let mut wav = Vec::with_capacity(file_size as usize + 8);

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for sample in &samples {
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    wav
}

/// Play a short ascending tone when recording starts (two quick notes going up)
pub fn play_start_sound() {
    let wav1 = generate_tone(880.0, 80, 0.3); // A5 - 80ms
    let wav2 = generate_tone(1100.0, 80, 0.3); // ~C#6 - 80ms

    play_wav_bytes(&wav1);
    std::thread::sleep(std::time::Duration::from_millis(30));
    play_wav_bytes(&wav2);
}

/// Play a short descending tone when recording stops (two quick notes going down)
pub fn play_stop_sound() {
    let wav1 = generate_tone(1100.0, 80, 0.3);
    let wav2 = generate_tone(880.0, 80, 0.3);

    play_wav_bytes(&wav1);
    std::thread::sleep(std::time::Duration::from_millis(30));
    play_wav_bytes(&wav2);
}

fn play_wav_bytes(wav_data: &[u8]) {
    use rodio::{Decoder, OutputStream, Sink};

    let Ok((_stream, handle)) = OutputStream::try_default() else {
        log::warn!("Impossible d'ouvrir la sortie audio pour le son de feedback");
        return;
    };

    let Ok(sink) = Sink::try_new(&handle) else {
        log::warn!("Impossible de creer le sink audio");
        return;
    };

    let cursor = Cursor::new(wav_data.to_vec());
    let Ok(source) = Decoder::new(cursor) else {
        log::warn!("Impossible de decoder le son de feedback");
        return;
    };

    sink.append(source);
    sink.sleep_until_end();
}
