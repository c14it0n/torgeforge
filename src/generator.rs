use data_encoding::BASE32_NOPAD;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use sha3::{Digest, Sha3_256};

/// Versión del protocolo onion v3
const ONION_V3_VERSION: u8 = 0x03;

/// Prefijo para el cálculo del checksum según especificación Tor
const CHECKSUM_PREFIX: &[u8] = b".onion checksum";

/// Sufijo para el cálculo del checksum
const CHECKSUM_SUFFIX: &[u8] = b"\x03";

/// Genera un par de claves Ed25519 aleatorio
/// 
/// Utiliza OsRng que es un generador de números aleatorios
/// criptográficamente seguro proporcionado por el sistema operativo.
/// 
/// # Returns
/// 
/// Tupla con (clave_privada, clave_pública) como arrays de 32 bytes
pub fn generate_keypair() -> ([u8; 32], [u8; 32]) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    
    let private_key = signing_key.to_bytes();
    let public_key = verifying_key.to_bytes();
    
    (private_key, public_key)
}

/// Calcula el checksum para una dirección .onion v3
/// 
/// Según la especificación de Tor:
/// CHECKSUM = primeros 2 bytes de SHA3-256(".onion checksum" || PUBKEY || VERSION || "\x03")
/// 
/// # Arguments
/// 
/// * `public_key` - Clave pública Ed25519 (32 bytes)
/// 
/// # Returns
/// 
/// Array de 2 bytes con el checksum
pub fn calculate_checksum(public_key: &[u8; 32]) -> [u8; 2] {
    let mut hasher = Sha3_256::new();
    
    // Construir el mensaje: ".onion checksum" || PUBKEY || VERSION || "\x03"
    hasher.update(CHECKSUM_PREFIX);
    hasher.update(public_key);
    hasher.update(&[ONION_V3_VERSION]);
    hasher.update(CHECKSUM_SUFFIX);
    
    let hash = hasher.finalize();
    
    // Tomar los primeros 2 bytes
    [hash[0], hash[1]]
}

/// Genera una dirección .onion v3 a partir de una clave pública
/// 
/// Formato de dirección v3:
/// onion_address = base32(PUBKEY || CHECKSUM || VERSION) + ".onion"
/// 
/// Donde:
/// - PUBKEY: 32 bytes (clave pública Ed25519)
/// - CHECKSUM: 2 bytes (calculado con SHA3-256)
/// - VERSION: 1 byte (0x03 para v3)
/// 
/// # Arguments
/// 
/// * `public_key` - Clave pública Ed25519 (32 bytes)
/// 
/// # Returns
/// 
/// String con la dirección .onion completa (56 caracteres + ".onion")
pub fn generate_onion_address(public_key: &[u8; 32]) -> String {
    let checksum = calculate_checksum(public_key);
    
    // Construir el payload: PUBKEY (32) || CHECKSUM (2) || VERSION (1) = 35 bytes
    let mut payload = Vec::with_capacity(35);
    payload.extend_from_slice(public_key);
    payload.extend_from_slice(&checksum);
    payload.push(ONION_V3_VERSION);
    
    // Codificar en base32 (sin padding) y convertir a minúsculas
    let encoded = BASE32_NOPAD.encode(&payload).to_lowercase();
    
    // Agregar el sufijo .onion
    format!("{}.onion", encoded)
}

/// Verifica si una dirección .onion cumple con alguno de los prefijos dados
/// 
/// # Arguments
/// 
/// * `address` - Dirección .onion completa
/// * `prefixes` - Lista de prefijos a buscar (en minúsculas)
/// 
/// # Returns
/// 
/// Option con el prefijo que coincidió, o None si no hay coincidencia
pub fn matches_prefix(address: &str, prefixes: &[String]) -> Option<String> {
    // Remover el sufijo .onion para comparar solo la parte base32
    let address_without_suffix = address.strip_suffix(".onion").unwrap_or(address);
    
    for prefix in prefixes {
        if address_without_suffix.starts_with(prefix) {
            return Some(prefix.clone());
        }
    }
    
    None
}

