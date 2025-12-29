#!/bin/bash
set -e

# Script de deploy automatizado para Hetzner VPS
# Uso: ./deploy.sh user@your-server.com

if [ -z "$1" ]; then
    echo "❌ Error: Especifica el servidor de destino"
    echo "Uso: ./deploy.sh user@your-server.com"
    exit 1
fi

SERVER=$1
REMOTE_USER=$(echo $SERVER | cut -d@ -f1)
REMOTE_HOST=$(echo $SERVER | cut -d@ -f2)

echo "🚀 Deploy Script - Anonymize Web"
echo "================================="
echo "Servidor: $SERVER"
echo ""

# 1. Build en local
echo "📦 1/6: Compilando binario..."
cargo build --release
echo "✅ Build completado"

# 2. Crear estructura en servidor
echo "📁 2/6: Creando estructura en servidor..."
ssh $SERVER << 'ENDSSH'
    sudo mkdir -p /opt/anonymize/src/web/static
    sudo chown -R $USER:$USER /opt/anonymize
ENDSSH
echo "✅ Estructura creada"

# 3. Copiar archivos
echo "📤 3/6: Copiando archivos al servidor..."
scp target/release/anonymize $SERVER:/opt/anonymize/
scp -r src/web/static/* $SERVER:/opt/anonymize/src/web/static/
echo "✅ Archivos copiados"

# 4. Configurar systemd service
echo "⚙️  4/6: Configurando systemd service..."
ssh $SERVER << 'ENDSSH'
    sudo tee /etc/systemd/system/anonymize.service > /dev/null << 'EOF'
[Unit]
Description=Anonymize Web Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/anonymize
ExecStart=/opt/anonymize/anonymize
Restart=on-failure
RestartSec=10
Environment="PORT=3000"

[Install]
WantedBy=multi-user.target
