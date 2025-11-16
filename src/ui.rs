use crate::types::Config;
use std::io::{self, Write};
use std::path::PathBuf;

/// Muestra el banner de Torge Forge
pub fn show_banner() {
    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║           ████████╗ ██████╗ ██████╗  ██████╗ ███████╗        ║");
    println!("║           ╚══██╔══╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝        ║");
    println!("║              ██║   ██║   ██║██████╔╝██║  ███╗█████╗          ║");
    println!("║              ██║   ██║   ██║██╔══██╗██║   ██║██╔══╝          ║");
    println!("║              ██║   ╚██████╔╝██║  ██║╚██████╔╝███████╗        ║");
    println!("║              ╚═╝    ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝        ║");
    println!("║                                                               ║");
    println!("║              ███████╗ ██████╗ ██████╗  ██████╗ ███████╗      ║");
    println!("║              ██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝      ║");
    println!("║              █████╗  ██║   ██║██████╔╝██║  ███╗█████╗        ║");
    println!("║              ██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝        ║");
    println!("║              ██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗      ║");
    println!("║              ╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝      ║");
    println!("║                                                               ║");
    println!("║                    by Nahum Deavila                           ║");
    println!("║                                                               ║");
    println!("║              Vanity .onion v3 Address Generator               ║");
    println!("║                        v0.1.0                                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n");
}

/// Muestra el menú principal
pub fn show_main_menu() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                        MENÚ PRINCIPAL                         ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║  [1] 🎯 Generar dirección .onion vanity                       ║");
    println!("║  [2] 📊 Estimar tiempo de búsqueda                            ║");
    println!("║  [3] ⚙️  Configuración avanzada                                ║");
    println!("║  [4] ℹ️  Información y ayuda                                   ║");
    println!("║  [5] 🚪 Salir                                                 ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    print!("\n👉 Selecciona una opción [1-5]: ");
    io::stdout().flush().unwrap();
}

/// Lee una línea de entrada del usuario
pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Lee un número entero del usuario
pub fn read_number(prompt: &str, default: usize) -> usize {
    print!("{} [default: {}]: ", prompt, default);
    io::stdout().flush().unwrap();
    
    let input = read_line();
    if input.is_empty() {
        default
    } else {
        input.parse().unwrap_or(default)
    }
}

/// Lee una confirmación (s/n)
pub fn read_confirmation(prompt: &str) -> bool {
    print!("{} [s/n]: ", prompt);
    io::stdout().flush().unwrap();
    
    let input = read_line().to_lowercase();
    input == "s" || input == "si" || input == "y" || input == "yes"
}

/// Solicita los prefijos al usuario
pub fn get_prefixes() -> Vec<String> {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    CONFIGURAR PREFIJOS                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n📝 Los prefijos deben usar solo: a-z y 2-7 (alfabeto base32)");
    println!("⚠️  Prefijos largos (>5 caracteres) pueden tomar mucho tiempo\n");
    
    let mut prefixes = Vec::new();
    
    loop {
        print!("Ingresa un prefijo (o Enter para terminar): ");
        io::stdout().flush().unwrap();
        
        let prefix = read_line().to_lowercase();
        
        if prefix.is_empty() {
            if prefixes.is_empty() {
                println!("❌ Debes ingresar al menos un prefijo");
                continue;
            }
            break;
        }
        
        // Validar prefijo
        if !is_valid_prefix(&prefix) {
            println!("❌ Prefijo inválido. Solo usa: a-z y 2-7");
            continue;
        }
        
        if prefix.len() > 8 {
            println!("⚠️  ADVERTENCIA: Prefijo muy largo ({} caracteres)", prefix.len());
            println!("   Esto puede tomar días o semanas");
            if !read_confirmation("¿Continuar de todas formas?") {
                continue;
            }
        }
        
        prefixes.push(prefix.clone());
        println!("✅ Prefijo '{}' agregado", prefix);
        
        if !read_confirmation("\n¿Agregar otro prefijo?") {
            break;
        }
    }
    
    println!("\n📋 Prefijos configurados: {}", prefixes.join(", "));
    prefixes
}

/// Valida que un prefijo sea válido (solo a-z y 2-7)
fn is_valid_prefix(prefix: &str) -> bool {
    if prefix.is_empty() {
        return false;
    }
    
    prefix.chars().all(|c| c.is_ascii_lowercase() || ('2'..='7').contains(&c))
}

