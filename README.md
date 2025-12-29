# anonymize

Deterministic text anonymization engine written in Rust with web interface.

## 🚀 Features

- **Deterministic**: Same input always produces same output
- **Auditable**: Complete traceability of all replacements
- **Offline**: Zero external dependencies, works air-gapped
- **Conservative**: When in doubt, leaves data untouched

## 🔍 Detection Capabilities

### Personal Data
- Emails, phone numbers (ES/EN/UK)
- IBANs, credit cards
- National IDs (Spanish DNI/NIE, US SSN)

### Corporate/Industrial Data
- Project codes, contract numbers
- Work orders, purchase orders
- Serial numbers, cost centers

## 🏗️ Architecture

```
┌──────────────────┐
│   Web Browser    │
│   (HTMX)         │
└────────┬─────────┘
         │ HTTP POST /api/anonymize
         ▼
┌──────────────────┐
│   Axum Server    │
│   (Port 3000)    │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Anonymize Core  │
│  Engine (Rust)   │
└──────────────────┘
```

## 🛠️ Local Development

### Prerequisites
- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Git

### Run Locally

```bash
# Clone repository
git clone https://github.com/yourusername/anonymize.git
cd anonymize

# Build and run
cargo run --release

# Server starts on http://localhost:3000
```

### Environment Variables

```bash
# Port (default: 3000)
PORT=8080 cargo run
```

## 📦 Production Deployment (Hetzner VPS)

### Manual Deployment

```bash
# 1. Compile on your local machine
cargo build --release

# 2. Copy binary to server
scp target/release/anonymize user@your-server.com:~/

# 3. SSH into server
ssh user@your-server.com

# 4. Create systemd service
sudo nano /etc/systemd/system/anonymize.service
```

**anonymize.service**:
```ini
[Unit]
Description=Anonymize Web Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/anonymize
ExecStart=/opt/anonymize/anonymize
Restart=on-failure
Environment="PORT=3000"

[Install]
WantedBy=multi-user.target
```

```bash
# 5. Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable anonymize
sudo systemctl start anonymize
sudo systemctl status anonymize
```

### Nginx Reverse Proxy

```bash
sudo nano /etc/nginx/sites-available/anonymize
```

**nginx config**:
```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/anonymize /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### HTTPS with Let's Encrypt

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal is configured automatically
```

## 🤖 Automated Deployment Script

Use `deploy.sh` for automated deployment:

```bash
./deploy.sh user@your-server.com
```

## 🧪 Testing

```bash
# Run tests
cargo test

# Test with sample data
echo "Contact: juan@empresa.com, DNI: 12345678Z" | cargo run
```

## 📊 API Documentation

### POST /api/anonymize

**Request:**
```json
{
  "text": "Mi email es juan@empresa.com y mi DNI 12345678Z"
}
```

**Response:**
```json
{
  "anonymized_text": "Mi email es [EMAIL_1] y mi DNI [NATIONAL_ID_1]",
  "audit_report": {
    "version": "0.1.0",
    "timestamp": "2025-01-15T10:30:00Z",
    "statistics": {
      "total_matches": 2,
      "processing_time_ms": 5
    },
    "replacements": [...]
  },
  "hash": "abc123..."
}
```

## 🔧 Configuration

Currently uses default configuration. Future versions will support `anonymize.toml`:

```toml
[general]
strict_mode = true
max_input_size = 104857600  # 100 MB

[detection]
locale = "es+en"
parallel = true

[replacement]
strategy = "sequential"
template = "[{category}_{n}]"
```

## 📝 License

TBD

## 🤝 Contributing

Contributions are welcome! Please ensure:
- Code follows Rust best practices
- All tests pass (`cargo test`)
- Changes don't break determinism
- Core engine remains unchanged for API stability

## 🐛 Troubleshooting

### Server won't start
```bash
# Check if port is in use
sudo lsof -i :3000

# Check logs
journalctl -u anonymize -f
```

### Build fails
```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build
```

## 📚 Architecture Details

See [ARCHITECTURE.md](./ARCHITECTURE.md) for technical specification.

## 🔐 Security Notes

- Audit reports in "full" mode contain original sensitive data
- Always use HTTPS in production
- Configure appropriate firewall rules
- Consider rate limiting for public deployments

---

Made with ❤️ and Rust
