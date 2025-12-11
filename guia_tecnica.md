# Guía para Desarrolladores - MACY-UTP

## 📋 Resumen Ejecutivo

### Propósito del Proyecto
**MACY-UTP** (Manejador Automático de Cajas Yappy UTP) es un sistema backend robusto desarrollado en Rust para gestionar operaciones de cajas registradoras integradas con el sistema de pagos móviles **Yappy** de Panamá. El sistema está diseñado para la Universidad Tecnológica de Panamá (UTP) y maneja transacciones financieras, pagos y control de cajas de forma automatizada.

### Tecnologías Principales

| Tecnología | Versión | Propósito |
|------------|---------|-----------|
| **Rust** | 1.88 | Lenguaje principal |
| **Axum** | 0.8.4 | Framework web asíncrono |
| **Diesel** | 2.2.11 | ORM para base de datos |
| **MariaDB** | 11.3 | Base de datos relacional |
| **Nginx** | latest | Proxy inverso y SSL/TLS |
| **Docker** | - | Containerización |
| **Tokio** | 1.46.1 | Runtime asíncrono |

### Características Clave

✅ **API REST** completa para gestión de cajas  
✅ **Integración con Yappy** para pagos móviles  
✅ **Generación de códigos QR** dinámicos para cobros  
✅ **Cierre automático** de cajas a las 23:00 (cron job)  
✅ **Autenticación JWT** para seguridad  
✅ **Gestión de estados** de cajas (Abierto/Cerrado)  
✅ **Registro de transacciones** y errores  
✅ **Soporte SSL/TLS** mediante Nginx  

---

## 🏗️ Arquitectura del Proyecto

### Visión General de Alto Nivel

```
┌─────────────────────────────────────────────────────────────┐
│                    CLIENTE (Frontend)                        │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTPS:3000
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              NGINX (Proxy Inverso + SSL)                     │
│  • Terminación SSL/TLS                                       │
│  • Balanceo de carga                                         │
│  • Headers de proxy                                          │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP:3333
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                  MACY-BACKEND (Rust/Axum)                    │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Capa de Presentación (Axum)                          │   │
│  │  • Rutas HTTP                                        │   │
│  │  • Middlewares (CORS, Logging, Panic Handler)       │   │
│  │  • Validación de entrada                            │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Capa de Negocio (Controllers)                        │   │
│  │  • Lógica de cajas (abrir/cerrar)                   │   │
│  │  • Integración Yappy (QR, transacciones)            │   │
│  │  • Gestión de grupos                                │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Capa de Datos (Diesel ORM)                           │   │
│  │  • Modelos de datos                                  │   │
│  │  • Consultas SQL type-safe                          │   │
│  │  • Pool de conexiones (R2D2)                        │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Servicios Auxiliares                                 │   │
│  │  • Schedulers (cierre automático)                   │   │
│  │  • Utilidades (errores, auth headers)               │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────┬────────────────────────────────────┘
                         │ MySQL Protocol:3307
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    MARIADB (Base de Datos)                   │
│  • Tabla: cajas                                              │
│  • Tabla: grupos                                             │
│  • Tabla: kioskos                                            │
│  • Tabla: caja_cierre_resumen                                │
│  • Tabla: caja_cierre_errores                                │
└─────────────────────────────────────────────────────────────┘
```

### Componentes Clave

#### 1. **Servidor Web (Axum)**
- **Ubicación**: `src/start_axum.rs`
- **Responsabilidad**: Gestión de rutas HTTP, middlewares y servidor
- **Puerto**: 3333 (interno)

#### 2. **Controladores**
- **Ubicación**: `src/controllers/`
- **Componentes**:
  - `yappy.rs`: Integración con API de Yappy
  - `grupos.rs`: Gestión de grupos y cajas
  - `structs/`: DTOs para comunicación con Yappy

#### 3. **Capa de Datos**
- **Ubicación**: `src/db/`
- **Componentes**:
  - `conection.rs`: Pool de conexiones MySQL
  - `models.rs`: Modelos ORM (Caja, Grupo, Kiosko)
  - `types/enums.rs`: Tipos personalizados (CajasEstadoEnum)

#### 4. **Schedulers**
- **Ubicación**: `src/schedulers/cajas.rs`
- **Función**: Cierre automático de cajas a las 23:00 hora Panamá

#### 5. **Utilidades**
- **Ubicación**: `src/utils/`
- **Funciones**:
  - Manejo de errores JSON
  - Headers de autenticación
  - Operaciones con Yappy

### Patrones de Diseño Utilizados

#### 🎯 **Repository Pattern**
```rust
// Diesel ORM actúa como repositorio
use diesel::prelude::*;

let cajas = cajas::table
    .filter(cajas::estado.eq(CajasEstadoEnum::Abierto))
    .load::<Caja>(&mut conn)?;
```

#### 🎯 **Dependency Injection**
```rust
#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}

// Inyectado en todos los handlers
pub async fn handler(State(state): State<AppState>) { }
```

#### 🎯 **Adapter Pattern**
```rust
// Adaptación de estructuras internas a formato API Yappy
impl AbrirCaja {
    pub fn to_payload(&self) -> RootPayload {
        // Transformación de datos
    }
}
```

#### 🎯 **Middleware Chain**
```rust
let app = Router::new()
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http())
    .layer(CatchPanicLayer::new());
```

### Flujo de Datos

#### Ejemplo: Generar Código QR para Pago