/// Configuración interactiva completa
pub fn interactive_config() -> Config {
    show_banner();
    
    println!("🎯 Configuración de generación de dirección .onion vanity\n");
    
    // Obtener prefijos
    let prefixes = get_prefixes();
    
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                  CONFIGURACIÓN DE RENDIMIENTO                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    // Número de hilos
    let default_threads = num_cpus::get();
    let threads = read_number(
        &format!("💻 Número de hilos (CPUs disponibles: {})", default_threads),
        default_threads
    );
    
    // Número de resultados
    let max_results = read_number("🎯 Número máximo de resultados", 1);
    
    // Límite de intentos
    let max_attempts = if read_confirmation("\n⏱️  ¿Establecer límite de intentos?") {
        Some(read_number("   Número máximo de intentos", 1_000_000) as u64)
    } else {
        None
    };
    
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                  CONFIGURACIÓN DE SALIDA                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    // Directorio de salida
    print!("📁 Directorio de salida [default: ./output]: ");
    io::stdout().flush().unwrap();
    let output_input = read_line();
    let output_dir = if output_input.is_empty() {
        PathBuf::from("./output")
    } else {
        PathBuf::from(output_input)
    };
    
    // Modo dry-run
    let dry_run = read_confirmation("\n🔍 ¿Modo dry-run (no guardar en disco)?");
    
    // Resumen de configuración
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    RESUMEN DE CONFIGURACIÓN                   ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║  📝 Prefijos: {:45} ║", format!("{}", prefixes.join(", ")));
    println!("║  💻 Hilos: {:48} ║", threads);
    println!("║  🎯 Resultados máximos: {:36} ║", max_results);
    if let Some(attempts) = max_attempts {
        println!("║  ⏱️  Intentos máximos: {:37} ║", attempts);
    } else {
        println!("║  ⏱️  Intentos máximos: {:37} ║", "Ilimitado");
    }
    println!("║  📁 Directorio: {:44} ║", output_dir.display());
    println!("║  🔍 Modo dry-run: {:40} ║", if dry_run { "Sí" } else { "No" });
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    if !read_confirmation("¿Iniciar búsqueda con esta configuración?") {
        println!("\n❌ Búsqueda cancelada\n");
        std::process::exit(0);
    }
    
    Config {
        prefixes,
        threads,
        max_results,
        max_attempts,
        output_dir,
        dry_run,
    }
}

/// Muestra información y ayuda
pub fn show_help() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    INFORMACIÓN Y AYUDA                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    println!("📖 ¿Qué es Torge Forge?");
    println!("   Torge Forge es un generador de direcciones .onion v3 vanity para");
    println!("   servicios ocultos de Tor. Permite crear direcciones personalizadas");
    println!("   que comiencen con un prefijo de tu elección.\n");
    
    println!("🔐 Seguridad:");
    println!("   • Usa criptografía Ed25519 y SHA3-256");
    println!("   • Generador de números aleatorios criptográficamente seguro");
    println!("   • Las claves privadas se guardan de forma segura\n");
    
    println!("⚡ Rendimiento:");
    println!("   • Paralelización multi-hilo para máximo rendimiento");
    println!("   • Aprovecha todos los núcleos de tu CPU\n");
    
    println!("📊 Tiempos estimados (en un i7 12ª gen):");
    println!("   • 1-2 caracteres: < 1 segundo");
    println!("   • 3 caracteres: ~30 segundos");
    println!("   • 4 caracteres: ~15 minutos");
    println!("   • 5 caracteres: ~8 horas");
    println!("   • 6+ caracteres: días o semanas\n");
    
    println!("⚠️  Importante:");
    println!("   • Solo usa caracteres: a-z y 2-7 (alfabeto base32)");
    println!("   • Guarda las claves privadas de forma segura");
    println!("   • Las claves dan control total sobre la dirección .onion\n");
    
    println!("📁 Archivos generados:");
    println!("   • *_private.key - Clave privada (¡GUARDAR SEGURA!)");
    println!("   • *_metadata.json - Metadatos de la generación");
    println!("   • *_hostname.txt - Solo la dirección .onion");
    println!("   • README.txt - Instrucciones de uso\n");
    
    println!("🔗 Más información:");
    println!("   • README.md - Documentación completa");
    println!("   • SECURITY.md - Guía de seguridad");
    println!("   • EXAMPLES.md - Ejemplos de uso\n");
    
    pause();
}

