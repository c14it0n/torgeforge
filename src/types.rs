use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuración principal de la aplicación
#[derive(Debug, Clone)]
pub struct Config {
    /// Prefijos a buscar (en minúsculas, alfabeto base32)
    pub prefixes: Vec<String>,
    /// Número de hilos a utilizar
    pub threads: usize,
    /// Número máximo de resultados a generar
    pub max_results: usize,
    /// Número máximo de intentos antes de detenerse (None = ilimitado)
    pub max_attempts: Option<u64>,
    /// Directorio de salida para guardar las claves
    pub output_dir: PathBuf,
    /// Modo dry-run (no guardar en disco)
    pub dry_run: bool,
}

impl Config {
    /// Valida que los prefijos sean válidos para direcciones .onion v3
    /// El alfabeto base32 válido es: a-z y 2-7
    pub fn validate_prefixes(&self) -> Result<(), String> {
        for prefix in &self.prefixes {
            if prefix.is_empty() {
                return Err("Los prefijos no pueden estar vacíos".to_string());
            }
            
            for ch in prefix.chars() {
                if !ch.is_ascii_lowercase() && !('2'..='7').contains(&ch) {
                    return Err(format!(
                        "Prefijo '{}' contiene caracteres inválidos. Solo se permiten a-z y 2-7",
                        prefix
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Resultado de una búsqueda exitosa
#[derive(Debug, Clone)]
pub struct VanityResult {
    /// Dirección .onion completa (incluyendo .onion)
    pub address: String,
    /// Prefijo que coincidió
    pub matched_prefix: String,
    /// Clave privada Ed25519 (32 bytes)
    pub private_key: [u8; 32],
    /// Clave pública Ed25519 (32 bytes)
    pub public_key: [u8; 32],
    /// Timestamp de generación
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Metadatos a guardar en disco junto con la clave
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Dirección .onion generada
    pub onion_address: String,
    /// Prefijo usado para la búsqueda
    pub matched_prefix: String,
    /// Fecha y hora de generación (ISO 8601)
    pub generated_at: String,
    /// Número de hilos utilizados
    pub threads_used: usize,
    /// Clave pública en hexadecimal
    pub public_key_hex: String,
}

/// Estadísticas de la búsqueda
#[derive(Debug, Clone, Default)]
pub struct SearchStats {
    /// Total de intentos realizados
    pub total_attempts: u64,
    /// Número de resultados encontrados
    pub results_found: usize,
    /// Tiempo transcurrido en segundos
    pub elapsed_seconds: f64,
}

impl SearchStats {
    /// Calcula la tasa de generación (intentos por segundo)
    pub fn rate(&self) -> f64 {
        if self.elapsed_seconds > 0.0 {
            self.total_attempts as f64 / self.elapsed_seconds
        } else {
            0.0
        }
    }
}
