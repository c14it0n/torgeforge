use crate::error::{Result, VanityError};
use crate::types::{KeyMetadata, VanityResult};
use std::fs;
use std::path::{Path, PathBuf};

/// Guarda un resultado de búsqueda en disco
/// 
/// Crea los siguientes archivos:
/// 1. `<address>_private.key` - Clave privada en formato hexadecimal (backup)
/// 2. `<address>_tor/hs_ed25519_secret_key` - Clave en formato binario de Tor (listo para usar)
/// 3. `<address>_tor/hostname` - Dirección .onion (formato Tor)
/// 4. `<address>_metadata.json` - Metadatos en formato JSON
/// 
/// # Arguments
/// 
/// * `result` - Resultado de la búsqueda a guardar
/// * `output_dir` - Directorio donde guardar los archivos
/// * `threads_used` - Número de hilos utilizados en la búsqueda
/// 
/// # Returns
/// 
/// PathBuf con la ruta al directorio Tor generado
pub fn save_result(
    result: &VanityResult,
    output_dir: &Path,
    threads_used: usize,
) -> Result<PathBuf> {
    // Crear el directorio de salida si no existe
    fs::create_dir_all(output_dir).map_err(|e| {
        VanityError::DirectoryCreation(format!("No se pudo crear {}: {}", output_dir.display(), e))
    })?;

    // Extraer el nombre base de la dirección (sin .onion)
    let base_name = result
        .address
        .strip_suffix(".onion")
        .unwrap_or(&result.address);

    // Construir rutas de archivos
    let private_key_path = output_dir.join(format!("{}_private.key", base_name));
    let metadata_path = output_dir.join(format!("{}_metadata.json", base_name));
    let hostname_path = output_dir.join(format!("{}_hostname.txt", base_name));
    
    // Crear directorio para archivos de Tor
    let tor_dir = output_dir.join(format!("{}_tor", base_name));
    fs::create_dir_all(&tor_dir).map_err(|e| {
        VanityError::DirectoryCreation(format!("No se pudo crear directorio Tor {}: {}", tor_dir.display(), e))
    })?;

    // Guardar clave privada en formato hexadecimal (backup)
    let private_key_hex = hex::encode(result.private_key);
    fs::write(&private_key_path, &private_key_hex).map_err(|e| {
        VanityError::KeyStorage(format!(
            "No se pudo guardar la clave privada en {}: {}",
            private_key_path.display(),
            e
        ))
    })?;
    
    // Guardar clave privada en formato Tor (binario)
    save_tor_secret_key(&result.private_key, &tor_dir)?;
    
    // Guardar hostname en formato Tor
    save_tor_hostname(&result.address, &tor_dir)?;

    // Guardar clave pública en formato hexadecimal
    let public_key_hex = hex::encode(result.public_key);

    // Crear metadatos
    let metadata = KeyMetadata {
        onion_address: result.address.clone(),
        matched_prefix: result.matched_prefix.clone(),
        generated_at: result.timestamp.to_rfc3339(),
        threads_used,
        public_key_hex,
    };

    // Guardar metadatos en JSON
    let metadata_json = serde_json::to_string_pretty(&metadata)?;
    fs::write(&metadata_path, metadata_json).map_err(|e| {
        VanityError::KeyStorage(format!(
            "No se pudo guardar los metadatos en {}: {}",
            metadata_path.display(),
            e
        ))
    })?;

    // Guardar hostname (solo la dirección .onion) para facilitar su uso
    fs::write(&hostname_path, &result.address).map_err(|e| {
        VanityError::KeyStorage(format!(
            "No se pudo guardar el hostname en {}: {}",
            hostname_path.display(),
            e
        ))
    })?;

    Ok(tor_dir)
}

/// Guarda la clave privada en formato binario de Tor
/// 
/// Formato correcto de Tor (96 bytes total):
/// - Bytes 0-31: "== ed25519v1-secret: type0 =="
/// - Bytes 32-34: 0x00 0x00 0x00
/// - Bytes 35-98: 64 bytes de clave expandida (SHA-512 de la clave privada)
/// 
/// # Arguments
/// 
/// * `private_key` - Clave privada de 32 bytes
/// * `tor_dir` - Directorio donde guardar el archivo
fn save_tor_secret_key(private_key: &[u8; 32], tor_dir: &Path) -> Result<()> {
    let secret_key_path = tor_dir.join("hs_ed25519_secret_key");
    
    // Expandir la clave privada usando SHA-512 (estándar, no SHA3)
    // Tor usa la "expanded" secret key que es el hash SHA-512 de la clave privada
    use sha2::{Sha512, Digest};
    let mut hasher = Sha512::new();
    hasher.update(private_key);
    let mut expanded_key = hasher.finalize();
    
    // Aplicar clamping según especificación Ed25519
    expanded_key[0] &= 248;
    expanded_key[31] &= 127;
    expanded_key[31] |= 64;
    
    // Crear el archivo en formato binario de Tor
    let mut content = Vec::new();
    
    // Header: "== ed25519v1-secret: type0 ==" (32 bytes exactos)
    content.extend_from_slice(b"== ed25519v1-secret: type0 ==");
    
    // 3 bytes null
    content.extend_from_slice(&[0u8; 3]);
    
    // 64 bytes de clave expandida
    content.extend_from_slice(&expanded_key[..]);
    
    fs::write(&secret_key_path, content).map_err(|e| {
        VanityError::KeyStorage(format!(
            "No se pudo guardar hs_ed25519_secret_key en {}: {}",
            secret_key_path.display(),
            e
        ))
    })?;
    
    // Establecer permisos restrictivos en Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&secret_key_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&secret_key_path, perms)?;
    }
    
    Ok(())
}

