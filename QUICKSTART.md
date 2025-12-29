# Quick Start Guide

## 5-Minute Setup

### Local Development

```bash
# 1. Clone and enter directory
git clone https://github.com/yourusername/anonymize.git
cd anonymize

# 2. Run (compiles automatically)
cargo run --release

# 3. Open browser
# Navigate to: http://localhost:3000
```

That's it! The web interface will open at `http://localhost:3000`.

### Test with cURL

```bash
curl -X POST http://localhost:3000/api/anonymize \
  -H "Content-Type: application/json" \
  -d '{"text":"Mi email es juan@empresa.com y mi DNI 12345678Z"}'
```

### Test with Sample File

```bash
cat test_input.txt | cargo run
```

## Production Deployment (15 minutes)

### Prerequisites on Server
- Ubuntu 20.04+ or Debian 11+
- Nginx installed
- SSH access

### One-Command Deploy

```bash
# From your local machine
./deploy.sh user@your-server.com
```

The script will:
1. ✅ Compile the binary
2. ✅ Copy to server
3. ✅ Set up systemd service
4. ✅ Configure Nginx
5. ✅ Start the service

### Add HTTPS (5 more minutes)

```bash
ssh user@your-server.com
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

## Common Commands

```bash
# Development
make dev              # Run in dev mode
make test             # Run tests

# Production
make build            # Build release binary
make deploy SERVER=user@host  # Deploy

# Code quality
make fmt              # Format code
make check            # Run linter
make pre-commit       # All checks
```

## Environment Variables

```bash
# Change port (default: 3000)
PORT=8080 cargo run

# In production (systemd)
# Edit /etc/systemd/system/anonymize.service
Environment="PORT=8080"
```

## Troubleshooting

### Port already in use
```bash
# Find what's using port 3000
sudo lsof -i :3000

# Kill process
kill -9 <PID>
```

### Server won't start
```bash
# Check logs
sudo journalctl -u anonymize -f

# Restart
sudo systemctl restart anonymize
```

### Build fails
```bash
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo build --release
```

## What Gets Detected?

✅ **Personal Data:**
- Emails: `juan@empresa.com` → `[EMAIL_1]`
- Phones: `+34 666 123 456` → `[PHONE_1]`
- DNI/NIE: `12345678Z` → `[NATIONAL_ID_1]`
- IBAN: `ES91 2100 0418...` → `[IBAN_1]`
- Credit Cards: `4532-1234-5678-9010` → `[CREDIT_CARD_1]`

✅ **Corporate Data:**
- Projects: `PRJ-2024-001` → `[PROJECT_CODE_1]`
- Contracts: `CTR-2024-1234` → `[CONTRACT_NUMBER_1]`
- Work Orders: `OT-2024-1234` → `[WORK_ORDER_1]`

## Next Steps

1. Read [README.md](./README.md) for full documentation
2. Check [ARCHITECTURE.md](./ARCHITECTURE.md) for technical details
3. Star the repo if you find it useful! ⭐

## Need Help?

- 🐛 Issues: https://github.com/yourusername/anonymize/issues
- 📧 Email: your@email.com
- 💬 Discussions: https://github.com/yourusername/anonymize/discussions

---

**Pro Tip:** Use `make help` to see all available commands!
