use crate::types::Config;
use clap::Parser;
use std::path::PathBuf;

/// Generador de direcciones .onion v3 vanity para servicios ocultos de Tor
#[derive(Parser, Debug)]
#[command(name = "vanity-onion-v3")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "Genera direcciones .onion v3 con prefijos personalizados", long_about = None)]
pub struct Cli {
    /// Prefijo(s) a buscar (puede especificarse múltiples veces)
    /// 
    /// Los prefijos deben usar solo caracteres válidos en base32: a-z y 2-7
    /// 
    /// Ejemplo: --prefix ctec --prefix nahum
    #[arg(short, long = "prefix", required = true)]
    pub prefixes: Vec<String>,

    /// Número de hilos a utilizar (por defecto: número de CPUs lógicas)
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub threads: usize,

    /// Número máximo de resultados a generar antes de detenerse
    #[arg(short = 'n', long, default_value_t = 1)]
    pub max_results: usize,

    /// Número máximo de intentos antes de detenerse (opcional, sin límite por defecto)
    #[arg(short = 'a', long)]
    pub max_attempts: Option<u64>,

    /// Directorio de salida para guardar las claves generadas
    #[arg(short, long, default_value = "./output")]
    pub output_dir: PathBuf,

    /// Modo dry-run: no guardar claves en disco (solo mostrar en consola)
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Mostrar estadísticas de progreso cada N segundos
    #[arg(long, default_value_t = 10)]
    pub stats_interval: u64,

    /// Modo silencioso: no mostrar estadísticas de progreso
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Mostrar estimación de tiempo para cada prefijo
    #[arg(long, default_value_t = false)]
    pub estimate: bool,
}

impl Cli {
    /// Convierte los argumentos CLI en una configuración
    pub fn into_config(self) -> Config {
        Config {
            prefixes: self.prefixes,
            threads: self.threads,
            max_results: self.max_results,
            max_attempts: self.max_attempts,
            output_dir: self.output_dir,
            dry_run: self.dry_run,
        }
    }

    /// Valida los argumentos de la CLI
    pub fn validate(&self) -> Result<(), String> {
        // Validar que hay al menos un prefijo
        if self.prefixes.is_empty() {
            return Err("Debe especificar al menos un prefijo con --prefix".to_string());
        }

        // Validar que los prefijos no estén vacíos
        for prefix in &self.prefixes {
            if prefix.is_empty() {
                return Err("Los prefijos no pueden estar vacíos".to_string());
            }

            // Validar caracteres del prefijo
            for ch in prefix.chars() {
                if !ch.is_ascii_lowercase() && !('2'..='7').contains(&ch) {
                    return Err(format!(
                        "Prefijo '{}' contiene caracteres inválidos. Solo se permiten a-z y 2-7",
                        prefix
                    ));
                }
            }

            // Advertir sobre prefijos muy largos
            if prefix.len() > 8 {
                eprintln!(
                    "⚠️  ADVERTENCIA: El prefijo '{}' es muy largo ({} caracteres).",
                    prefix,
                    prefix.len()
                );
                eprintln!(
                    "    Encontrar este prefijo podría tomar un tiempo extremadamente largo."
                );
                eprintln!(
                    "    Tiempo estimado: ~32^{} = 2^{} intentos",
                    prefix.len(),
                    prefix.len() * 5
                );
            }
        }

        // Validar número de hilos
        if self.threads == 0 {
            return Err("El número de hilos debe ser mayor que 0".to_string());
        }

        // Validar número de resultados
        if self.max_results == 0 {
            return Err("El número máximo de resultados debe ser mayor que 0".to_string());
        }

        Ok(())
    }

    /// Muestra información de configuración antes de iniciar
    pub fn print_config(&self) {
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          GENERADOR DE DIRECCIONES .ONION V3 VANITY            ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("📋 Configuración:");
        println!("   • Prefijos buscados: {}", self.prefixes.join(", "));
        println!("   • Hilos: {}", self.threads);
        println!("   • Resultados máximos: {}", self.max_results);
        
        if let Some(max_attempts) = self.max_attempts {
            println!("   • Intentos máximos: {}", max_attempts);
        } else {
            println!("   • Intentos máximos: ilimitado");
        }
        
        if self.dry_run {
            println!("   • Modo: DRY-RUN (no se guardarán claves)");
        } else {
            println!("   • Directorio de salida: {}", self.output_dir.display());
        }
        
        println!();
    }

    /// Muestra estimaciones de tiempo para los prefijos
    pub fn print_estimates(&self, rate: f64) {
        use crate::search::{estimate_time, format_duration};

        println!("⏱️  Estimaciones de tiempo (basadas en tasa actual: {:.0} intentos/s):", rate);
        println!();
        
        for prefix in &self.prefixes {
            let time = estimate_time(prefix, rate);
            let formatted = format_duration(time);
            let probability = 1.0 / 32f64.powi(prefix.len() as i32);
            
            println!(
                "   • '{}' ({} caracteres): ~{} (probabilidad: 1 en {:.0})",
                prefix,
                prefix.len(),
                formatted,
                1.0 / probability
            );
        }
        
        println!();
    }
}

// Función auxiliar para obtener el número de CPUs
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_prefix() {
        let cli = Cli {
            prefixes: vec!["test".to_string(), "abc2".to_string()],
            threads: 4,
            max_results: 1,
            max_attempts: None,
            output_dir: PathBuf::from("./output"),
            dry_run: false,
            stats_interval: 10,
            quiet: false,
            estimate: false,
        };

        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_prefix() {
        let cli = Cli {
            prefixes: vec!["TEST".to_string()], // Mayúsculas no permitidas
            threads: 4,
            max_results: 1,
            max_attempts: None,
            output_dir: PathBuf::from("./output"),
            dry_run: false,
            stats_interval: 10,
            quiet: false,
            estimate: false,
        };

        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_validate_empty_prefix() {
        let cli = Cli {
            prefixes: vec!["".to_string()],
            threads: 4,
            max_results: 1,
            max_attempts: None,
            output_dir: PathBuf::from("./output"),
            dry_run: false,
            stats_interval: 10,
            quiet: false,
            estimate: false,
        };

        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_validate_zero_threads() {
        let cli = Cli {
            prefixes: vec!["test".to_string()],
            threads: 0,
            max_results: 1,
            max_attempts: None,
            output_dir: PathBuf::from("./output"),
            dry_run: false,
            stats_interval: 10,
            quiet: false,
            estimate: false,
        };

        assert!(cli.validate().is_err());
    }
}
