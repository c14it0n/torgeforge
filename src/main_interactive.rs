mod error;
mod generator;
mod search;
mod storage;
mod types;
mod ui;

use search::search_vanity;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use ui::{
    interactive_config, pause, read_line, show_banner, show_final_stats, show_help,
    show_main_menu, show_result_found, show_search_header, show_time_estimation,
};

fn main() {
    // Mostrar banner
    show_banner();

    // Loop principal del menú
    loop {
        show_main_menu();

        let choice = read_line();

        match choice.as_str() {
            "1" => generate_vanity_address(),
            "2" => show_time_estimation(),
            "3" => {
                println!("\n⚙️  Configuración avanzada disponible en modo generación (opción 1)\n");
                pause();
            }
            "4" => show_help(),
            "5" => {
                println!("\n👋 ¡Gracias por usar Torge Forge!\n");
                println!("   Desarrollado por Nahum Deavila");
                println!("   Mantén tus claves seguras 🔐\n");
                std::process::exit(0);
            }
            _ => {
                println!("\n❌ Opción inválida. Por favor selecciona 1-5\n");
                pause();
            }
        }
    }
}

fn generate_vanity_address() {
    // Obtener configuración interactiva
    let config = interactive_config();

    let output_dir = config.output_dir.clone();
    let dry_run = config.dry_run;
    let threads = config.threads;
    let max_results = config.max_results;

    // Configurar manejador de señales para Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n\n⚠️  Recibida señal de interrupción. Deteniendo búsqueda...");
        r.store(false, Ordering::Relaxed);
    })
    .expect("Error al configurar manejador de Ctrl+C");

    // Crear directorio de salida si no existe (y no es dry-run)
    if !dry_run {
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            eprintln!("\n❌ Error al crear directorio de salida: {}\n", e);
            pause();
            return;
        }

        // Crear README en el directorio de salida
        if let Err(e) = storage::create_readme(&output_dir) {
            eprintln!("⚠️  Advertencia: No se pudo crear README: {}", e);
        }
    }

    show_search_header();

    let start_time = Instant::now();
    let mut results_count = 0;

    // Ejecutar búsqueda
    let search_result = search_vanity(&config, |result| {
        // Verificar si debemos detenernos
        if !running.load(Ordering::Relaxed) {
            return Err(error::VanityError::Cancelled);
        }

        results_count += 1;

        // Mostrar resultado encontrado
        show_result_found(
            &result.matched_prefix,
            &result.address,
            results_count,
            max_results,
        );

        // Guardar en disco si no es dry-run
        if !dry_run {
            match storage::save_result(&result, &output_dir, threads) {
                Ok(path) => {
                    println!("💾 Guardado en: {}\n", path.display());
                }
                Err(e) => {
                    eprintln!("⚠️  Error al guardar: {}\n", e);
                }
            }
        } else {
            println!("🔍 Modo dry-run: no se guardó en disco\n");
        }

        Ok(())
    });

    // Manejar resultado de la búsqueda
    match search_result {
        Ok(stats) => {
            let elapsed = start_time.elapsed();

            show_final_stats(
                stats.results_found,
                stats.total_attempts,
                elapsed.as_secs_f64(),
                &output_dir.display().to_string(),
                dry_run,
            );
        }
        Err(e) => match e {
            error::VanityError::Cancelled => {
                println!("\n⚠️  Búsqueda cancelada por el usuario.\n");
            }
            error::VanityError::MaxAttemptsReached => {
                println!("\n⚠️  Se alcanzó el límite máximo de intentos.\n");
            }
            _ => {
                eprintln!("\n❌ Error durante la búsqueda: {}\n", e);
            }
        },
    }

    pause();
}
