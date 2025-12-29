# 📄 Procesamiento de Documentos - NUEVA FUNCIONALIDAD

## ✨ ¿Qué hay de nuevo?

Ahora **anonymize** puede procesar documentos completos:

- ✅ **Upload DOCX/PDF** vía interfaz web
- ✅ **Descarga automática** del documento anonimizado
- ✅ **Mantiene formato** (lo mejor posible)
- ✅ **Sin perder texto plano** (la funcionalidad original sigue disponible)

---

## 🎯 Interfaz con Tabs

### Tab 1: **Texto** (original)
- Pega texto plano
- Anonimiza
- Ve resultados + JSON audit

### Tab 2: **Archivo** (nuevo)
- Sube .docx o .pdf
- Click "Anonimizar"
- Descarga automática del archivo anonimizado

---

## 🔧 Dependencies Añadidas

```toml
# Document processing
docx-rs = "0.4"
lopdf = "0.32"
pdf-extract = "0.7"
```

---

## 📦 Arquitectura

```
POST /api/anonymize-file
    ↓
Multipart file upload
    ↓
Detectar tipo (.docx o .pdf)
    ↓
Extraer texto
    ↓
Anonimizar con motor existente
    ↓
Reconstruir documento
    ↓
Devolver archivo para descarga
```

---

## 🚀 Uso

### Vía Web (recomendado)

1. Abre http://localhost:3000
2. Click en tab "**📄 Archivo**"
3. Sube tu .docx o .pdf
4. Click "Anonimizar"
5. Descarga automática del archivo anonimizado

### Vía cURL

```bash
curl -X POST http://localhost:3000/api/anonymize-file \
  -F "file=@documento.docx" \
  --output documento_anonymized.docx
```

---

## ⚠️ Limitaciones

### DOCX
- ✅ Texto plano preservado
- ✅ Estructura básica (párrafos)
- ⚠️ Tablas complejas simplificadas
- ⚠️ Estilos avanzados no preservados
- ⚠️ Imágenes eliminadas

### PDF
- ✅ Texto extraído y anonimizado
- ✅ PDF nuevo generado
- ⚠️ Formato muy básico (texto simple)
- ⚠️ Sin imágenes, sin tablas complejas
- ⚠️ Paginación automática (50 líneas/página)

---

## 🎯 Roadmap Futuro

- [ ] Preservar estilos avanzados en DOCX
- [ ] Tablas complejas en PDF
- [ ] Soporte para Excel (.xlsx)
- [ ] Mantener imágenes (sin anonimizar)
- [ ] Batch processing (múltiples archivos)

---

## 📊 Comparación

| Feature | Texto Plano | DOCX | PDF |
|---------|-------------|------|-----|
| Anonimización | ✅ | ✅ | ✅ |
| Formato preservado | N/A | ⚠️ Básico | ⚠️ Básico |
| Audit report | ✅ | ⚠️ No visible | ⚠️ No visible |
| Descarga | ❌ | ✅ | ✅ |

---

## 🔍 Testing

### Test DOCX

```bash
# Crear documento de prueba
echo "Mi email es juan@empresa.com y DNI 12345678Z" > test.txt
# (convertir a .docx con Word/LibreOffice)

# Subir vía web o cURL
curl -X POST http://localhost:3000/api/anonymize-file \
  -F "file=@test.docx" \
  --output test_anonymized.docx
```

### Test PDF

```bash
# Similar con .pdf
curl -X POST http://localhost:3000/api/anonymize-file \
  -F "file=@test.pdf" \
  --output test_anonymized.pdf
```

---

## 🛠️ Troubleshooting

**Error: "Tipo de archivo no soportado"**
- Verifica que el archivo termine en .docx o .pdf

**Error: "Error al leer DOCX"**
- El archivo podría estar corrupto
- Intenta abrirlo y guardarlo de nuevo

**PDF vacío**
- Algunos PDFs son imágenes escaneadas
- pdf-extract solo funciona con PDFs con texto

---

## 📝 Notas Técnicas

### Módulo `document_processor`

```
src/document_processor/
├── mod.rs          # Entry point, detecta tipo
├── docx.rs         # Procesamiento DOCX
└── pdf.rs          # Procesamiento PDF
```

### Flujo DOCX

1. `docx-rs` lee el archivo
2. Extrae texto de párrafos y tablas
3. Anonimiza con motor existente
4. Crea nuevo DOCX con texto anonimizado
5. Serializa a bytes

### Flujo PDF

1. `pdf-extract` extrae texto
2. Anonimiza con motor existente
3. `lopdf` crea PDF nuevo con texto simple
4. Devuelve bytes

---

## ✅ Checklist de Deploy

Si ya tenías anonymize desplegado:

```bash
# 1. Pull nuevo código
git pull

# 2. Actualizar dependencies
cargo build --release

# 3. Deploy
./deploy.sh user@server.com

# 4. Verificar
curl -X POST http://your-server.com/api/anonymize-file \
  -F "file=@test.docx" \
  --output result.docx
```

---

**🎉 ¡Disfruta de la nueva funcionalidad!**

Ahora puedes anonimizar **texto Y documentos** desde la misma interfaz.
