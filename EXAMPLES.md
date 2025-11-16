# 📚 Ejemplos de Uso - Torge Forge

## 🎯 Ejemplos Básicos

### Ejemplo 1: Primera Dirección

```bash
# Ejecutar Torge Forge
./target/release/torge-forge

# Menú interactivo
> 1  # Generar dirección
> test  # Prefijo deseado
> [Enter]  # Usar defaults (8 threads, 1 resultado)
> s  # Confirmar

# Resultado
✅ RESULTADO ENCONTRADO
🎯 Prefijo: test
🧅 Dirección: testxxx...onion
💾 Guardado en: output/testxxx_tor/
```

### Ejemplo 2: Modo CLI Rápido

```bash
# Generar con un comando
./target/release/torge-forge-cli hello

# Resultado
Found: helloxxx...onion
Saved to: output/helloxxx_tor/
```

### Ejemplo 3: Múltiples Prefijos

```bash
# Buscar cualquiera de estos prefijos
./target/release/torge-forge-cli cat dog bird

# Encuentra el primero que aparezca
Found: catxxx...onion
```

---

## ⚙️ Ejemplos Avanzados

### Ejemplo 4: Configuración Personalizada

```bash
# Usar 16 threads, generar 3 resultados
./target/release/torge-forge-cli test \
  --threads 16 \
  --max-results 3 \
  --output-dir ./my_onions

# Resultado
Found 3 addresses:
- testxxx1...onion
- testxxx2...onion
- testxxx3...onion
```

### Ejemplo 5: Limitar Intentos

```bash
# Buscar por máximo 1 millón de intentos
./target/release/torge-forge-cli hello \
  --max-attempts 1000000

# Si no encuentra, se detiene
No results found after 1,000,000 attempts
```

### Ejemplo 6: Dry Run (Prueba)

```bash
# Generar sin guardar archivos
./target/release/torge-forge-cli test --dry-run

# Muestra resultado pero no guarda
Found: testxxx...onion
(Dry run - not saved)
```

---

## 🌐 Configuración con Tor

### Ejemplo 7: Servicio Oculto con Nginx (Linux)

```bash
# 1. Generar dirección
./target/release/torge-forge
> 1
> mysite
> [Enter]
> s

# 2. Instalar y configurar Nginx
sudo apt install nginx -y
sudo systemctl start nginx

# 3. Copiar claves a Tor
sudo cp output/mysite*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/
sudo chmod 600 /var/lib/tor/hidden_service/hs_ed25519_secret_key

# 4. Configurar torrc
sudo nano /etc/tor/torrc
# Agregar:
# HiddenServiceDir /var/lib/tor/hidden_service/
# HiddenServicePort 80 127.0.0.1:80

# 5. Reiniciar Tor
sudo systemctl restart tor@default

# 6. Esperar 5-10 minutos y visitar
# mysitexxx...onion en Tor Browser
```

### Ejemplo 8: Servicio Oculto con Apache (Linux)

```bash
# 1. Generar dirección
./target/release/torge-forge-cli blog

# 2. Instalar Apache
sudo apt install apache2 -y
sudo systemctl start apache2

# 3. Configurar Tor
sudo cp output/blog*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/

# 4. Editar torrc
echo "HiddenServiceDir /var/lib/tor/hidden_service/" | sudo tee -a /etc/tor/torrc
echo "HiddenServicePort 80 127.0.0.1:80" | sudo tee -a /etc/tor/torrc

# 5. Reiniciar
sudo systemctl restart tor@default
```

### Ejemplo 9: Servidor Web Simple Python

```bash
# 1. Generar dirección
./target/release/torge-forge-cli demo

# 2. Crear sitio web
mkdir ~/demo_site
cd ~/demo_site
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Mi Servicio Oculto</title>
</head>
<body>
    <h1>¡Bienvenido a mi servicio oculto Tor!</h1>
    <p>Generado con Torge Forge</p>
</body>
</html>
EOF

# 3. Iniciar servidor
python3 -m http.server 8080 &

# 4. Configurar Tor
sudo cp ~/torge-forge/output/demo*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/

# 5. Configurar torrc para puerto 8080
sudo nano /etc/tor/torrc
# HiddenServiceDir /var/lib/tor/hidden_service/
# HiddenServicePort 80 127.0.0.1:8080

# 6. Reiniciar Tor
sudo systemctl restart tor@default
```

---

## 🔄 Ejemplos de Migración

### Ejemplo 10: Migrar Servicio Existente

```bash
# 1. Generar nueva dirección personalizada
./target/release/torge-forge-cli newsite

# 2. Detener Tor
sudo systemctl stop tor@default

# 3. Backup de claves antiguas
sudo cp -r /var/lib/tor/hidden_service /var/lib/tor/hidden_service.backup

# 4. Reemplazar con nuevas claves
sudo rm /var/lib/tor/hidden_service/hs_ed25519_*
sudo cp output/newsite*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/

# 5. Reiniciar Tor
sudo systemctl start tor@default

# 6. Verificar nueva dirección
sudo cat /var/lib/tor/hidden_service/hostname
```

