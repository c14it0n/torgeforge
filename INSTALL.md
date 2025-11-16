# 📦 Guía de Instalación - Torge Forge

## 📋 Requisitos del Sistema

### Mínimos
- **CPU:** 2 núcleos
- **RAM:** 512 MB
- **Disco:** 50 MB

### Recomendados
- **CPU:** 4+ núcleos (mejor rendimiento)
- **RAM:** 2 GB
- **Disco:** 100 MB

---

## 🐧 Linux

### Debian / Ubuntu / Kali

```bash
# 1. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Instalar dependencias del sistema
sudo apt update
sudo apt install build-essential git -y

# 3. Clonar repositorio
git clone https://github.com/c14it0n/torgeforge
cd torgeforge

# 4. Compilar
cargo build --release

# 5. Ejecutar
./target/release/torge-forge
```

### Fedora / RHEL / CentOS

```bash
# 1. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Instalar dependencias
sudo dnf groupinstall "Development Tools" -y
sudo dnf install git -y

# 3. Clonar y compilar
git clone https://github.com/c14it0n/torgeforge
cd torgeforge
cargo build --release

# 4. Ejecutar
./target/release/torge-forge
```

### Arch Linux

```bash
# 1. Instalar Rust y dependencias
sudo pacman -S rust git base-devel

# 2. Clonar y compilar
git clone https://github.com/c14it0n/torgeforge
cd torgeforge
cargo build --release

# 3. Ejecutar
./target/release/torge-forge
```

---

## 🪟 Windows

### Opción 1: Instalación Completa

```powershell
# 1. Instalar Rust
# Descargar de: https://rustup.rs/
# Ejecutar rustup-init.exe

# 2. Instalar Visual Studio Build Tools
# Descargar de: https://visualstudio.microsoft.com/downloads/
# Seleccionar: "Desktop development with C++"

# 3. Instalar Git
# Descargar de: https://git-scm.com/download/win

# 4. Abrir PowerShell y clonar
git clone https://github.com/c14it0n/torgeforge
cd torgeforge

# 5. Compilar
cargo build --release

# 6. Ejecutar
.\target\release\torge-forge.exe
```

### Opción 2: WSL (Windows Subsystem for Linux)

```powershell
# 1. Instalar WSL
wsl --install

# 2. Reiniciar Windows

# 3. Abrir Ubuntu desde el menú inicio

# 4. Seguir instrucciones de Linux (Debian/Ubuntu)
```

---

## 🍎 macOS

```bash
# 1. Instalar Homebrew (si no lo tienes)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Instalar Git (si no lo tienes)
brew install git

# 4. Clonar y compilar
git clone https://github.com/c14it0n/torgeforge
cd torgeforge
cargo build --release

# 5. Ejecutar
./target/release/torge-forge
```

---

## 🚀 Instalación Global

Para poder ejecutar `torge-forge` desde cualquier ubicación:

### Linux / macOS

```bash
# Opción 1: Usar cargo install
cd torge-forge
cargo install --path .

# Ahora puedes ejecutar desde cualquier lugar
torge-forge

# Opción 2: Copiar binario manualmente
sudo cp target/release/torge-forge /usr/local/bin/
sudo chmod +x /usr/local/bin/torge-forge
```

### Windows

```powershell
# Agregar al PATH
$env:Path += ";C:\ruta\a\torge-forge\target\release"

# O copiar a una ubicación en el PATH
Copy-Item target\release\torge-forge.exe C:\Windows\System32\
```

---

## ✅ Verificar Instalación

```bash
# Verificar versión de Rust
rustc --version
# Debe mostrar: rustc 1.70.0 o superior

# Verificar que Torge Forge funciona
./target/release/torge-forge --version

# Ejecutar test rápido
./target/release/torge-forge-cli test --max-results 1
```

---

## 🔄 Actualizar

```bash
# Ir al directorio del proyecto
cd torge-forge

# Obtener últimos cambios
git pull

# Recompilar
cargo build --release
```

---

## 🗑️ Desinstalar

```bash
# Si instalaste globalmente
cargo uninstall torge-forge

# O eliminar binario manual
sudo rm /usr/local/bin/torge-forge

# Eliminar directorio del proyecto
cd ..
rm -rf torge-forge
```

---

## 🐛 Problemas de Instalación

### Error: "rustc not found"

**Solución:**
```bash
# Recargar el PATH
source $HOME/.cargo/env

# O cerrar y abrir nueva terminal
```

### Error: "linker 'cc' not found"

**Solución (Linux):**
```bash
sudo apt install build-essential
```

**Solución (Windows):**
```
Instalar Visual Studio Build Tools
```

### Error: "failed to compile"

**Solución:**
```bash
# Actualizar Rust
rustup update

# Limpiar y recompilar
cargo clean
cargo build --release
```

### Error: Compilación muy lenta

**Solución:**
```bash
# Usar menos paralelismo durante compilación
cargo build --release -j 2
```

---

## 📚 Siguientes Pasos

Después de instalar:

1. Lee el [README.md](README.md) para uso básico
2. Revisa [EXAMPLES.md](EXAMPLES.md) para ejemplos prácticos
3. Consulta [TROUBLESHOOTING.md](TROUBLESHOOTING.md) si tienes problemas

---

**Torge Forge** - Instalación completada 🔥