/// Guarda el hostname en formato Tor
/// 
/// # Arguments
/// 
/// * `address` - Dirección .onion completa
/// * `tor_dir` - Directorio donde guardar el archivo
fn save_tor_hostname(address: &str, tor_dir: &Path) -> Result<()> {
    let hostname_path = tor_dir.join("hostname");
    
    fs::write(&hostname_path, address).map_err(|e| {
        VanityError::KeyStorage(format!(
            "No se pudo guardar hostname en {}: {}",
            hostname_path.display(),
            e
        ))
    })?;
    
    Ok(())
}

/// Crea un archivo README en el directorio de salida con instrucciones
/// 
/// # Arguments
/// 
/// * `output_dir` - Directorio donde crear el README
pub fn create_readme(output_dir: &Path) -> Result<()> {
    let readme_path = output_dir.join("README.txt");
    
    // Solo crear si no existe
    if readme_path.exists() {
        return Ok(());
    }

    let content = r#"VANITY ONION V3 - ARCHIVOS GENERADOS
=====================================

Este directorio contiene las claves y metadatos de las direcciones .onion v3 generadas.

ARCHIVOS POR CADA DIRECCIÓN:
----------------------------

1. <address>_private.key
   - Clave privada Ed25519 en formato hexadecimal (64 caracteres)
   - MANTENER SEGURA Y PRIVADA
   - Necesaria para configurar el servicio oculto de Tor

2. <address>_metadata.json
   - Metadatos de la generación (fecha, prefijo, clave pública, etc.)
   - Información de referencia

3. <address>_hostname.txt
   - Solo la dirección .onion completa
   - Útil para copiar y pegar

CÓMO USAR CON TOR:
------------------

Cada dirección generada incluye una carpeta <address>_tor/ con archivos listos para Tor:

- hs_ed25519_secret_key (formato binario de Tor)
- hostname (dirección .onion)

Para usar en Linux:

1. Copiar los archivos al directorio de servicio oculto:
   sudo cp <address>_tor/* /var/lib/tor/hidden_service/
   sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
   sudo chmod 700 /var/lib/tor/hidden_service/
   sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key

2. Configurar torrc:
   HiddenServiceDir /var/lib/tor/hidden_service/
   HiddenServicePort 80 127.0.0.1:8080

3. Reiniciar Tor:
   sudo systemctl restart tor

Para usar en Windows:

1. Copiar los archivos a: C:\Users\<usuario>\AppData\Roaming\tor\hidden_service\
2. Configurar torrc y reiniciar Tor

SEGURIDAD:
----------

- NUNCA compartas la clave privada
- Haz backup de estos archivos en un lugar seguro
- Considera cifrar este directorio
- Las claves privadas dan control total sobre la dirección .onion

Para más información sobre servicios ocultos de Tor:
https://community.torproject.org/onion-services/
"#;

    fs::write(&readme_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::TempDir;

    #[test]
    fn test_save_result() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        let result = VanityResult {
            address: "test1234567890abcdefghijklmnopqrstuvwxyz234567abcdefgh.onion".to_string(),
            matched_prefix: "test".to_string(),
            private_key: [1u8; 32],
            public_key: [2u8; 32],
            timestamp: Utc::now(),
        };

        let saved_path = save_result(&result, output_dir, 8).unwrap();
        
        // Verificar que se crearon los archivos
        assert!(saved_path.exists());
        assert!(output_dir.join("test1234567890abcdefghijklmnopqrstuvwxyz234567abcdefgh_metadata.json").exists());
        assert!(output_dir.join("test1234567890abcdefghijklmnopqrstuvwxyz234567abcdefgh_hostname.txt").exists());

        // Verificar contenido de la clave privada
        let private_key_content = fs::read_to_string(&saved_path).unwrap();
        assert_eq!(private_key_content.len(), 64); // 32 bytes en hex = 64 caracteres
    }

    #[test]
    fn test_create_readme() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        create_readme(output_dir).unwrap();
        
        let readme_path = output_dir.join("README.txt");
        assert!(readme_path.exists());

        let content = fs::read_to_string(&readme_path).unwrap();
        assert!(content.contains("VANITY ONION V3"));
        assert!(content.contains("SEGURIDAD"));
    }
}