/// Valida que una dirección .onion v3 tenga el formato correcto
/// 
/// # Arguments
/// 
/// * `address` - Dirección a validar
/// 
/// # Returns
/// 
/// true si la dirección tiene el formato correcto
pub fn validate_onion_address(address: &str) -> bool {
    // Debe terminar en .onion
    if !address.ends_with(".onion") {
        return false;
    }
    
    // Remover .onion y verificar longitud
    let base = address.strip_suffix(".onion").unwrap();
    
    // Una dirección v3 codificada en base32 tiene 56 caracteres
    // (35 bytes * 8 bits / 5 bits por carácter base32 = 56)
    if base.len() != 56 {
        return false;
    }
    
    // Verificar que todos los caracteres sean válidos en base32
    base.chars().all(|c| c.is_ascii_lowercase() || ('2'..='7').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (priv1, pub1) = generate_keypair();
        let (priv2, pub2) = generate_keypair();
        
        // Las claves deben ser diferentes
        assert_ne!(priv1, priv2);
        assert_ne!(pub1, pub2);
        
        // Las claves deben tener 32 bytes
        assert_eq!(priv1.len(), 32);
        assert_eq!(pub1.len(), 32);
    }

    #[test]
    fn test_calculate_checksum() {
        // Vector de prueba con una clave pública conocida
        let public_key = [0u8; 32];
        let checksum = calculate_checksum(&public_key);
        
        // El checksum debe tener 2 bytes
        assert_eq!(checksum.len(), 2);
        
        // El checksum debe ser determinístico
        let checksum2 = calculate_checksum(&public_key);
        assert_eq!(checksum, checksum2);
    }

    #[test]
    fn test_generate_onion_address() {
        let public_key = [0u8; 32];
        let address = generate_onion_address(&public_key);
        
        // Debe terminar en .onion
        assert!(address.ends_with(".onion"));
        
        // Debe tener la longitud correcta (56 + 6 = 62)
        assert_eq!(address.len(), 62);
        
        // Debe ser válida
        assert!(validate_onion_address(&address));
        
        // Debe ser determinística
        let address2 = generate_onion_address(&public_key);
        assert_eq!(address, address2);
    }

    #[test]
    fn test_matches_prefix() {
        let address = "test1234567890abcdefghijklmnopqrstuvwxyz234567abcdefgh.onion";
        let prefixes = vec!["test".to_string(), "hello".to_string()];
        
        let matched = matches_prefix(address, &prefixes);
        assert_eq!(matched, Some("test".to_string()));
        
        let prefixes_no_match = vec!["hello".to_string(), "world".to_string()];
        let matched = matches_prefix(address, &prefixes_no_match);
        assert_eq!(matched, None);
    }

    #[test]
    fn test_validate_onion_address() {
        // Dirección válida (longitud correcta)
        let valid = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3.onion";
        assert!(validate_onion_address(valid));
        
        // Dirección inválida (muy corta)
        let invalid_short = "short.onion";
        assert!(!validate_onion_address(invalid_short));
        
        // Dirección inválida (sin .onion)
        let invalid_no_suffix = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3";
        assert!(!validate_onion_address(invalid_no_suffix));
        
        // Dirección inválida (caracteres inválidos)
        let invalid_chars = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA3.onion";
        assert!(!validate_onion_address(invalid_chars));
    }

    #[test]
    fn test_onion_v3_format_integration() {
        // Test de integración: generar una clave y verificar que la dirección sea válida
        let (_, public_key) = generate_keypair();
        let address = generate_onion_address(&public_key);
        
        assert!(validate_onion_address(&address));
        assert_eq!(address.len(), 62); // 56 + ".onion"
    }
}
