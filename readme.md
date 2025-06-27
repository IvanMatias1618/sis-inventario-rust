┌─────────────────────────────────────────────┐
│             PROYECTO: INVENTARIO            │
├─────────────────────────────────────────────┤
│ ⏳ Estado actual:                            │
│   - ✅ Frontend conectado con backend        │
│   - ✅ POST funcional y probado              │
│   - ⚠️ Mejoras futuras:                      │
│       • Validación visual por campo         │
│       • Modularización de config/IP         │
│       • Notificaciones elegantes (UX)       │
│                                             │
│ 🗃️ Archivos organizados:                     │
│   /frontend/src/*.ts                         │
│   /frontend/dist/*.js                        │
│   main.rs con CORS activo                   │
│                                             │
│ 🔌 IP usada: 127.0.0.1 ó IP LAN              │
│ 🌍 Front servido desde: http://localhost:3000│
│ 📡 Backend en: http://127.0.0.1:8080         │
└─────────────────────────────────────────────┘

# 📦 Inventario

Sistema de gestión de insumos y recetas con frontend en TypeScript y backend en Rust.

## 🚀 ¿Qué hace?
Permite crear, listar y buscar insumos desde un formulario web, conectando con una API REST hecha en Rust con actix-web.

## 🧰 Tecnologías
- Frontend: TypeScript, HTML, CSS
- Backend: Rust + actix-web
- Servidor local: `live-server` o `python3 -m http.server`

## 🛠 Cómo usar

1. Clona el repositorio
2. En `frontend/`, compila TypeScript:
   ```bash
   npx tsc
