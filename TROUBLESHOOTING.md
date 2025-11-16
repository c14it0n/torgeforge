# 🔧 Solución de Problemas - Torge Forge

## 🐛 Problemas Comunes

### 1. Tor Regenera la Clave

**Síntoma:**
```
Después de reiniciar Tor, el hostname cambia a una dirección diferente
```

**Diagnóstico:**
```bash
# Verificar tamaño del archivo (debe ser 96 bytes)
ls -l /var/lib/tor/hidden_service/hs_ed25519_secret_key

# Verificar header
xxd /var/lib/tor/hidden_service/hs_ed25519_secret_key | head -n 2
```

**Solución:**
```bash
# El archivo debe tener exactamente 96 bytes
# Header debe ser: "== ed25519v1-secret: type0 =="

# Si es incorrecto, asegúrate de usar Torge Forge v0.1.1 o superior
git pull
cargo build --release

# Regenerar dirección
./target/release/torge-forge-cli mysite

# Copiar archivos correctos
sudo cp output/mysite*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/
sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key
```

---

### 2. Servicio No Accesible

**Síntoma:**
```
"Connection refused" al visitar la dirección .onion
```

**Diagnóstico:**
```bash
# 1. Verificar que el servidor web está corriendo
curl http://127.0.0.1:80

# 2. Verificar configuración de torrc
sudo cat /etc/tor/torrc | grep HiddenService

# 3. Verificar estado de Tor
sudo systemctl status tor@default

# 4. Ver logs de Tor
sudo journalctl -u tor@default --since "10 minutes ago"
```

**Solución:**

#### Paso 1: Iniciar Servidor Web

```bash
# Opción A: Nginx
sudo apt install nginx -y
sudo systemctl start nginx
sudo systemctl enable nginx

# Opción B: Apache
sudo apt install apache2 -y
sudo systemctl start apache2
sudo systemctl enable apache2

# Opción C: Python (desarrollo)
cd /var/www/html
python3 -m http.server 80
```

#### Paso 2: Configurar torrc Correctamente

```bash
sudo nano /etc/tor/torrc

# Agregar (o verificar):
HiddenServiceDir /var/lib/tor/hidden_service/
HiddenServicePort 80 127.0.0.1:80
```

#### Paso 3: Reiniciar Tor

```bash
sudo systemctl restart tor@default
```

#### Paso 4: Esperar 5-10 Minutos

```bash
# Los servicios ocultos tardan en publicar su descriptor
sudo journalctl -u tor@default -f | grep -i "descriptor"

# Buscar: "Uploaded rendezvous descriptor"
```

---

### 3. Tor No Inicia (Kali Linux)

**Síntoma:**
```bash
$ sudo systemctl status tor
● tor.service - Anonymizing overlay network for TCP (multi-instance-master)
     Active: active (exited)
```

**Causa:**
Kali usa multi-instance mode. El servicio `tor` es solo un master vacío.

**Solución:**
```bash
# Detener master
sudo systemctl stop tor

# Iniciar instancia real
sudo systemctl start tor@default
sudo systemctl enable tor@default

# Verificar
sudo systemctl status tor@default

# Debe mostrar: active (running)
```

---

### 4. Permisos Incorrectos

**Síntoma:**
```
Tor logs: "Permission denied" o "Bad file descriptor"
```

**Diagnóstico:**
```bash
ls -la /var/lib/tor/hidden_service/
```

**Solución:**
```bash
# Corregir propietario
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/

# Corregir permisos del directorio
sudo chmod 700 /var/lib/tor/hidden_service/

# Corregir permisos de archivos
sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key
sudo chmod 644 /var/lib/tor/hidden_service/hostname

# Reiniciar Tor
sudo systemctl restart tor@default
```

---

### 5. Servicio Tarda Mucho en Estar Disponible

**Síntoma:**
```
El .onion no carga después de 5 minutos
```

**Diagnóstico:**
```bash
# Verificar que Tor publicó el descriptor
sudo journalctl -u tor@default | grep -i "upload"

# Verificar conectividad de Tor
sudo journalctl -u tor@default | grep -i "bootstrap"
```

**Solución:**
```bash
# 1. Verificar que Tor completó bootstrap
sudo journalctl -u tor@default | grep "Bootstrapped 100%"

# 2. Esperar mensaje de descriptor
sudo journalctl -u tor@default -f | grep "Uploaded rendezvous descriptor"

# 3. Si no aparece después de 10 minutos, reiniciar
sudo systemctl restart tor@default

# 4. Verificar que no hay errores
sudo journalctl -u tor@default --since "5 minutes ago" | grep -i error
```

---

### 6. Error de Compilación

**Síntoma:**
```
error: linker 'cc' not found
```

**Solución (Linux):**
```bash
# Debian/Ubuntu
sudo apt install build-essential -y

# Fedora/RHEL
sudo dnf groupinstall "Development Tools" -y

# Arch
sudo pacman -S base-devel
```

**Solución (Windows):**
```
1. Descargar Visual Studio Build Tools
2. Instalar "Desktop development with C++"
3. Reiniciar terminal
4. Intentar compilar de nuevo
```

---

### 7. Rust No Encontrado

**Síntoma:**
```bash
$ cargo build
cargo: command not found
```

**Solución:**
```bash
# Recargar PATH
source $HOME/.cargo/env

# Si no funciona, reinstalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar
rustc --version
cargo --version
```

---

### 8. Puerto Ya en Uso

**Síntoma:**
```
Error: Address already in use (os error 98)
```

