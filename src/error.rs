use thiserror::Error;

/// Errores personalizados de la aplicación
#[derive(Error, Debug)]
pub enum VanityError {
    #[error("Error de I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de serialización JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error de serialización TOML: {0}")]
    Toml(#[from] toml::ser::Error),

    #[error("Error criptográfico: {0}")]
    Crypto(String),

    #[error("Prefijo inválido: {0}")]
    InvalidPrefix(String),

    #[error("Configuración inválida: {0}")]
    InvalidConfig(String),

    #[error("Error al crear directorio: {0}")]
    DirectoryCreation(String),

    #[error("Error al guardar clave: {0}")]
    KeyStorage(String),

    #[error("Búsqueda cancelada por el usuario")]
    Cancelled,

    #[error("Se alcanzó el límite máximo de intentos")]
    MaxAttemptsReached,
}

pub type Result<T> = std::result::Result<T, VanityError>;
