# 🔥 Torge Forge

<div align="center">

**Generador de direcciones .onion v3 personalizadas para Tor**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-lightgrey.svg)]()

*Forja direcciones .onion personalizadas con prefijos únicos*

[Instalación](#-instalación) • [Uso](#-uso) • [Ejemplos](#-ejemplos) • [Solución de Problemas](#-solución-de-problemas)

</div>

---

## 📋 Descripción

**Torge Forge** es un generador de alto rendimiento de direcciones .onion v3 personalizadas (vanity addresses) para servicios ocultos de Tor. Utiliza criptografía Ed25519 y búsqueda paralela para encontrar direcciones que comiencen con tu prefijo deseado.

### ✨ Características

- 🎯 **Prefijos Personalizados** - Genera direcciones .onion que comiencen con tu texto
- ⚡ **Búsqueda Paralela** - Utiliza todos los núcleos de tu CPU con Rayon
- 🔐 **Criptografía Segura** - Ed25519 + SHA3-256 según especificación Tor v3
- 🎨 **Interfaz Interactiva** - Menú intuitivo con validación de entrada
- 📦 **Formato Tor Nativo** - Genera archivos listos para usar en Tor (96 bytes)
- 💾 **Múltiples Formatos** - Backup en hex + formato binario de Tor
- 🚀 **Alto Rendimiento** - Miles de intentos por segundo

### 🎯 Formato de Salida

Cada dirección generada incluye:

```
output/
├── <address>_private.key          # Clave privada en hex (backup)
├── <address>_metadata.json        # Metadatos de generación
├── <address>_hostname.txt         # Dirección .onion
└── <address>_tor/                 # ✨ Listo para Tor
    ├── hs_ed25519_secret_key      # Formato binario Tor (96 bytes)
    └── hostname                   # Dirección .onion
```

---

## 🚀 Instalación

### Requisitos Previos

- **Rust 1.70+** - [Instalar Rust](https://rustup.rs/)
- **Git** - Para clonar el repositorio

### Linux / macOS

```bash
# Clonar repositorio
git clone https://github.com/c14it0n/torgeforge
cd torgeforge

# Compilar
cargo build --release

# Ejecutar
./target/release/torge-forge
```

### Windows

```powershell
# Clonar repositorio
git clone https://github.com/c14it0n/torgeforge
cd torgeforge

# Compilar
cargo build --release

# Ejecutar
.\target\release\torge-forge.exe
```

### Instalación Global (Opcional)

```bash
cargo install --path .
torge-forge
```

---

## 💻 Uso

### Modo Interactivo (Recomendado)

```bash
./target/release/torge-forge
```

Menú interactivo:
```
╔═══════════════════════════════════════════════════════════════╗
║                        MENÚ PRINCIPAL                         ║
╠═══════════════════════════════════════════════════════════════╣
║  [1] 🎯 Generar dirección .onion vanity                       ║
║  [2] 📊 Estimar tiempo de búsqueda                            ║
║  [3] ⚙️  Configuración avanzada                                ║
║  [4] ℹ️  Información y ayuda                                   ║
║  [5] 🚪 Salir                                                 ║
╚═══════════════════════════════════════════════════════════════╝
```

### Modo CLI

```bash
# Generar dirección con prefijo "hello"
./target/release/torge-forge-cli hello

# Múltiples prefijos
./target/release/torge-forge-cli hello world test

# Opciones avanzadas
./target/release/torge-forge-cli hello \
  --threads 8 \
  --max-results 3 \
  --output-dir ./my_onions

# Ver todas las opciones
./target/release/torge-forge-cli --help
```

---

## 📚 Ejemplos

### Ejemplo 1: Generar Dirección Simple

```bash
./target/release/torge-forge
> 1  # Generar
> test  # Prefijo
> [Enter]  # Defaults
> s  # Confirmar
```

**Resultado:**
```
✅ RESULTADO ENCONTRADO
🎯 Prefijo: test
🧅 Dirección: testxxx...onion
💾 Guardado en: output/testxxx_tor/
```

### Ejemplo 2: Configurar en Tor (Linux)

```bash
# 1. Generar dirección
./target/release/torge-forge
# Seleccionar prefijo: mysite

# 2. Copiar archivos a Tor
sudo cp output/mysite*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/
sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key

# 3. Configurar torrc
sudo nano /etc/tor/torrc
# Agregar:
# HiddenServiceDir /var/lib/tor/hidden_service/
# HiddenServicePort 80 127.0.0.1:80

# 4. Reiniciar Tor
sudo systemctl restart tor@default

# 5. Verificar
sudo cat /var/lib/tor/hidden_service/hostname
```

### Ejemplo 3: Prefijos Más Largos

```bash
# Prefijo de 4 caracteres (más difícil)
./target/release/torge-forge-cli test --threads 16

# Prefijo de 5 caracteres (muy difícil)
./target/release/torge-forge-cli hello --threads 16 --max-attempts 10000000
```

### Ejemplo 4: Múltiples Resultados

```bash
# Generar 5 direcciones con prefijo "cat"
./target/release/torge-forge-cli cat --max-results 5
```

---

## 🔧 Solución de Problemas

### Problema: Tor Regenera la Clave

**Síntoma:** Después de reiniciar Tor, el hostname cambia.

**Causa:** Archivo `hs_ed25519_secret_key` incorrecto.

**Solución:**
```bash
# Verificar tamaño (debe ser 96 bytes)
ls -l /var/lib/tor/hidden_service/hs_ed25519_secret_key

# Verificar header
xxd /var/lib/tor/hidden_service/hs_ed25519_secret_key | head -n 2
# Debe mostrar: "== ed25519v1-secret: type0 =="

# Si es incorrecto, regenerar con Torge Forge v0.1.1+
```

### Problema: Servicio No Accesible

**Síntoma:** "Connection refused" al visitar .onion

**Causa:** No hay servidor web corriendo.

**Solución:**
```bash
# Verificar que tienes un servidor web en el puerto configurado
curl http://127.0.0.1:80

# Si falla, iniciar servidor web (ejemplo con Nginx)
sudo systemctl start nginx

# Verificar configuración de torrc
sudo cat /etc/tor/torrc | grep HiddenService
# Debe mostrar:
# HiddenServiceDir /var/lib/tor/hidden_service/
# HiddenServicePort 80 127.0.0.1:80
```

### Problema: Tor No Inicia (Kali Linux)

**Síntoma:** `systemctl status tor` muestra "exited"

**Solución:**
```bash
# Kali usa multi-instance mode
sudo systemctl stop tor
sudo systemctl start tor@default
sudo systemctl enable tor@default

# Verificar
sudo systemctl status tor@default
```

### Problema: Servicio Tarda en Estar Disponible

**Síntoma:** .onion no carga inmediatamente

**Solución:**
```bash
# Los servicios ocultos tardan 5-10 minutos en publicar su descriptor
# Verificar logs
sudo journalctl -u tor@default -f

# Buscar: "Uploaded rendezvous descriptor"
# Esperar este mensaje antes de probar
```

### Problema: Permisos Incorrectos

**Síntoma:** Tor muestra errores de permisos en logs

**Solución:**
```bash
# Corregir permisos
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/
sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key
sudo chmod 644 /var/lib/tor/hidden_service/hostname
```

### Problema: Compilación Falla

**Síntoma:** `cargo build` muestra errores

**Solución:**
```bash
# Actualizar Rust
rustup update

# Limpiar y recompilar
cargo clean
cargo build --release

# Verificar versión de Rust
rustc --version
# Debe ser 1.70 o superior
```

---

## 📊 Rendimiento

### Tiempos Estimados

| Prefijo | Longitud | Intentos Aprox. | Tiempo (8 cores) |
|---------|----------|-----------------|------------------|
| `a` | 1 char | 32 | < 1 segundo |
| `ab` | 2 chars | 1,024 | < 1 segundo |
| `abc` | 3 chars | 32,768 | 1-5 segundos |
| `abcd` | 4 chars | 1,048,576 | 30-60 segundos |
| `abcde` | 5 chars | 33,554,432 | 30-60 minutos |
| `abcdef` | 6 chars | 1,073,741,824 | 20-40 horas |

**Nota:** Los tiempos varían según el hardware. Más núcleos = más rápido.

### Optimización

```bash
# Usar todos los núcleos disponibles
./target/release/torge-forge-cli test --threads $(nproc)

# Limitar intentos para prefijos largos
./target/release/torge-forge-cli hello --max-attempts 50000000
```

---

## 🔐 Seguridad

### ⚠️ Advertencias Importantes

- **NUNCA** compartas tu clave privada (`hs_ed25519_secret_key`)
- Haz **backup** de tus claves en múltiples ubicaciones seguras
- Las claves dan **control total** sobre la dirección .onion
- Considera **cifrar** el directorio de salida

### Buenas Prácticas

```bash
# Cifrar directorio de salida
tar -czf onion_keys.tar.gz output/
gpg -c onion_keys.tar.gz
rm -rf output/ onion_keys.tar.gz

# Backup seguro
rsync -av output/ user@backup-server:/secure/location/
```

---

## 🛠️ Tecnologías

- **Rust 2021** - Lenguaje de programación
- **ed25519-dalek** - Criptografía Ed25519
- **sha3** - Checksums SHA3-256
- **sha2** - Expansión de clave SHA-512
- **rayon** - Paralelización
- **clap** - CLI parsing
- **data-encoding** - Base32 encoding

---

## 📄 Licencia

MIT License - Ver [LICENSE](LICENSE) para más detalles.

---

## 👤 Autor

**Nahum Deavila**

---

## 🌟 Agradecimientos

- [Tor Project](https://www.torproject.org/) - Por la especificación de servicios ocultos v3
- Comunidad Rust - Por las excelentes librerías criptográficas

---

<div align="center">

**Torge Forge** - Forjando direcciones .onion personalizadas 🔥

*Hecho con ❤️ en Rust*

</div>
