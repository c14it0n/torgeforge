use crate::error::{Result, VanityError};
use crate::generator::{generate_keypair, generate_onion_address, matches_prefix};
use crate::types::{Config, SearchStats, VanityResult};
use crossbeam::channel::bounded;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Ejecuta la búsqueda de direcciones vanity en paralelo
/// 
/// Utiliza rayon para paralelizar la generación de claves y búsqueda.
/// La búsqueda se detiene cuando:
/// - Se alcanza max_results
/// - Se alcanza max_attempts (si está configurado)
/// - Se recibe una señal de cancelación
/// 
/// # Arguments
/// 
/// * `config` - Configuración de la búsqueda
/// * `callback` - Función que se llama cada vez que se encuentra un resultado
/// 
/// # Returns
/// 
/// Estadísticas de la búsqueda
pub fn search_vanity<F>(config: &Config, mut callback: F) -> Result<SearchStats>
where
    F: FnMut(VanityResult) -> Result<()>,
{
    // Validar prefijos
    config.validate_prefixes().map_err(VanityError::InvalidPrefix)?;

    // Configurar el pool global de rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.threads)
        .build_global()
        .ok(); // Ignorar error si ya está configurado

    // Contadores atómicos compartidos entre hilos
    let attempts = Arc::new(AtomicU64::new(0));
    let found = Arc::new(AtomicU64::new(0));
    let should_stop = Arc::new(AtomicBool::new(false));

    // Canal para comunicar resultados desde los hilos workers al hilo principal
    let (tx, rx) = bounded::<VanityResult>(100);

    let start_time = Instant::now();

    // Clonar referencias para el closure
    let attempts_clone = Arc::clone(&attempts);
    let found_clone = Arc::clone(&found);
    let should_stop_clone = Arc::clone(&should_stop);
    let prefixes = config.prefixes.clone();
    let max_attempts = config.max_attempts;
    let max_results = config.max_results;

    // Spawn del worker paralelo
    let worker_handle = std::thread::spawn(move || {
        // Usar repeat_n que es el reemplazo de repeatn
        use rayon::iter::repeat_n;
        repeat_n((), usize::MAX)
            .try_for_each_with(tx, |tx, _| {
                // Verificar si debemos detenernos
                if should_stop_clone.load(Ordering::Relaxed) {
                    return Err(());
                }

                // Verificar límite de intentos
                let current_attempts = attempts_clone.fetch_add(1, Ordering::Relaxed) + 1;
                if let Some(max) = max_attempts {
                    if current_attempts >= max {
                        should_stop_clone.store(true, Ordering::Relaxed);
                        return Err(());
                    }
                }

                // Generar par de claves
                let (private_key, public_key) = generate_keypair();

                // Generar dirección .onion
                let address = generate_onion_address(&public_key);

                // Verificar si coincide con algún prefijo
                if let Some(matched_prefix) = matches_prefix(&address, &prefixes) {
                    let result = VanityResult {
                        address,
                        matched_prefix,
                        private_key,
                        public_key,
                        timestamp: chrono::Utc::now(),
                    };

                    // Enviar resultado al hilo principal
                    if tx.send(result).is_err() {
                        // El receptor se cerró, detener
                        should_stop_clone.store(true, Ordering::Relaxed);
                        return Err(());
                    }

                    // Incrementar contador de encontrados
                    let current_found = found_clone.fetch_add(1, Ordering::Relaxed) + 1;

                    // Verificar si alcanzamos el máximo de resultados
                    if current_found >= max_results as u64 {
                        should_stop_clone.store(true, Ordering::Relaxed);
                        return Err(());
                    }
                }

                Ok(())
            })
    });

    // Hilo principal: recibir y procesar resultados
    let mut results_count = 0;
    let mut last_error = None;

    for result in rx {
        results_count += 1;

        // Llamar al callback con el resultado
        if let Err(e) = callback(result) {
            last_error = Some(e);
            should_stop.store(true, Ordering::Relaxed);
            break;
        }

        // Verificar si alcanzamos el máximo
        if results_count >= config.max_results {
            should_stop.store(true, Ordering::Relaxed);
            break;
        }
    }

    // Esperar a que terminen los workers
    let _ = worker_handle.join();

    // Si hubo un error en el callback, propagarlo
    if let Some(e) = last_error {
        return Err(e);
    }

    // Recopilar estadísticas
    let elapsed = start_time.elapsed();
    let stats = SearchStats {
        total_attempts: attempts.load(Ordering::Relaxed),
        results_found: results_count,
        elapsed_seconds: elapsed.as_secs_f64(),
    };

    Ok(stats)
}

/// Muestra estadísticas de progreso durante la búsqueda
/// 
/// # Arguments
/// 
/// * `stats` - Estadísticas actuales
pub fn print_stats(stats: &SearchStats) {
    let rate = stats.rate();
    println!(
        "[STATS] Intentos: {} | Encontrados: {} | Tasa: {:.2} intentos/s | Tiempo: {:.2}s",
        stats.total_attempts,
        stats.results_found,
        rate,
        stats.elapsed_seconds
    );
}

/// Estima el tiempo necesario para encontrar un prefijo
/// 
/// Calcula una estimación basada en la longitud del prefijo y la tasa de generación.
/// 
/// # Arguments
/// 
/// * `prefix` - Prefijo a buscar
/// * `rate` - Tasa de generación (intentos por segundo)
/// 
/// # Returns
/// 
/// Tiempo estimado en segundos
pub fn estimate_time(prefix: &str, rate: f64) -> f64 {
    // El alfabeto base32 tiene 32 caracteres
    // La probabilidad de encontrar un prefijo de longitud n es 1/32^n
    let probability = 1.0 / 32f64.powi(prefix.len() as i32);
    let expected_attempts = 1.0 / probability;
    
    if rate > 0.0 {
        expected_attempts / rate
    } else {
        f64::INFINITY
    }
}

/// Formatea un tiempo en segundos a una cadena legible
/// 
/// # Arguments
/// 
/// * `seconds` - Tiempo en segundos
/// 
/// # Returns
/// 
/// String formateado (ej: "2h 30m 15s")
pub fn format_duration(seconds: f64) -> String {
    if seconds.is_infinite() {
        return "∞".to_string();
    }

    let total_secs = seconds as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_time() {
        // Para un prefijo de 1 carácter con tasa de 1000/s
        let time = estimate_time("a", 1000.0);
        // Probabilidad = 1/32, esperamos ~32 intentos, a 1000/s = 0.032s
        assert!(time > 0.0 && time < 1.0);

        // Para un prefijo de 2 caracteres
        let time = estimate_time("ab", 1000.0);
        // Probabilidad = 1/1024, esperamos ~1024 intentos, a 1000/s = ~1s
        assert!(time > 0.5 && time < 2.0);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30.0), "30s");
        assert_eq!(format_duration(90.0), "1m 30s");
        assert_eq!(format_duration(3661.0), "1h 1m 1s");
        assert_eq!(format_duration(f64::INFINITY), "∞");
    }

    #[test]
    fn test_search_vanity_basic() {
        use std::path::PathBuf;

        let config = Config {
            prefixes: vec!["a".to_string()], // Prefijo muy común
            threads: 2,
            max_results: 1,
            max_attempts: Some(100000), // Límite de seguridad
            output_dir: PathBuf::from("./test_output"),
            dry_run: true,
        };

        let mut found_count = 0;
        let result = search_vanity(&config, |result| {
            found_count += 1;
            assert!(result.address.starts_with("a"));
            Ok(())
        });

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.results_found > 0 || stats.total_attempts >= 100000);
    }
}