```
1. Cliente → POST /generar-qr
   {
     "tipo_qr": "DYN",
     "subtotal": 10.00,
     "total": 11.50,
     "impuesto": 1.50
   }
   ↓
2. Nginx → Proxy a macy-backend:3333
   ↓
3. Axum Router → generar_qr()
   ↓
4. Validar mac-address en headers
   ↓
5. Consultar BD → Obtener info de caja
   ↓
6. ¿Caja abierta?
   NO → Llamar abrir_caja() automáticamente
   SÍ → Continuar
   ↓
7. Formatear payload → GenerarQR::to_payload()
   ↓
8. HTTP POST → API Yappy
   ↓
9. Guardar transaction_id en BD
   ↓
10. Respuesta al cliente ← Código QR + datos
```

---

## ⚙️ Configuración e Instalación

### Requisitos Previos

#### Software Necesario

| Software | Versión Mínima | Propósito |
|----------|----------------|-----------|
| **Rust** | 1.70+ | Compilador principal |
| **Cargo** | (incluido con Rust) | Gestor de paquetes |
| **MySQL/MariaDB** | 8.0+ / 11.3+ | Base de datos |
| **Docker** | 20.10+ | Containerización (opcional) |
| **Docker Compose** | 1.29+ | Orquestación (opcional) |
| **OpenSSL** | 1.1.1+ | Criptografía |

#### Instalación de Rust

**Windows:**
```powershell
# Descargar desde https://rustup.rs/
# O usar winget
winget install Rustlang.Rustup
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Instalación de Diesel CLI

```bash
# Con soporte MySQL
cargo install diesel_cli --no-default-features --features mysql
```

### Configuración del Entorno

#### 1. Clonar el Repositorio

```bash
git clone <repository-url>
cd MACY-UTP
```

#### 2. Configurar Variables de Entorno

Crear archivo `.env` en la raíz del proyecto:

```env
# Base de Datos
DATABASE_URL=mysql://usuario:contraseña@localhost:3307/macy

# API Yappy
YAPPY_ENDPOINT=https://uat-api-pago.yappy.com.pa

# MySQL Client (Windows)
MYSQLCLIENT_VERSION=8.0
MYSQLCLIENT_LIB_DIR=C:\Program Files\MySQL\MySQL Server 8.0\lib

# Logging
RUST_LOG=info
RUST_BACKTRACE=1
```

⚠️ **Nota para Windows**: Ajustar `MYSQLCLIENT_LIB_DIR` según tu instalación de MySQL.

#### 3. Configurar Base de Datos

**Opción A: Docker Compose (Recomendado)**

```bash
# Iniciar servicios
docker-compose up -d macy-mariadb

# Verificar que esté corriendo
docker ps
```

**Opción B: MySQL/MariaDB Local**

```sql
CREATE DATABASE macy CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'macy_user'@'localhost' IDENTIFIED BY 'tu_contraseña';
GRANT ALL PRIVILEGES ON macy.* TO 'macy_user'@'localhost';
FLUSH PRIVILEGES;
```

#### 4. Ejecutar Migraciones

```bash
# Configurar Diesel
diesel setup

# Ejecutar migraciones
diesel migration run

# Verificar esquema generado
cat src/schema.rs
```

### Configuración de Certificados SSL (Producción)

```bash
# Crear directorio
mkdir -p nginx/certificados

# Copiar certificados
cp /ruta/a/fullchain.pem nginx/certificados/
cp /ruta/a/comodin.key nginx/certificados/

# Ajustar permisos
chmod 600 nginx/certificados/comodin.key
```

---

## 📁 Organización del Código

### Estructura de Directorios

```
MACY-UTP/
├── src/
│   ├── main.rs                    # Punto de entrada
│   ├── start_axum.rs              # Configuración del servidor
│   ├── schema.rs                  # Esquema DB (generado)
│   │
│   ├── controllers/               # Lógica de negocio
│   │   ├── mod.rs
│   │   ├── yappy.rs              # Integración Yappy
│   │   ├── grupos.rs             # Gestión de grupos
│   │   └── structs/              # DTOs
│   │       ├── mod.rs
│   │       └── yappy.rs          # Estructuras Yappy
│   │
│   ├── db/                        # Capa de datos
│   │   ├── mod.rs
│   │   ├── conection.rs          # Pool de conexiones
│   │   ├── models.rs             # Modelos ORM
│   │   └── types/
│   │       ├── mod.rs
│   │       └── enums.rs          # Enums personalizados
│   │
│   ├── schedulers/                # Tareas programadas
│   │   ├── mod.rs
│   │   └── cajas.rs              # Cierre automático
│   │
│   └── utils/                     # Utilidades
│       ├── mod.rs
│       ├── utils.rs              # Funciones generales
│       └── cajas_utils.rs        # Operaciones Yappy
│
├── migrations/                    # Migraciones Diesel
│   └── YYYY-MM-DD-HHMMSS_nombre/
│       ├── up.sql
│       └── down.sql
│
├── nginx/                         # Configuración Nginx
│   ├── default.conf
│   └── certificados/
│       ├── fullchain.pem
│       └── comodin.key
│
├── Cargo.toml                     # Dependencias Rust
├── Cargo.lock                     # Versiones bloqueadas
├── diesel.toml                    # Configuración Diesel
├── dockerfile.macy                # Dockerfile multi-etapa
├── docker-compose.yml             # Orquestación Docker
├── .env                           # Variables de entorno
└── README.md                      # Documentación
```

### Archivos Clave y sus Propósitos

#### 🔑 **Archivos de Configuración**

| Archivo | Propósito |
|---------|-----------|
| `Cargo.toml` | Dependencias y metadatos del proyecto |
| `diesel.toml` | Configuración del ORM Diesel |
| `.env` | Variables de entorno (no versionar) |
| `docker-compose.yml` | Definición de servicios Docker |

#### 🔑 **Archivos de Código