**Diagnóstico:**
```bash
# Ver qué está usando el puerto 80
sudo netstat -tlnp | grep :80
# o
sudo lsof -i :80
```

**Solución:**
```bash
# Opción 1: Detener el servicio que usa el puerto
sudo systemctl stop apache2
# o
sudo systemctl stop nginx

# Opción 2: Usar otro puerto
# Editar torrc:
HiddenServicePort 80 127.0.0.1:8080

# Iniciar servidor en puerto 8080
python3 -m http.server 8080
```

---

### 9. Dirección .onion Inválida

**Síntoma:**
```
Tor Browser: "Invalid Onion Site Address"
```

**Diagnóstico:**
```bash
# Verificar hostname generado
sudo cat /var/lib/tor/hidden_service/hostname

# Debe tener 56 caracteres + .onion
# Ejemplo: abc123...xyz.onion (total 62 caracteres)
```

**Solución:**
```bash
# Si el hostname es incorrecto, regenerar
sudo rm /var/lib/tor/hidden_service/hostname
sudo systemctl restart tor@default

# Esperar 1 minuto
sudo cat /var/lib/tor/hidden_service/hostname
```

---

### 10. Firewall Bloqueando

**Síntoma:**
```
Servidor web funciona localmente pero .onion no carga
```

**Diagnóstico:**
```bash
# Verificar firewall
sudo ufw status
# o
sudo firewalld-cmd --list-all
```

**Solución:**
```bash
# UFW (Ubuntu/Debian)
sudo ufw allow 80/tcp
sudo ufw reload

# Firewalld (Fedora/RHEL)
sudo firewall-cmd --add-service=http --permanent
sudo firewall-cmd --reload

# iptables
sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT
sudo iptables-save
```

---

## 🔍 Comandos de Diagnóstico

### Script de Diagnóstico Completo

```bash
#!/bin/bash
# diagnostico_tor.sh

echo "=== DIAGNÓSTICO TORGE FORGE ==="
echo ""

echo "1. Verificando archivos de Tor..."
ls -lh /var/lib/tor/hidden_service/
echo ""

echo "2. Verificando tamaño de clave secreta..."
stat -c%s /var/lib/tor/hidden_service/hs_ed25519_secret_key
echo "   (Debe ser: 96 bytes)"
echo ""

echo "3. Verificando header de clave..."
xxd /var/lib/tor/hidden_service/hs_ed25519_secret_key | head -n 2
echo "   (Debe mostrar: == ed25519v1-secret: type0 ==)"
echo ""

echo "4. Verificando hostname..."
cat /var/lib/tor/hidden_service/hostname
echo ""

echo "5. Verificando configuración torrc..."
grep -A 2 HiddenService /etc/tor/torrc | grep -v "^#"
echo ""

echo "6. Verificando estado de Tor..."
systemctl status tor@default --no-pager | head -n 10
echo ""

echo "7. Verificando servidor web local..."
curl -s -o /dev/null -w "HTTP Status: %{http_code}\n" http://127.0.0.1:80
echo ""

echo "8. Verificando puerto 80..."
sudo netstat -tlnp | grep :80
echo ""

echo "9. Últimos logs de Tor..."
sudo journalctl -u tor@default --since "5 minutes ago" --no-pager | tail -n 20
echo ""

echo "=== FIN DIAGNÓSTICO ==="
```

Ejecutar:
```bash
chmod +x diagnostico_tor.sh
./diagnostico_tor.sh
```

---

## 📋 Checklist de Verificación

Antes de reportar un problema, verifica:

- [ ] Torge Forge versión 0.1.1 o superior
- [ ] Archivo `hs_ed25519_secret_key` tiene 96 bytes
- [ ] Header del archivo es correcto
- [ ] Permisos correctos (700 directorio, 600 archivo)
- [ ] Propietario es `debian-tor:debian-tor`
- [ ] Servidor web corriendo en puerto configurado
- [ ] `curl http://127.0.0.1:80` funciona
- [ ] torrc configurado correctamente
- [ ] Tor está corriendo (`systemctl status tor@default`)
- [ ] Esperado al menos 5-10 minutos
- [ ] Logs no muestran errores
- [ ] Probado en Tor Browser (no navegador normal)

---

## 🆘 Obtener Ayuda

Si ninguna solución funciona:

1. **Ejecutar script de diagnóstico** (ver arriba)
2. **Revisar logs completos:**
   ```bash
   sudo journalctl -u tor@default --since "1 hour ago" > tor_logs.txt
   ```
3. **Crear issue en GitHub** con:
   - Salida del script de diagnóstico
   - Logs de Tor
   - Sistema operativo y versión
   - Versión de Torge Forge

---

## 🔄 Reinicio Completo

Si todo falla, reinicio desde cero:

```bash
# 1. Detener Tor
sudo systemctl stop tor@default

# 2. Limpiar directorio
sudo rm -rf /var/lib/tor/hidden_service/*

# 3. Regenerar con Torge Forge
./target/release/torge-forge-cli mysite

# 4. Copiar archivos
sudo cp output/mysite*_tor/* /var/lib/tor/hidden_service/

# 5. Corregir permisos
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/
sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key

# 6. Verificar torrc
sudo nano /etc/tor/torrc
# HiddenServiceDir /var/lib/tor/hidden_service/
# HiddenServicePort 80 127.0.0.1:80

# 7. Reiniciar Tor
sudo systemctl restart tor@default

# 8. Monitorear logs
sudo journalctl -u tor@default -f
```

---

**Torge Forge** - Solución de problemas 🔥