---

## 📊 Ejemplos de Rendimiento

### Ejemplo 11: Benchmark de Velocidad

```bash
# Generar 10 direcciones con prefijo "a" (rápido)
time ./target/release/torge-forge-cli a --max-results 10

# Resultado típico:
# real    0m0.523s
# user    0m3.142s
# sys     0m0.089s
```

### Ejemplo 12: Prefijo Largo con Límite

```bash
# Intentar prefijo de 5 caracteres con límite
./target/release/torge-forge-cli hello \
  --threads 16 \
  --max-attempts 50000000

# Puede tardar 30-60 minutos
```

---

## 🛠️ Ejemplos de Scripting

### Ejemplo 13: Generar Múltiples Direcciones

```bash
#!/bin/bash
# generate_multiple.sh

PREFIXES=("cat" "dog" "bird" "fish")

for prefix in "${PREFIXES[@]}"; do
    echo "Generating: $prefix"
    ./target/release/torge-forge-cli "$prefix" --max-results 1
    echo "---"
done
```

### Ejemplo 14: Backup Automático

```bash
#!/bin/bash
# backup_keys.sh

OUTPUT_DIR="output"
BACKUP_DIR="$HOME/onion_backups/$(date +%Y%m%d_%H%M%S)"

mkdir -p "$BACKUP_DIR"
cp -r "$OUTPUT_DIR"/* "$BACKUP_DIR/"

echo "Backup guardado en: $BACKUP_DIR"

# Cifrar backup
tar -czf "$BACKUP_DIR.tar.gz" "$BACKUP_DIR"
gpg -c "$BACKUP_DIR.tar.gz"
rm -rf "$BACKUP_DIR" "$BACKUP_DIR.tar.gz"

echo "Backup cifrado: $BACKUP_DIR.tar.gz.gpg"
```

---

## 🔐 Ejemplos de Seguridad

### Ejemplo 15: Generar en Directorio Cifrado

```bash
# 1. Crear directorio cifrado
mkdir ~/secure_onions
cd ~/secure_onions

# 2. Generar dirección
~/torge-forge/target/release/torge-forge-cli secret \
  --output-dir ./keys

# 3. Cifrar directorio
tar -czf keys.tar.gz keys/
gpg -c keys.tar.gz
rm -rf keys/ keys.tar.gz

# 4. Desencriptar cuando necesites
gpg -d keys.tar.gz.gpg > keys.tar.gz
tar -xzf keys.tar.gz
```

---

## 🎨 Ejemplos de Prefijos Creativos

### Ejemplo 16: Prefijos Temáticos

```bash
# Tecnología
./target/release/torge-forge-cli tech code dev

# Negocios
./target/release/torge-forge-cli shop store buy

# Personal
./target/release/torge-forge-cli blog news info

# Servicios
./target/release/torge-forge-cli chat mail file
```

---

## 📱 Ejemplo Completo: Blog Personal

```bash
# 1. Generar dirección
./target/release/torge-forge-cli blog

# 2. Instalar Jekyll (generador de sitios estáticos)
sudo apt install ruby-full build-essential -y
gem install jekyll bundler

# 3. Crear blog
jekyll new myblog
cd myblog

# 4. Compilar sitio
bundle exec jekyll build

# 5. Servir con Nginx
sudo cp -r _site/* /var/www/html/
sudo systemctl restart nginx

# 6. Configurar Tor
sudo cp ~/torge-forge/output/blog*_tor/* /var/lib/tor/hidden_service/
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/
sudo chmod 700 /var/lib/tor/hidden_service/

# 7. Configurar torrc
echo "HiddenServiceDir /var/lib/tor/hidden_service/" | sudo tee -a /etc/tor/torrc
echo "HiddenServicePort 80 127.0.0.1:80" | sudo tee -a /etc/tor/torrc

# 8. Reiniciar Tor
sudo systemctl restart tor@default

# 9. Tu blog está en: blogxxx...onion
```

---

## 🎯 Tips y Trucos

### Usar Todos los Núcleos

```bash
# Detectar núcleos automáticamente
./target/release/torge-forge-cli test --threads $(nproc)
```

### Generar Mientras Duermes

```bash
# Prefijo largo que puede tardar horas
nohup ./target/release/torge-forge-cli hello --threads 16 > generation.log 2>&1 &

# Verificar progreso
tail -f generation.log
```

### Validar Antes de Generar

```bash
# Estimar tiempo primero
./target/release/torge-forge
> 2  # Estimar tiempo
> hello  # Prefijo
# Ve el tiempo estimado antes de decidir
```

---

**Torge Forge** - Ejemplos de uso 🔥
