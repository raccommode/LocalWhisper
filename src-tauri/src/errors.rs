use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Erreur audio : {0}")]
    Audio(String),

    #[error("Erreur de transcription : {0}")]
    Transcription(String),

    #[error("Erreur de configuration : {0}")]
    Config(String),

    #[error("Erreur presse-papier : {0}")]
    Clipboard(String),

    #[error("Erreur raccourci clavier : {0}")]
    Hotkey(String),

    #[error("Erreur de téléchargement : {0}")]
    Download(String),

    #[error("Erreur I/O : {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