/// Muestra estimación de tiempos
pub fn show_time_estimation() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                  ESTIMACIÓN DE TIEMPOS                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    print!("Ingresa el prefijo para estimar: ");
    io::stdout().flush().unwrap();
    let prefix = read_line().to_lowercase();
    
    if !is_valid_prefix(&prefix) {
        println!("❌ Prefijo inválido\n");
        pause();
        return;
    }
    
    let len = prefix.len();
    let probability = 1.0 / 32f64.powi(len as i32);
    let expected_attempts = 1.0 / probability;
    
    // Asumir ~300k intentos/s en hardware moderno
    let rate = 300_000.0;
    let seconds = expected_attempts / rate;
    
    println!("\n📊 Estimación para prefijo '{}':", prefix);
    println!("   • Longitud: {} caracteres", len);
    println!("   • Probabilidad: 1 en {:.0}", 1.0 / probability);
    println!("   • Intentos esperados: {:.0}", expected_attempts);
    println!("   • Tiempo estimado (300k intentos/s): {}", format_duration(seconds));
    
    if len >= 6 {
        println!("\n⚠️  ADVERTENCIA: Este prefijo puede tomar mucho tiempo");
        println!("   Considera usar un prefijo más corto");
    }
    
    println!();
    pause();
}

/// Formatea una duración en segundos a texto legible
fn format_duration(seconds: f64) -> String {
    if seconds < 1.0 {
        return "< 1 segundo".to_string();
    }
    
    let total_secs = seconds as u64;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    
    let mut parts = Vec::new();
    
    if days > 0 {
        parts.push(format!("{} día{}", days, if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        parts.push(format!("{} hora{}", hours, if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 {
        parts.push(format!("{} min", minutes));
    }
    if secs > 0 || parts.is_empty() {
        parts.push(format!("{} seg", secs));
    }
    
    parts.join(", ")
}

/// Pausa hasta que el usuario presione Enter
pub fn pause() {
    print!("Presiona Enter para continuar...");
    io::stdout().flush().unwrap();
    read_line();
}

/// Obtiene el número de CPUs disponibles
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}

/// Muestra barra de progreso durante la búsqueda
pub fn show_search_header() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    BÚSQUEDA EN PROGRESO                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("🔍 Generando direcciones .onion...");
    println!("⏸️  Presiona Ctrl+C para detener\n");
}

/// Muestra resultado encontrado
pub fn show_result_found(prefix: &str, address: &str, count: usize, total: usize) {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    ✅ RESULTADO ENCONTRADO                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n🎯 Prefijo: {}", prefix);
    println!("🧅 Dirección: {}", address);
    println!("📊 Progreso: {}/{}\n", count, total);
}

/// Muestra estadísticas finales
pub fn show_final_stats(results: usize, attempts: u64, elapsed: f64, output_dir: &str, dry_run: bool) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    BÚSQUEDA COMPLETADA                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    println!("📊 Estadísticas:");
    println!("   • Resultados encontrados: {}", results);
    println!("   • Total de intentos: {}", attempts);
    println!("   • Tiempo transcurrido: {:.2}s", elapsed);
    println!("   • Tasa promedio: {:.0} intentos/s", attempts as f64 / elapsed);
    
    if results > 0 {
        println!("   • Promedio por resultado: {:.0} intentos", attempts as f64 / results as f64);
    }
    
    if !dry_run && results > 0 {
        println!("\n📁 Archivos guardados en: {}", output_dir);
        println!("\n📦 Archivos generados por cada dirección:");
        println!("   • <address>_private.key - Clave privada hex (backup)");
        println!("   • <address>_metadata.json - Metadatos");
        println!("   • <address>_hostname.txt - Dirección .onion");
        println!("   • <address>_tor/ - ✨ Carpeta lista para Tor:");
        println!("       - hs_ed25519_secret_key (formato binario Tor)");
        println!("       - hostname (dirección .onion)");
        println!("\n🚀 Uso directo en Tor (Linux):");
        println!("   sudo cp <address>_tor/* /var/lib/tor/hidden_service/");
        println!("   sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/");
        println!("   sudo chmod 700 /var/lib/tor/hidden_service/");
        println!("   sudo systemctl restart tor");
        println!("\n⚠️  IMPORTANTE:");
        println!("   • Guarda las claves privadas de forma segura");
        println!("   • Las claves dan control total sobre las direcciones .onion");
        println!("   • Haz backup en múltiples ubicaciones seguras");
    }
    
    println!();
}
