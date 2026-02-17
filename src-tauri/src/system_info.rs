use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub total_ram_gb: f64,
    pub cpu_cores: usize,
    pub os: String,
    pub arch: String,
    pub recommended_model: String,
    pub recommended_model_reason: String,
}

pub fn get_system_info() -> SystemInfo {
    let sys = System::new_all();
    let ram_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    let cpu_cores = sys.cpus().len();
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();

    let (model, reason) = recommend_model(ram_gb);

    SystemInfo {
        total_ram_gb: (ram_gb * 10.0).round() / 10.0,
        cpu_cores,
        os,
        arch,
        recommended_model: model.to_string(),
        recommended_model_reason: reason.to_string(),
    }
}

fn recommend_model(ram_gb: f64) -> (&'static str, &'static str) {
    if ram_gb < 4.0 {
        (
            "ggml-tiny-q5_1",
            "Votre machine a moins de 4 Go de RAM. Le modèle Tiny quantisé est recommandé pour des performances optimales.",
        )
    } else if ram_gb < 8.0 {
        (
            "ggml-base",
            "Avec 4 à 8 Go de RAM, le modèle Base offre un bon équilibre entre précision et performance.",
        )
    } else if ram_gb < 16.0 {
        (
            "ggml-small",
            "Avec 8 à 16 Go de RAM, le modèle Small offre une bonne précision sans surcharger votre machine.",
        )
    } else {
        (
            "ggml-medium",
            "Avec plus de 16 Go de RAM, le modèle Medium offre une excellente précision de transcription.",
        )
    }
}
