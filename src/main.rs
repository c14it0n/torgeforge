mod cli;
mod error;
mod generator;
mod search;
mod storage;
mod types;

use clap::Parser;
use cli::Cli;
use search::search_vanity;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

fn main() {
    // Parsear argumentos de línea de comandos
    let cli = Cli::parse();

    // Validar argumentos
    if let Err(e) = cli.validate() {
        eprintln!("❌ Error de validación: {}", e);
        std::process::exit(1);
    }

    // Guardar valores antes de mover cli
    let quiet = cli.quiet;
    
    // Mostrar configuración
    if !quiet {
        cli.print_config();
    }

    // Configurar manejador de señales para Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        println!("\n\n⚠️  Recibida señal de interrupción. Deteniendo búsqueda...");
        r.store(false, Ordering::Relaxed);
    })
    .expect("Error al configurar manejador de Ctrl+C");

    // Convertir CLI a Config
    let config = cli.into_config();
    let output_dir = config.output_dir.clone();
    let dry_run = config.dry_run;
    let threads = config.threads;

    // Crear directorio de salida si no existe (y no es dry-run)
    if !dry_run {
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            eprintln!("❌ Error al crear directorio de salida: {}", e);
            std::process::exit(1);
        }

        // Crear README en el directorio de salida
        if let Err(e) = storage::create_readme(&output_dir) {
            eprintln!("⚠️  Advertencia: No se pudo crear README: {}", e);
        }
    }

    println!("🚀 Iniciando búsqueda...");
    println!();

    let start_time = Instant::now();

    // Ejecutar búsqueda
    let search_result = search_vanity(&config, |result| {
        // Verificar si debemos detenernos
        if !running.load(Ordering::Relaxed) {
            return Err(error::VanityError::Cancelled);
        }

        // Mostrar resultado encontrado
        println!(
            "✅ [ENCONTRADO] prefijo=\"{}\" dirección=\"{}\"",
            result.matched_prefix, result.address
        );

        // Guardar en disco si no es dry-run
        if !dry_run {
            match storage::save_result(&result, &output_dir, threads) {
                Ok(path) => {
                    println!("   💾 Guardado en: {}", path.display());
                }
                Err(e) => {
                    eprintln!("   ⚠️  Error al guardar: {}", e);
                }
            }
        } else {
            println!("   🔍 Modo dry-run: no se guardó en disco");
        }

        println!();

        Ok(())
    });

    // Manejar resultado de la búsqueda
    match search_result {
        Ok(stats) => {
            let elapsed = start_time.elapsed();
            
            println!("╔════════════════════════════════════════════════════════════════╗");
            println!("║                      BÚSQUEDA COMPLETADA                       ║");
            println!("╚════════════════════════════════════════════════════════════════╝");
            println!();
            println!("📊 Estadísticas finales:");
            println!("   • Resultados encontrados: {}", stats.results_found);
            println!("   • Total de intentos: {}", stats.total_attempts);
            println!("   • Tiempo transcurrido: {:.2}s", elapsed.as_secs_f64());
            println!("   • Tasa promedio: {:.2} intentos/s", stats.rate());
            
            if stats.results_found > 0 {
                let avg_attempts = stats.total_attempts as f64 / stats.results_found as f64;
                println!("   • Promedio de intentos por resultado: {:.0}", avg_attempts);
            }
            
            println!();

            if !dry_run && stats.results_found > 0 {
                println!("📁 Archivos guardados en: {}", output_dir.display());
                println!();
                println!("⚠️  IMPORTANTE: Guarda las claves privadas de forma segura.");
                println!("   Las claves privadas dan control total sobre las direcciones .onion");
                println!();
            }
        }
        Err(e) => {
            match e {
                error::VanityError::Cancelled => {
                    println!("⚠️  Búsqueda cancelada por el usuario.");
                }
                error::VanityError::MaxAttemptsReached => {
                    println!("⚠️  Se alcanzó el límite máximo de intentos.");
                }
                _ => {
                    eprintln!("❌ Error durante la búsqueda: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
