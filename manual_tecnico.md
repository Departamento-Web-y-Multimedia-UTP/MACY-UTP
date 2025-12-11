Project Overview:
# Resumen del Proyecto MACY-UTP

## Lenguaje Principal
**Rust** - Identificado por los archivos `Cargo.toml` (gestor de paquetes de Rust) y la extensión `.rs` en todos los archivos fuente.

## Propósito del Proyecto

Este es un **sistema backend web desarrollado en Rust** que gestiona operaciones relacionadas con **pagos y cajas**, específicamente integrado con **Yappy** (sistema de pagos móviles popular en Panamá).

## Componentes Principales

### 1. **Framework Web**
- Utiliza **Axum** como servidor web (archivo `start_axum.rs`)
- Configurado con **Nginx** como proxy reverso
- Dockerizado para despliegue en contenedores

### 2. **Base de Datos**
- Implementa **Diesel ORM** (`diesel.toml`) para gestión de base de datos
- Incluye modelos, esquemas y tipos personalizados (enums)
- Sistema de conexión estructurado

### 3. **Funcionalidades Clave**
- **Controladores de Grupos**: Gestión de agrupaciones
- **Integración con Yappy**: Procesamiento de pagos móviles
- **Sistema de Cajas**: Gestión de cajas registradoras/puntos de venta
- **Schedulers**: Tareas programadas para operaciones de cajas

### 4. **Arquitectura**
- Patrón MVC con separación clara de responsabilidades
- Utilidades compartidas para lógica de negocio
- Estructuras de datos tipadas para Yappy

## Contexto
Probablemente es un sistema para la **Universidad Tecnológica de Panamá (UTP)** que gestiona transacciones financieras, pagos estudiantiles o administrativos mediante Yappy, con control de cajas y grupos de usuarios.

File: Cargo.toml
# Análisis de Cargo.toml - MACY-UTP

## 1. Propósito General del Archivo

Este es el archivo de manifiesto de un proyecto **Rust** que define las dependencias y metadatos del paquete. `Cargo.toml` es el archivo de configuración principal para proyectos Rust, similar a `package.json` en Node.js o `requirements.txt` en Python.

El proyecto **MACY-UTP** parece ser una **aplicación web backend** con capacidades de:
- Servidor web API REST
- Conexión a base de datos MySQL
- Autenticación JWT
- Tareas programadas (cron jobs)
- Logging y trazabilidad

## 2. Campos/Variables y sus Propósitos

### Sección `[package]`
- **`name`**: "MACY-UTP" - Nombre del proyecto/paquete
- **`version`**: "0.1.0" - Versión semántica (inicial)
- **`edition`**: "2024"

### Sección `[dependencies]`

#### **Frameworks Web**
- **`axum`** (0.8.4): Framework web moderno y ergonómico construido sobre Tokio, usado para crear APIs REST

#### **Base de Datos**
- **`diesel`** (2.2.11): ORM (Object-Relational Mapping) para Rust
  - Features: `mysql`, `numeric`, `r2d2`, `serde_json`
  - Propósito: Interactuar con base de datos MySQL de forma segura
- **`diesel-derive-enum`** (2.1.0): Macros para derivar enums compatibles con Diesel
- **`bigdecimal`** (0.4.8): Manejo de números decimales de precisión arbitraria (útil para valores monetarios)

#### **Manejo de Fechas**
- **`chrono`** (0.4.41): Biblioteca para manejo de fechas y horas
  - Feature: `serde` - Serialización/deserialización
- **`chrono-tz`** (0.10.4): Soporte para zonas horarias

#### **Serialización**
- **`serde`** (1.0.219): Framework de serialización/deserialización
  - Feature: `derive` - Macros para derivar automáticamente
- **`serde_json`** (1.0.140): Soporte JSON para Serde

#### **Autenticación**
- **`jsonwebtoken`** (10.2.0): Implementación de JWT (JSON Web Tokens)
  - Feature: `aws_lc_rs` - Backend criptográfico de AWS

#### **HTTP Cliente**
- **`reqwest`** (0.12.22): Cliente HTTP asíncrono
  - Feature: `json` - Soporte para JSON

#### **Runtime Asíncrono**
- **`tokio`** (1.46.1): Runtime asíncrono para Rust
  - Feature: `full` - Todas las características habilitadas

#### **Tareas Programadas**
- **`tokio-cron-scheduler`** (0.14.0): Programador de tareas tipo cron para Tokio

#### **Middleware HTTP**
- **`tower-http`** (0.6.6): Middleware HTTP para Tower/Axum
  - Features: `catch-panic`, `cors`, `trace`
  - Propósito: Manejo de errores, CORS, trazabilidad

#### **Logging y Trazabilidad**
- **`tracing`** (0.1.41): Framework de instrumentación para logging estructurado
- **`tracing-subscriber`** (0.3.19): Utilidades para consumir eventos de tracing

#### **Utilidades**
- **

File: diesel.toml
# Análisis de diesel.toml

## 1. Propósito General del Archivo

Este es un archivo de configuración para **Diesel CLI**, una herramienta de línea de comandos para el ORM (Object-Relational Mapping) Diesel en Rust. Diesel es utilizado para interactuar con bases de datos de manera segura y eficiente. Este archivo configura cómo Diesel genera código y maneja migraciones de base de datos.

## 2. Lista de Campos/Variables y sus Propósitos

### Sección `[print_schema]`
Configura cómo Diesel genera el archivo de esquema de base de datos:

- **`file = "src/schema.rs"`**
  - **Propósito**: Define la ubicación donde Diesel generará automáticamente el archivo de esquema
  - **Valor**: Ruta al archivo que contendrá las definiciones de tablas de la base de datos
  - **Uso**: Cuando ejecutas `diesel migration run` o `diesel print-schema`, el código se genera aquí

- **`custom_type_derives = ["diesel::query_builder::QueryId", "Clone"]`**
  - **Propósito**: Especifica traits adicionales que se derivarán automáticamente para las estructuras generadas
  - **Valores**:
    - `diesel::query_builder::QueryId`: Permite que las tablas sean identificables en el sistema de queries de Diesel
    - `Clone`: Permite clonar las estructuras de tabla generadas
  - **Beneficio**: Añade funcionalidad extra a las estructuras generadas automáticamente

- **`#filter = { except_tables = ["cajas"] }`** (COMENTADO)
  - **Propósito**: Cuando está activo, excluye tablas específicas de la generación del esquema
  - **Estado**: Actualmente deshabilitado (comentado con `#`)
  - **Uso potencial**: Si se descomenta, la tabla "cajas" no se incluiría en `schema.rs`

### Sección `[migrations_directory]`
Configura la ubicación de las migraciones de base de datos:

- **`dir = "C:\\Proyectos\\AxumApps\\MACY-UTP\\migrations"`**
  - **Propósito**: Define el directorio donde se almacenan los archivos de migración de base de datos
  - **Ruta absoluta**: Apunta a una ubicación específica en Windows
  - **Contenido**: Carpeta con archivos SQL para crear/modificar la estructura de la base de datos

## 3. Definiciones de Funciones

**No aplica** - Este es un archivo de configuración TOML, no contiene funciones ejecutables. Es leído por Diesel CLI para configurar su comportamiento.

## 4. Estructuras/Clases

**No aplica** - Este archivo no define estructuras o clases. Sin embargo, **influye en la generación** de estructuras en `src/schema.rs`.

### Estructuras Generadas (Ejemplo)
Basándose en esta configuración, Diesel generaría algo como:

```rust
// En src/schema.rs (generado automáticamente)
#[derive(diesel::query_builder::QueryId, Clone)]
table! {
    usuarios (id) {
        id -> Integer,
        nombre -> Text,
        // ... otros campos
    }
}
```

## 5. Cómo Encaja en el Proyecto

### Contexto del Proyecto: MACY-UTP
Este archivo es parte de un proyecto llamado **MACY-UTP** construido con:
- **Axum**: Framework web para Rust
- **Diesel**: ORM para manejo de base de datos

### Rol en la Arquitectura

```
┌─────────────────────────────────────────┐
│         diesel.toml                     │
│    (Configuración de Diesel CLI)        │
└──────────────┬──────────────────────────┘
               │
               ├──► Genera: src/schema.rs
               │

File: docker-compose.yml
# Análisis de docker-compose.yml

## 1. Propósito General del Archivo

Este archivo `docker-compose.yml` define la infraestructura de contenedores para una aplicación llamada "Macy". Orquesta tres servicios principales:
- Un backend (aplicación principal)
- Una base de datos MariaDB
- Un servidor web Nginx como proxy inverso/balanceador de carga

Es la configuración de despliegue que permite levantar toda la aplicación con un solo comando.

## 2. Lista de Campos/Variables y sus Propósitos

### **Servicio: macy-backend**

| Campo | Propósito |
|-------|-----------|
| `build.context: .` | Directorio raíz para construir la imagen Docker |
| `build.dockerfile: dockerfile.macy` | Archivo Dockerfile específico para construir el backend |
| `container_name: macy-backend` | Nombre identificador del contenedor |
| `depends_on: - macy-mariadb` | Indica que este servicio requiere que MariaDB esté iniciado primero |
| `environment.DATABASE_URL` | URL de conexión a la base de datos MySQL/MariaDB |
| `environment.YAPPY_ENDPOINT` | Endpoint de API de integración con Yappy (sistema de pagos) en ambiente UAT |
| `environment.MYSQLCLIENT_VERSION: 8.0` | Versión del cliente MySQL a utilizar |
| `environment.MYSQLCLIENT_LIB_DIR` | Directorio de librerías del cliente MySQL en el sistema |
| `expose: - "3333:3333"` | Expone el puerto 3333 internamente en la red Docker |
| `restart: always` | Política de reinicio automático del contenedor |

### **Servicio: macy-mariadb**

| Campo | Propósito |
|-------|-----------|
| `image: mariadb:11.3` | Imagen oficial de MariaDB versión 11.3 |
| `container_name: macy-mariadb` | Nombre del contenedor de base de datos |
| `restart: always` | Reinicio automático en caso de fallo |
| `environment.MYSQL_ROOT_PASSWORD` | Contraseña del usuario root de MySQL (redactada por seguridad) |
| `environment.MYSQL_DATABASE: macy` | Nombre de la base de datos a crear automáticamente |
| `environment.MYSQL_USER` | Usuario de base de datos (redactado) |
| `environment.MYSQL_PASSWORD` | Contraseña del usuario (redactada) |
| `volumes: - mariadb_macy_data:/var/lib/mysql` | Volumen persistente para datos de la BD |
| `ports: - "3307:3306"` | Mapeo de puerto 3306 (interno) al 3307 (host) |

### **Servicio: nginx**

| Campo | Propósito |
|-------|-----------|
| `image: nginx:latest` | Imagen oficial de Nginx última versión |
| `container_name: macy-nginx` | Nombre del contenedor proxy |
| `restart: always` | Reinicio automático |
| `ports: - "3000:3000"` | Expone el puerto 3000 al host |
| `volumes: - ./nginx/default.conf` | Monta configuración personalizada de Nginx |
| `volumes: - ./nginx/certificados/comodin.key` | Monta certificado SSL (clave privada) |
| `volumes: - ./nginx/certificados/fullchain.pem` | Monta cadena completa de certificados SSL |
| `depends_on: - macy-backend` | Depende del backend para funcionar |

### **Volúmenes**

| Campo | Propósito |
|-------|-----------|
| `mariadb_macy_data` | Volumen nombrado para persistencia de datos de MariaDB |

## 3. Definiciones de Funciones

**Este

File: dockerfile.macy
# Análisis de dockerfile.macy

## 1. Propósito General del Archivo

Este es un **Dockerfile multi-etapa** diseñado para construir y ejecutar una aplicación Rust llamada "MACY-UTP". Utiliza el patrón de construcción en dos fases para optimizar el tamaño de la imagen final, separando las herramientas de compilación del entorno de ejecución.

## 2. Campos/Variables y sus Propósitos

### **Etapa de Construcción (builder)**
- **`FROM rust:1.88-slim-bookworm`**: Imagen base con Rust 1.88 sobre Debian Bookworm (versión ligera)
- **Dependencias del sistema instaladas**:
  - `build-essential`: Herramientas de compilación (gcc, g++, make)
  - `libssl-dev`: Bibliotecas de desarrollo para OpenSSL (criptografía/HTTPS)
  - `libmariadb-dev`: Bibliotecas de desarrollo para MariaDB/MySQL
  - `pkg-config`: Herramienta para gestionar flags de compilación

- **`WORKDIR /app`**: Directorio de trabajo establecido en `/app`
- **Archivos copiados**:
  - `Cargo.toml` y `Cargo.lock`: Manifiestos de dependencias de Rust
  - `src`: Código fuente de la aplicación

### **Etapa de Ejecución (runtime)**
- **`FROM debian:12-slim`**: Imagen base minimalista de Debian 12
- **Dependencias de runtime**:
  - `libssl-dev`: Bibliotecas SSL para ejecución
  - `libmariadb-dev`: Bibliotecas MariaDB para ejecución
- **Binario copiado**: `MACY-UTP` desde la etapa de construcción

## 3. Definiciones de Funciones (Comandos Docker)

### **RUN apt-get update && apt upgrade -y && apt-get install -y**
- **Entrada**: Lista de paquetes del sistema
- **Salida**: Sistema con dependencias instaladas
- **Propósito**: Actualizar repositorios e instalar herramientas necesarias para compilación

### **RUN cargo build --release**
- **Entrada**: Código fuente en `/app`
- **Salida**: Binario optimizado en `/app/target/release/MACY-UTP`
- **Propósito**: Compilar la aplicación Rust en modo release (optimizado)

### **COPY --from=builder**
- **Entrada**: Binario de la etapa builder
- **Salida**: Binario en la imagen runtime
- **Propósito**: Transferir solo el ejecutable final, sin herramientas de compilación

### **CMD ["./MACY-UTP"]**
- **Entrada**: Ninguna
- **Salida**: Proceso de aplicación en ejecución
- **Propósito**: Comando por defecto al iniciar el contenedor

## 4. Estructuras/Clases (Etapas Docker)

### **Stage: builder**
```dockerfile
FROM rust:1.88-slim-bookworm AS builder
```
- **Importancia**: Entorno completo de compilación con todas las herramientas necesarias
- **Características**: 
  - Contiene compilador Rust
  - Herramientas de desarrollo
  - Genera el binario optimizado

### **Stage: runtime**
```dockerfile
FROM debian:12-slim AS runtime
```
- **Importancia**: Imagen final ligera para producción
- **Características**:
  - Solo bibliotecas de runtime necesarias
  - Sin herramientas de compilación
  - Tamaño reducido (~80% más pequeña que incluir todo)

## 5. Cómo Encaja en el Proyecto

### **Rol en el Proyecto MACY-UTP**

1. **Containerización**: Empaqueta la aplicación Rust en un contenedor Docker portable

File: README.md
# Análisis del archivo README.md

## 1. Propósito General del Archivo

Este es un archivo de documentación (README) para un proyecto llamado **"Manejador Automático de Cajas Yappy UTP"**. Su propósito es proporcionar:
- Información sobre el creador del proyecto
- Instrucciones de instalación y configuración
- Guía para ejecutar el programa
- Solución a problemas comunes
- Comandos útiles para el ORM Diesel

## 2. Campos/Variables Mencionadas

| Variable | Propósito |
|----------|-----------|
| `MYSQLCLIENT_VERSION` | Variable de entorno que especifica la versión de libmysqlclient instalada |
| `MYSQLCLIENT_LIB_DIR` | Variable de entorno que apunta al directorio donde se encuentra la librería MySQL |
| `RUST_BACKTRACE` | Variable de entorno para mostrar trazas detalladas de errores en Rust |

## 3. Comandos/Funciones Documentadas

### Comandos de Cargo (Rust):
- **`cargo run`**
  - **Entrada**: Ninguna
  - **Salida**: Ejecuta el programa
  - **Propósito**: Compila y ejecuta automáticamente el proyecto, descargando dependencias necesarias

### Comandos de Diesel ORM:
- **`diesel setup`**
  - **Propósito**: Inicialización principal del ORM Diesel en el proyecto

- **`diesel migration generate --diff-schema {{nombre_de_migracion}}`**
  - **Entrada**: Nombre de la migración
  - **Propósito**: Genera archivos de migración basados en diferencias del schema de base de datos

- **`diesel migration run`**
  - **Propósito**: Ejecuta las migraciones pendientes en la base de datos

## 4. Estructuras/Clases

No se definen estructuras o clases en este archivo, ya que es documentación. Sin embargo, se mencionan:

- **Crates (paquetes)**: Dependencias de Rust que el proyecto utiliza
- **mysqlclient-sys**: Crate para conectividad con MySQL
- **Diesel**: ORM (Object-Relational Mapping) para Rust

## 5. Cómo Encaja Este Archivo en el Proyecto

### Rol en el Proyecto:
Este README es el **punto de entrada para desarrolladores** que necesitan:

1. **Configurar el entorno de desarrollo**
   - Instalación de Rust
   - Configuración de MySQL
   - Instalación de OpenSSL

2. **Entender la arquitectura**
   - El proyecto usa Rust como lenguaje principal
   - Utiliza MySQL como base de datos
   - Implementa Diesel como ORM para manejo de datos

3. **Resolver problemas comunes**
   - Errores de compilación relacionados con MySQL
   - Problemas con DLLs de OpenSSL en Windows

### Contexto del Proyecto:
- **Dominio**: Sistema de gestión de cajas para Yappy (probablemente un sistema de pagos o logística)
- **Institución**: UTP (Universidad Tecnológica de Panamá, presumiblemente)
- **Tecnologías clave**: Rust, MySQL, Diesel ORM
- **Plataforma objetivo**: Principalmente Windows (según los errores documentados)

### Importancia:
Este archivo es **crítico** para:
- Onboarding de nuevos desarrolladores
- Documentación de configuración específica de Windows
- Referencia rápida de comandos de Diesel
- Troubleshooting de problemas de compilación

Directory: .
# Análisis del Directorio: MACY-UTP

## 🎯 Propósito General del Proyecto

**MACY-UTP** (Manejador Automático de Cajas Yappy UTP) es una **aplicación backend web construida en Rust** que gestiona operaciones de cajas para el sistema de pagos Yappy. Es un sistema empresarial con arquitectura moderna que incluye:

- API REST para gestión de transacciones
- Integración con sistema de pagos Yappy
- Base de datos relacional (MariaDB/MySQL)
- Autenticación JWT
- Tareas programadas (cron jobs)
- Despliegue containerizado con Docker

---

## 🏗️ Arquitectura del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                    CLIENTE (Frontend)                        │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTPS (Puerto 3000)
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                   NGINX (Proxy Inverso)                      │
│  - Terminación SSL (certificados comodín)                    │
│  - Balanceo de carga                                         │
│  - Configuración: nginx/default.conf                         │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP (Puerto 3333)
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              MACY-BACKEND (Aplicación Rust)                  │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Axum Framework (Web Server)                         │    │
│  │  - Rutas API REST                                   │    │
│  │  - Middleware (CORS, Logging, Panic Handler)        │    │
│  │  - Autenticación JWT                                │    │
│  └─────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Diesel ORM                                          │    │
│  │  - Conexión a BD (Pool R2D2)                        │    │
│  │  - Migraciones automáticas                          │    │
│  │  - Schema generado: src/schema.rs                   │    │
│  └─────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Servicios de Negocio                                │    │
│  │  - Integración Yappy (reqwest)                      │    │
│  │  - Tareas programadas (tokio-cron-scheduler)        │    │
│  │  - Manejo de decimales (bigdecimal)                 │    │
│  └─────────────────────────────────────────────────────┘    │
└────────────────────────┬────────────────────────────────────┘
                         │ MySQL Protocol (Puerto 3307)
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                 MARIADB 11.3 (Base de Datos)                 │
│  - Base de datos: macy                                       │
│  - Volumen persistente: mariadb_macy_data                    │
│  - Migraciones en: migrations/                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 📦 Cómo Trabajan Juntos los Archivos

### **1. Configuración del Proyecto (Cargo.toml)**
**Rol**:

File: nginx\default.conf
# Análisis de nginx\default.conf

## 1. Propósito General del Archivo

Este archivo es una configuración de servidor **Nginx** que actúa como **proxy inverso con SSL/TLS** para una aplicación backend. Específicamente, configura un servidor que escucha en el puerto 3000 con HTTPS y redirige todas las peticiones a un servicio backend llamado `macy-backend` que corre en el puerto 3333.

## 2. Lista de Campos/Variables y sus Propósitos

### Configuración del Servidor

| Campo/Directiva | Valor | Propósito |
|----------------|-------|-----------|
| `listen` | `3000 ssl` | Puerto de escucha con SSL habilitado |
| `server_name` | `dtwm-pruebas.utp.ac.pa` | Nombre de dominio del servidor |

### Configuración SSL/TLS

| Campo/Directiva | Valor | Propósito |
|----------------|-------|-----------|
| `ssl_certificate` | `/etc/nginx/certs/fullchain.pem` | Ruta al certificado SSL completo (incluye certificados intermedios) |
| `ssl_certificate_key` | `/etc/nginx/certs/comodin.key` | Ruta a la clave privada del certificado (certificado wildcard) |
| `ssl_protocols` | `TLSv1 TLSv1.1 TLSv1.2` | Protocolos TLS permitidos (⚠️ TLSv1 y TLSv1.1 están obsoletos) |
| `ssl_ciphers` | `HIGH:!aNULL:!MD5` | Cifrados permitidos: alta seguridad, sin cifrados nulos ni MD5 |

### Headers del Proxy

| Header | Valor | Propósito |
|--------|-------|-----------|
| `X-Real-IP` | `$remote_addr` | IP real del cliente |
| `X-Forwarded-For` | `$proxy_add_x_forwarded_for` | Cadena de IPs de proxies intermedios |
| `Host` | `$http_host` | Host original solicitado por el cliente |
| `X-Nginx-Proxy` | `true` | Indica que la petición pasa por Nginx |
| `Upgrade` | `$http_upgrade` | Soporte para WebSockets |
| `Connection` | `"upgrade"` | Mantiene conexión para WebSockets |

### Configuración del Proxy

| Campo/Directiva | Valor | Propósito |
|----------------|-------|-----------|
| `proxy_pass` | `http://macy-backend:3333/` | URL del servicio backend destino |
| `proxy_redirect` | `off` | Desactiva reescritura automática de redirects |
| `proxy_http_version` | `1.1` | Usa HTTP/1.1 para comunicación con backend |

## 3. Definiciones de Funciones (Bloques de Configuración)

### Bloque `location /`
- **Entrada**: Todas las peticiones HTTP(S) al servidor
- **Procesamiento**: 
  - Añade headers de proxy
  - Reenvía la petición al backend
  - Habilita soporte para WebSockets
- **Salida**: Respuesta del backend `macy-backend:3333`
- **Propósito**: Actuar como proxy inverso transparente con soporte para conexiones persistentes

### Bloque `location = /50x.html`
- **Entrada**: Peticiones a `/50x.html`
- **Procesamiento**: Sirve página estática de error
- **Salida**: Archivo HTML de error desde `/usr/share/nginx/html`
- **Propósito**: Página de error personalizada para códigos 500-504

## 4. Estructuras/Clases (Bloques de Configuración)

### Bloque `server { }`
**Import

Directory: nginx
# Análisis del Directorio `nginx`

## 📋 Resumen Ejecutivo

Este directorio contiene la **configuración de Nginx como proxy inverso con SSL/TLS** para la aplicación Macy. Actúa como capa de seguridad y enrutamiento entre los clientes externos y el backend de la aplicación.

## 🎯 Propósito Principal

El directorio `nginx` configura un servidor web que:

1. **Termina conexiones SSL/TLS** - Maneja el cifrado HTTPS en el puerto 3000
2. **Actúa como proxy inverso** - Redirige tráfico al backend en el puerto 3333
3. **Añade seguridad** - Implementa protocolos TLS y cifrados seguros
4. **Habilita WebSockets** - Soporta conexiones bidireccionales en tiempo real
5. **Gestiona headers** - Preserva información del cliente original

## 🔧 Componentes y Funcionamiento

### **default.conf**
Archivo único de configuración que define:

```
Cliente (HTTPS:3000) 
    ↓
[Nginx - Terminación SSL]
    ↓ (headers + proxy)
Backend (HTTP:3333 - macy-backend)
    ↓
[Nginx - Respuesta]
    ↓
Cliente
```

### Flujo de Trabajo

1. **Recepción**: Cliente se conecta a `dtwm-pruebas.utp.ac.pa:3000` vía HTTPS
2. **Desencriptación**: Nginx desencripta usando certificado wildcard (`comodin.key`)
3. **Enriquecimiento**: Añade headers (`X-Real-IP`, `X-Forwarded-For`, etc.)
4. **Proxy**: Reenvía petición HTTP a `macy-backend:3333`
5. **Respuesta**: Devuelve respuesta del backend al cliente encriptada

## 🔐 Características de Seguridad

- ✅ Certificados SSL/TLS (fullchain + wildcard)
- ✅ Cifrados de alta seguridad
- ⚠️ **Advertencia**: Usa TLSv1 y TLSv1.1 (obsoletos, vulnerables)
- ✅ Bloquea cifrados nulos y MD5

## 🐳 Integración con Docker

Este archivo está diseñado para montarse en un contenedor Nginx:
- Certificados esperados en `/etc/nginx/certs/`
- Configuración en `/etc/nginx/conf.d/`
- Comunica con servicio Docker `macy-backend` por nombre

## 💡 Casos de Uso

- **Producción/Staging**: Ambiente de pruebas UTP
- **Microservicios**: Desacopla frontend/backend
- **Balanceo**: Base para añadir múltiples backends
- **Monitoreo**: Headers permiten trazabilidad de peticiones

## ⚠️ Recomendaciones

1. **Actualizar protocolos TLS** a solo TLSv1.2 y TLSv1.3
2. **Añadir rate limiting** para prevenir ataques DDoS
3. **Implementar logs** estructurados para auditoría
4. **Considerar HTTP/2** para mejor rendimiento

File: src\main.rs
# Análisis de `src/main.rs`

## 1. Propósito General del Archivo

Este es el **punto de entrada principal** de una aplicación web Rust construida con el framework Axum. El archivo configura la estructura modular del proyecto, inicializa la conexión a la base de datos MySQL, ejecuta un trabajo programado para cerrar cajas y arranca el servidor web asíncrono.

## 2. Campos/Variables y sus Propósitos

| Variable | Tipo | Propósito |
|----------|------|-----------|
| `db_pool` | `MySqlPool` | Pool de conexiones a la base de datos MySQL para gestionar conexiones eficientemente |
| `state` | `AppState` | Estado compartido de la aplicación que contiene el pool de base de datos, accesible desde todos los handlers |

## 3. Definiciones de Funciones

### `main()`
```rust
#[tokio::main]
async fn main()
```

**Atributo**: `#[tokio::main]` - Macro que convierte la función main en un runtime asíncrono de Tokio

**Entradas**: Ninguna

**Salidas**: Ninguna (implícitamente `()`)

**Propósito**: 
- Inicializar las variables de entorno desde archivo `.env`
- Crear el pool de conexiones a la base de datos
- Instanciar el estado de la aplicación
- Ejecutar el job de cierre de cajas
- Iniciar el servidor web Axum

**Flujo de ejecución**:
1. `dotenv().ok()` - Carga variables de entorno
2. `create_pool()` - Establece conexión con MySQL
3. Crea `AppState` con el pool
4. `cerrar_cajas_job(&state).await.unwrap()` - Ejecuta tarea programada
5. `start_axum(&state).await.unwrap()` - Inicia servidor HTTP

## 4. Structs/Clases y su Importancia

### `AppState`
```rust
#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}
```

**Derivaciones**: `Clone` - Permite clonar el estado (necesario para compartirlo entre handlers)

**Campos**:
- `db_pool`: Pool de conexiones MySQL compartido

**Importancia**: 
- **Patrón de Estado Compartido**: Permite que todos los endpoints de Axum accedan al pool de base de datos
- **Thread-Safe**: Al ser cloneable y usar un pool, puede compartirse de forma segura entre múltiples tareas asíncronas
- **Extensibilidad**: Facilita agregar más recursos compartidos (caché, configuración, etc.)

## 5. Cómo Encaja en el Proyecto

### Arquitectura del Proyecto

```
main.rs (Punto de entrada)
├── schema (Modelos de base de datos)
├── start_axum (Configuración del servidor web)
├── db (Gestión de base de datos)
│   └── conection (Pool de conexiones)
├── controllers (Lógica de negocio/endpoints)
├── schedulers (Tareas programadas)
│   └── cajas (Job de cierre de cajas)
└── utils (Utilidades generales)
```

### Rol en el Sistema

1. **Orquestador Principal**: Coordina la inicialización de todos los componentes
2. **Inyección de Dependencias**: Proporciona `AppState` a todos los módulos que lo necesiten
3. **Ciclo de Vida**: Gestiona el arranque y mantiene viva la aplicación

### Flujo de Datos

```
.env → dotenv() → Variables de entorno
                ↓
         create_pool() → MySqlPool
                ↓
            AppState → Compartido con:
                ├── cerrar_cajas_job (Scheduler)
                └── start_axum (Servidor web)
                        

File: src\schema.rs
# Análisis de `src/schema.rs`

## 1. Propósito General del Archivo

Este archivo define el **esquema de base de datos** utilizando el ORM **Diesel** para Rust. Actúa como una representación en código de las tablas de la base de datos y sus relaciones, permitiendo realizar consultas type-safe (con verificación de tipos en tiempo de compilación). El sistema parece gestionar **cajas registradoras** o **puntos de pago** (posiblemente integrados con Yappy, un sistema de pagos).

## 2. Tablas y sus Campos

### **Tabla: `caja_cierre_errores`**
Registra errores ocurridos durante el cierre de caja.

| Campo | Tipo | Propósito |
|-------|------|-----------|
| `id` | Integer (PK) | Identificador único del error |
| `id_caja` | Integer (FK) | Referencia a la caja donde ocurrió el error |
| `respuesta_json` | Json | Respuesta completa del error en formato JSON |
| `fecha` | Timestamp (Nullable) | Fecha y hora cuando ocurrió el error |

### **Tabla: `caja_cierre_resumen`**
Almacena resúmenes de cierre de caja con totales por tipo de transacción.

| Campo | Tipo | Propósito |
|-------|------|-----------|
| `id` | Integer (PK) | Identificador único del resumen |
| `id_caja` | Integer (FK) | Referencia a la caja cerrada |
| `tipo` | Varchar(50) | Tipo de transacción (ej: efectivo, tarjeta, Yappy) |
| `monto` | Decimal | Monto total para ese tipo de transacción |
| `transacciones` | Integer | Cantidad de transacciones de ese tipo |
| `fecha` | Timestamp (Nullable) | Fecha y hora del cierre |

### **Tabla: `cajas`**
Tabla principal que representa las cajas registradoras o puntos de pago.

| Campo | Tipo | Propósito |
|-------|------|-----------|
| `id` | Integer (PK) | Identificador único de la caja |
| `id_grupo` | Integer (FK) | Referencia al grupo/comercio al que pertenece |
| `nombre_caja` | Varchar(100) | Nombre descriptivo de la caja |
| `tipo` | Varchar(50) | Tipo de caja (ej: física, virtual, kiosko) |
| `token_autorizacion` | Text (Nullable) | Token para autenticación con servicios externos |
| `transaccion_actual` | Varchar(100) (Nullable) | ID de la transacción en curso |
| `estado` | Enum (7 chars) | Estado actual de la caja (usa tipo personalizado) |

**Nota importante**: Usa un tipo enum personalizado `CajasEstadoEnumMapping` importado desde `crate::db::types::enums`.

### **Tabla: `grupos`**
Representa comercios o grupos de comercios que tienen múltiples cajas.

| Campo | Tipo | Propósito |
|-------|------|-----------|
| `id` | Integer (PK) | Identificador único del grupo |
| `id_yappy` | Varchar(100) | Identificador del comercio en el sistema Yappy |
| `nombre` | Varchar(100) | Nombre del comercio/grupo |
| `api_key` | Varchar(255) | Clave API para integración con Yappy |
| `secret_key` | Varchar(255) | Clave secreta para autenticación |

### **Tabla: `kioskos`**
Representa dispositivos físicos (kioskos) asociados a cajas.

| Campo | Tipo | Propósito |
|-------|------|-----------|
| `id` | Integer (PK) | Identificador único del k

File: src\start_axum.rs
# Análisis de `src/start_axum.rs`

## 1. Propósito General del Archivo

Este archivo es el **punto de entrada y configuración del servidor web** usando el framework Axum. Se encarga de:
- Definir todas las rutas HTTP de la API
- Configurar middlewares (CORS, logging, manejo de pánico)
- Inicializar y arrancar el servidor HTTP
- Vincular los controladores con sus respectivas rutas

## 2. Campos/Variables y sus Propósitos

### Variables Locales en `start_axum`:

| Variable | Tipo | Propósito |
|----------|------|-----------|
| `app` | `Router` | Enrutador principal de Axum que contiene todas las rutas y middlewares configurados |
| `listener` | `TcpListener` | Socket TCP que escucha conexiones entrantes en `0.0.0.0:3333` |

## 3. Definiciones de Funciones

### `start_axum`

```rust
pub async fn start_axum(state: &AppState) -> Result<(), Box<dyn std::error::Error>>
```

**Entradas:**
- `state: &AppState` - Referencia al estado compartido de la aplicación que se pasará a todos los handlers

**Salidas:**
- `Result<(), Box<dyn std::error::Error>>` - Retorna `Ok(())` si el servidor se inicia correctamente, o un error en caso contrario

**Propósito:**
- Configurar y arrancar el servidor web Axum
- Definir todas las rutas de la API REST
- Aplicar capas de middleware para funcionalidades transversales
- Inicializar el sistema de logging
- Vincular el servidor al puerto 3333

**Flujo de ejecución:**
1. Crea el router con todas las rutas HTTP
2. Aplica middlewares (CORS, tracing, panic handling)
3. Asocia el estado de la aplicación
4. Configura el logging para desarrollo
5. Crea un listener TCP en `0.0.0.0:3333`
6. Inicia el servidor

## 4. Rutas HTTP Definidas

| Ruta | Método | Handler | Propósito |
|------|--------|---------|-----------|
| `/` | GET | `hello_world` | Endpoint de prueba/bienvenida |
| `/grupos` | GET | `get_grupos` | Obtener lista de grupos |
| `/abrir-sesion` | GET | `abrir_caja` | Abrir sesión de caja |
| `/generar-qr` | POST | `generar_qr` | Generar código QR para transacción |
| `/cerrar-sesion` | DELETE | `cerrar_caja` | Cerrar sesión de caja |
| `/estado-transaccion` | GET | `handle_transaccion` | Consultar estado de transacción |
| `/retornar-transaccion` | GET | `handle_transaccion` | Retornar/revertir transacción |

## 5. Middlewares Aplicados

### `CorsLayer::permissive()`
- **Propósito:** Permite peticiones CORS desde cualquier origen (modo permisivo)
- **Uso:** Facilita el desarrollo frontend desde diferentes dominios

### `TraceLayer::new_for_http()`
- **Propósito:** Añade logging automático de todas las peticiones HTTP
- **Uso:** Debugging y monitoreo de tráfico

### `CatchPanicLayer::new()`
- **Propósito:** Captura pánico en handlers y previene que el servidor se caiga
- **Uso:** Mejora la resiliencia del servidor

## 6. Dependencias Importadas

```rust
// Framework web
use axum::{Router, routing::{delete, get, post}};

// Middlewares de Tower
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer

Directory: src
# Análisis del Directorio `src`

## Resumen General

Este directorio contiene el **núcleo de una aplicación web de gestión de cajas registradoras** integrada con el sistema de pagos **Yappy**. Es una API REST construida con **Rust**, utilizando el framework **Axum** para el servidor web y **Diesel** como ORM para interactuar con una base de datos **MySQL**.

## Propósito del Sistema

El sistema gestiona:
- **Cajas registradoras** (físicas y virtuales) organizadas por grupos/comercios
- **Sesiones de caja** (apertura y cierre)
- **Transacciones de pago** mediante códigos QR
- **Integración con Yappy** para procesamiento de pagos
- **Cierres automáticos** de cajas mediante tareas programadas
- **Registro de errores** y resúmenes de transacciones

## Arquitectura y Flujo de Trabajo

```
┌─────────────────────────────────────────────────────────────┐
│                         main.rs                              │
│                   (Punto de Entrada)                         │
│                                                              │
│  1. Carga variables de entorno (.env)                       │
│  2. Crea pool de conexiones MySQL                           │
│  3. Inicializa AppState (estado compartido)                 │
│  4. Ejecuta job de cierre de cajas                          │
│  5. Arranca servidor Axum                                   │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ├─────────────────────────────────────────┐
                   │                                         │
                   ▼                                         ▼
         ┌──────────────────┐                    ┌──────────────────┐
         │  start_axum.rs   │                    │    schema.rs     │
         │  (Servidor Web)  │                    │  (Modelo de DB)  │
         │                  │                    │                  │
         │ • Define rutas   │◄───────────────────┤ • Tabla cajas    │
         │ • Middlewares    │                    │ • Tabla grupos   │
         │ • CORS           │                    │ • Tabla kioskos  │
         │ • Logging        │                    │ • Cierres/errores│
         └────────┬─────────┘                    └──────────────────┘
                  │
                  │ Enruta peticiones HTTP
                  │
    ┌─────────────┼─────────────────────────────────┐
    │             │                                 │
    ▼             ▼                                 ▼
┌─────────┐  ┌─────────┐                    ┌──────────────┐
│  GET    │  │  POST   │                    │   DELETE     │
│ /grupos │  │/generar │                    │/cerrar-sesion│
└─────────┘  │  -qr    │                    └──────────────┘
             └─────────┘
                  │
                  │ Todos los handlers reciben
                  ▼
         ┌──────────────────┐
         │    AppState      │
         │                  │
         │  • db_pool       │◄──── Compartido entre
         │  (MySqlPool)     │      todos los módulos
         └──────────────────┘
                  │
                  │ Acceso a base de datos
                  ▼
         ┌──────────────────┐
         │   MySQL DB       │
         │                  │
         │ • cajas          │
         │ • grupos         │
         │ • transacciones  │
         │ • cierres        │
         └──────────────────┘
```

## Cómo Trabajan

File: src\controllers\grupos.rs
# Análisis del archivo `src/controllers/grupos.rs`

## 1. Propósito General
Este archivo implementa un controlador HTTP para gestionar grupos y sus cajas asociadas en una aplicación web construida con Axum. Su función principal es proporcionar un endpoint que devuelve información jerárquica de grupos con sus cajas relacionadas.

## 2. Campos/Variables y sus Propósitos

### En `GrupoConCajas`:
- **`id`** (i32): Identificador único del grupo
- **`id_yappy`** (String): Identificador del grupo en el sistema Yappy (posiblemente integración con servicio de pagos)
- **`nombre`** (String): Nombre descriptivo del grupo
- **`cajas`** (Vec<CajaConKiosko>): Lista de cajas asociadas al grupo
- **Comentados**: `api_key`, `secret_key` - Credenciales de API (actualmente deshabilitadas)

### En `CajaConKiosko`:
- **`id`** (i32): Identificador único de la caja
- **`nombre_caja`** (String): Nombre descriptivo de la caja
- **`tipo`** (String): Tipo de caja (ej: "kiosko", "tradicional")
- **`estado`** (CajasEstadoEnum): Estado actual de la caja (enum personalizado)
- **Comentados**: `token_autorizacion`, `kiosko` - Información adicional deshabilitada

### En `get_grupos`:
- **`state`** (AppState): Estado compartido de la aplicación con pool de conexiones DB
- **`conn`**: Conexión a la base de datos obtenida del pool
- **`all_grupos`**: Vector con todos los grupos de la BD
- **`all_cajas`**: Vector con todas las cajas de la BD
- **`grupos_con_cajas`**: Resultado final con grupos y sus cajas anidadas

## 3. Definiciones de Funciones

### `get_grupos`
```rust
pub async fn get_grupos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode>
```

**Entradas:**
- `State(state)`: Extractor de Axum que proporciona acceso al estado compartido de la aplicación (principalmente el pool de conexiones a BD)

**Salidas:**
- `Result<impl IntoResponse, StatusCode>`: 
  - **Éxito**: JSON con la lista de grupos y sus cajas
  - **Error**: Código de estado HTTP (500 Internal Server Error)

**Propósito:**
1. Obtiene todos los grupos de la base de datos
2. Obtiene todas las cajas de la base de datos
3. Relaciona las cajas con sus grupos correspondientes
4. Construye una estructura jerárquica (grupos → cajas)
5. Devuelve el resultado como JSON

**Flujo de ejecución:**
- Consulta la tabla `grupos` usando Diesel ORM
- Consulta la tabla `cajas` usando Diesel ORM
- Itera sobre los grupos y filtra las cajas por `id_grupo`
- Mapea los datos a las estructuras de respuesta
- Serializa y devuelve como JSON

## 4. Structs y su Importancia

### `GrupoConCajas`
```rust
#[derive(Debug, Serialize)]
pub struct GrupoConCajas
```
**Importancia:** 
- Estructura de respuesta principal del endpoint
- Representa un grupo con todas sus cajas anidadas
- Implementa `Serialize` para conversión automática a JSON
- Oculta información sensible (api_key, secret_key) en la respuesta

### `CajaConKiosko`
```rust
#[derive(Debug, Serialize)]
pub struct CajaConKiosko
```
**Importancia:**
- Representa una caja individual dentro de un grupo
- Estructura simplificada que excluye información sensible (token_

File: src\controllers\mod.rs
# Análisis de `src/controllers/mod.rs`

## 1. Propósito General del Archivo

Este es un **archivo de módulo raíz** (`mod.rs`) para el directorio `controllers`. Su función principal es declarar y exponer los submódulos que componen la capa de controladores de la aplicación. Actúa como punto de entrada organizacional para toda la lógica de control del proyecto.

## 2. Lista de Campos/Variables

**No hay campos ni variables definidos** en este archivo. Solo contiene declaraciones de módulos.

## 3. Definiciones de Funciones

**No hay funciones definidas** en este archivo. Es puramente declarativo.

## 4. Structs/Clases

**No hay structs ni clases definidas** en este archivo.

## 5. Estructura de Módulos Declarados

### `pub mod yappy;`
- **Propósito**: Declara y expone públicamente el módulo `yappy`
- **Ubicación esperada**: `src/controllers/yappy.rs` o `src/controllers/yappy/mod.rs`
- **Función probable**: Controlador principal o lógica relacionada con "Yappy" (posiblemente el nombre de la aplicación o un componente principal)

### `pub mod grupos;`
- **Propósito**: Declara y expone públicamente el módulo `grupos`
- **Ubicación esperada**: `src/controllers/grupos.rs` o `src/controllers/grupos/mod.rs`
- **Función probable**: Controlador para gestionar grupos (creación, modificación, consultas de grupos)

### `pub mod structs;`
- **Propósito**: Declara y expone públicamente el módulo `structs`
- **Ubicación esperada**: `src/controllers/structs.rs` o `src/controllers/structs/mod.rs`
- **Función probable**: Definiciones de estructuras de datos compartidas entre los controladores

## 6. Cómo Encaja en el Proyecto

### Arquitectura del Proyecto

```
src/
├── controllers/
│   ├── mod.rs          ← ESTE ARCHIVO
│   ├── yappy.rs        (o yappy/mod.rs)
│   ├── grupos.rs       (o grupos/mod.rs)
│   └── structs.rs      (o structs/mod.rs)
```

### Rol en la Arquitectura

1. **Capa de Controladores**: Este archivo organiza la capa de controladores, que típicamente maneja:
   - Lógica de negocio
   - Coordinación entre servicios
   - Procesamiento de solicitudes
   - Validación de datos

2. **Patrón de Organización**: Sigue el patrón estándar de Rust para organizar módulos jerárquicamente, permitiendo:
   - Encapsulación de funcionalidad relacionada
   - Exposición controlada de APIs públicas
   - Mejor mantenibilidad del código

3. **Punto de Acceso**: Otros módulos del proyecto pueden importar estos controladores mediante:
   ```rust
   use crate::controllers::yappy;
   use crate::controllers::grupos;
   use crate::controllers::structs;
   ```

### Importancia Estratégica

- **Separación de Responsabilidades**: Mantiene la lógica de control separada de modelos, vistas o servicios
- **Escalabilidad**: Facilita agregar nuevos controladores sin modificar código existente
- **Modularidad**: Cada controlador puede desarrollarse y probarse independientemente
- **Convención sobre Configuración**: Sigue las convenciones idiomáticas de Rust para proyectos bien estructurados

Este archivo es fundamental para la organización del proyecto, actuando como el "índice" de todos los controladores disponibles.

File: src\controllers\yappy.rs
# Análisis del archivo `src/controllers/yappy.rs`

## 1. Propósito General

Este archivo implementa un controlador para integrar el sistema de pagos **Yappy** (sistema de pagos móviles de Panamá) con un sistema de gestión de cajas registradoras. Actúa como intermediario entre las cajas físicas y la API de Yappy, manejando la generación de códigos QR para pagos, apertura/cierre de cajas y seguimiento de transacciones.

## 2. Campos/Variables y sus Propósitos

### Variables de Entorno
- **`YAPPY_ENDPOINT`**: URL base de la API de Yappy para realizar peticiones

### Variables Locales Principales
- **`mac_address`**: Identificador único de la caja registradora basado en su dirección MAC
- **`info`**: Estructura con información de la caja (credenciales, estado, tokens)
- **`transaccion_id`**: ID único de la transacción actual en Yappy
- **`tipo_qr`**: Tipo de código QR a generar ("DYN" dinámico, "HYB" híbrido)
- **`formatted`**: Payload formateado para enviar a la API de Yappy
- **`response_json`**: Respuesta parseada de la API de Yappy
- **`now_in_panama`**: Timestamp actual en zona horaria de Panamá

## 3. Definiciones de Funciones

### `hello_world()`
```rust
pub async fn hello_world() -> Json<Value>
```
- **Entrada**: Ninguna
- **Salida**: JSON con mensaje de bienvenida
- **Propósito**: Endpoint de prueba/health check que retorna la versión del sistema

---

### `abrir_caja()`
```rust
pub async fn abrir_caja(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)>
```
- **Entradas**:
  - `headers`: Encabezados HTTP (contiene mac-address)
  - `state`: Estado compartido de la aplicación (pool de BD, configuración)
- **Salida**: JSON con resultado de apertura de caja
- **Propósito**: Abre una caja registradora en el sistema Yappy, obteniendo token de autorización

---

### `generar_qr()`
```rust
pub async fn generar_qr(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(mut payload): Json<GenerarQR>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)>
```
- **Entradas**:
  - `headers`: Encabezados HTTP con mac-address
  - `state`: Estado de la aplicación
  - `payload`: Datos del pedido (GenerarQR struct)
- **Salida**: JSON con código QR generado y datos de transacción
- **Propósito**: 
  - Verifica si la caja está abierta (si no, la abre automáticamente)
  - Genera un código QR de pago en Yappy
  - Guarda el ID de transacción en la base de datos
  - Formatea la descripción con información del pedido

**Flujo importante**:
1. Valida mac-address
2. Obtiene información de la caja
3. Si está cerrada, la abre automáticamente
4. Formatea el payload con descripción personalizada
5. Llama a API de Yappy para generar QR
6. Actualiza la BD con el transaction ID

---

### `cerrar_caja()`
```rust
pub async fn cerrar_caja(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)>
```
- **Entradas**:
  - `headers`:

Directory: src\controllers
# Resumen del Directorio `src/controllers`

## Propósito General

Este directorio implementa la **capa de controladores** de una aplicación web de gestión de pagos mediante Yappy (sistema de pagos móviles de Panamá) integrado con un sistema de cajas registradoras. Actúa como intermediario entre las peticiones HTTP y la lógica de negocio, coordinando operaciones de pagos, gestión de cajas y organización de grupos.

## Arquitectura y Componentes

```
src/controllers/
├── mod.rs          → Punto de entrada y organización modular
├── yappy.rs        → Controlador principal de integración con Yappy
├── grupos.rs       → Controlador de consulta de grupos y cajas
└── structs.rs      → Estructuras de datos compartidas
```

## Cómo Trabajan en Conjunto

### 1. **mod.rs - Orquestador**
- Expone públicamente los tres módulos principales
- Permite que otros componentes del proyecto importen funcionalidad mediante `use crate::controllers::{yappy, grupos, structs}`
- Mantiene la organización jerárquica del código

### 2. **structs.rs - Definiciones Compartidas**
- Proporciona estructuras de datos comunes (DTOs) utilizadas por los otros controladores
- Define contratos de entrada/salida para los endpoints
- Centraliza la serialización/deserialización de JSON

### 3. **yappy.rs - Controlador Principal**
- **Funcionalidad Core**: Integración completa con la API de Yappy
- **Endpoints implementados**:
  - `hello_world()` - Health check
  - `abrir_caja()` - Apertura de caja registradora
  - `generar_qr()` - Generación de códigos QR para pagos
  - `cerrar_caja()` - Cierre de caja
- **Responsabilidades**:
  - Autenticación con Yappy mediante tokens
  - Gestión del ciclo de vida de cajas (abrir/cerrar)
  - Generación de transacciones de pago
  - Formateo de descripciones de pedidos
  - Manejo de zona horaria (Panamá)
  - Actualización de estados en base de datos

### 4. **grupos.rs - Controlador de Consultas**
- **Funcionalidad**: Proporciona vistas jerárquicas de la organización
- **Endpoint implementado**:
  - `get_grupos()` - Obtiene grupos con sus cajas asociadas
- **Responsabilidades**:
  - Consulta de datos organizacionales
  - Construcción de estructuras anidadas (grupos → cajas)
  - Filtrado de información sensible (API keys, tokens)
  - Serialización de respuestas JSON

## Flujo de Trabajo Típico

### Escenario: Generar un Pago con Yappy

```
1. Cliente solicita generar QR
   ↓
2. yappy.rs::generar_qr() recibe la petición
   ↓
3. Valida mac-address de la caja
   ↓
4. Consulta información de la caja en BD
   ↓
5. Si la caja está cerrada → llama a abrir_caja()
   ↓
6. Formatea payload usando structs de structs.rs
   ↓
7. Llama a API externa de Yappy
   ↓
8. Guarda transaction_id en BD
   ↓
9. Retorna QR y datos al cliente
```

### Escenario: Consultar Organización

```
1. Cliente solicita lista de grupos
   ↓
2. grupos.rs::get_grupos() recibe la petición
   ↓
3. Consulta tabla 'grupos' en BD
   ↓
4. Consulta tabla 'cajas' en BD
   ↓
5.

File: src\controllers\structs\mod.rs
# Análisis de `src\controllers\structs\mod.rs`

## 1. Propósito General del Archivo

Este es un **archivo de módulo** (`mod.rs`) que actúa como punto de entrada y organizador para el submódulo `structs` dentro del directorio `controllers`. Su función principal es declarar y exponer el módulo `yappy` para que pueda ser utilizado por otras partes del proyecto.

## 2. Lista de Campos/Variables

**No hay campos ni variables definidos** en este archivo. Solo contiene declaraciones de módulos.

## 3. Definiciones de Funciones

**No hay funciones definidas** en este archivo.

## 4. Structs/Clases

**No hay structs ni clases definidas** directamente en este archivo.

## 5. Cómo Encaja en el Proyecto

### Estructura Jerárquica

```
src/
└── controllers/
    └── structs/
        ├── mod.rs          <- Este archivo
        └── yappy.rs        <- Módulo declarado aquí
```

### Rol en la Arquitectura

1. **Organizador de Estructuras de Datos**: Este archivo organiza las estructuras de datos (structs) utilizadas por los controladores de la aplicación.

2. **Punto de Acceso**: Permite que otros módulos accedan al contenido de `yappy` mediante:
   ```rust
   use crate::controllers::structs::yappy;
   // o
   use crate::controllers::structs::yappy::AlgunaStruct;
   ```

3. **Separación de Responsabilidades**: Mantiene las definiciones de estructuras separadas de la lógica de los controladores, siguiendo principios de diseño limpio.

### Patrón de Diseño

Este archivo sigue el **patrón de módulos de Rust**, donde:
- `mod.rs` actúa como índice del directorio
- `pub mod yappy;` hace público el módulo `yappy`
- Facilita la modularidad y encapsulación del código

### Contexto del Proyecto

Basándome en la estructura, este proyecto parece:
- Usar una arquitectura MVC o similar (presencia de `controllers`)
- Tener un componente o servicio llamado "yappy"
- Organizar las estructuras de datos de forma modular y reutilizable

**Nota**: Para un análisis más completo, sería necesario revisar el contenido de `yappy.rs` para entender qué estructuras específicas se están definiendo y cómo se utilizan en los controladores.

File: src\controllers\structs\yappy.rs
# Análisis de `src/controllers/structs/yappy.rs`

## 1. Propósito General del Archivo

Este archivo define las estructuras de datos (structs) necesarias para interactuar con la API de **Yappy** (sistema de pagos móviles de Panamá). Proporciona modelos de serialización/deserialización para dos operaciones principales:
- **Abrir una caja registradora** (dispositivo de punto de venta)
- **Generar códigos QR** para cobros

## 2. Lista de Campos/Variables y sus Propósitos

### Función `default_f64()`
- **Retorno**: `0.0` - Valor por defecto para campos numéricos opcionales

### Struct `RootPayload`
- `body: Body` - Contenedor principal del payload para abrir caja

### Struct `Body`
- `device: Device` - Información del dispositivo/caja
- `group_id: String` - Identificador del grupo al que pertenece la caja

### Struct `Device`
- `id: String` - Identificador único del dispositivo
- `name: Option<String>` - Nombre opcional del dispositivo (se omite si es None)
- `user: Option<String>` - Usuario/cajero opcional (se omite si es None)

### Struct `AbrirCaja`
- `id_caja: String` - Identificador de la caja registradora
- `id_grupo: String` - Identificador del grupo comercial
- `nombre_caja: Option<String>` - Nombre descriptivo de la caja
- `nombre_cajero: Option<String>` - Nombre del cajero operando la caja

### Struct `GenerarQR`
- `tipo_qr: String` - Tipo de código QR a generar
- `subtotal: f64` - Monto antes de impuestos/propinas
- `total: f64` - Monto total a cobrar
- `impuesto: f64` - Monto de impuestos (default: 0.0)
- `propina: f64` - Monto de propina (default: 0.0)
- `descuento: f64` - Monto de descuento aplicado (default: 0.0)
- `id_orden: Option<String>` - Identificador opcional de la orden
- `descripcion: Option<String>` - Descripción opcional del cobro

### Struct `RootPayloadQR`
- `body: BodyGenerarQR` - Contenedor principal del payload para generar QR

### Struct `BodyGenerarQR`
- `charge_amount: ChargeAmount` - Desglose de montos del cobro
- `order_id: Option<String>` - ID de orden opcional
- `description: Option<String>` - Descripción opcional

### Struct `ChargeAmount`
- `sub_total: f64` - Subtotal del cobro
- `tax: f64` - Impuestos
- `tip: f64` - Propina
- `discount: f64` - Descuento
- `total: f64` - Total final

## 3. Definiciones de Funciones

### `default_f64() -> f64`
- **Entrada**: Ninguna
- **Salida**: `f64` con valor `0.0`
- **Propósito**: Proporcionar un valor por defecto para campos numéricos durante la deserialización

### `AbrirCaja::to_payload(&self) -> RootPayload`
- **Entrada**: `&self` - Referencia a la instancia de `AbrirCaja`
- **Salida**: `RootPayload` - Estructura formateada para la API de Yappy
- **Propósito**: Transformar los datos de apertura de caja al formato requerido por la API, mapeando campos en español a la estructura esperada

### `GenerarQR::to_payload(&self) -> RootPayloadQR`
- **Entrada

Directory: src\controllers\structs
# Resumen del Directorio `src/controllers/structs`

## Propósito General

Este directorio contiene las **definiciones de estructuras de datos** utilizadas por los controladores para interactuar con servicios externos de pago. Actualmente se enfoca en la integración con **Yappy**, el sistema de pagos móviles de Panamá.

## Arquitectura y Organización

```
src/controllers/structs/
├── mod.rs          → Punto de entrada y organizador del módulo
└── yappy.rs        → Estructuras específicas para la API de Yappy
```

### Flujo de Trabajo

1. **`mod.rs`** actúa como **índice público** que expone el módulo `yappy` al resto de la aplicación
2. **`yappy.rs`** define **dos flujos principales de datos**:
   - **Apertura de Caja Registradora** (`AbrirCaja` → `RootPayload`)
   - **Generación de Códigos QR** (`GenerarQR` → `RootPayloadQR`)

## Funcionalidad Conjunta

### 1. Transformación de Datos (Patrón Adapter)

Las estructuras trabajan en **pares de transformación**:

```
Entrada (español) → Método to_payload() → Salida (formato API)
```

**Ejemplo - Apertura de Caja:**
```rust
AbrirCaja {                    RootPayload {
  id_caja,          →            body: Body {
  id_grupo,         →              device: Device { id, name, user },
  nombre_caja,      →              group_id
  nombre_cajero     →            }
}                              }
```

**Ejemplo - Generación de QR:**
```rust
GenerarQR {                    RootPayloadQR {
  subtotal,         →            body: BodyGenerarQR {
  total,            →              charge_amount: ChargeAmount {
  impuesto,         →                sub_total, tax, tip,
  propina,          →                discount, total
  descuento         →              }
}                              }
```

### 2. Serialización/Deserialización

Todas las estructuras implementan:
- **`Serialize`**: Para convertir a JSON al enviar a la API
- **`Deserialize`**: Para recibir respuestas de la API
- **`Debug`**: Para facilitar el debugging

### 3. Manejo de Valores Opcionales

- Usa `#[serde(skip_serializing_if = "Option::is_none")]` para omitir campos nulos
- Implementa `default_f64()` para valores numéricos por defecto (0.0)

## Rol en el Proyecto

Este directorio actúa como **capa de abstracción de datos** entre:

```
Controladores (lógica de negocio)
         ↓
    structs/ (este directorio)
         ↓
    API de Yappy (servicio externo)
```

### Beneficios de esta Arquitectura:

1. **Separación de Responsabilidades**: Las estructuras están aisladas de la lógica de controladores
2. **Reutilización**: Múltiples controladores pueden usar las mismas estructuras
3. **Mantenibilidad**: Cambios en la API de Yappy solo requieren modificar este directorio
4. **Tipado Fuerte**: Rust garantiza que los datos enviados cumplan con el formato esperado
5. **Escalabilidad**: Fácil agregar nuevos servicios de pago (ej: `stripe.rs`, `paypal.rs`)

## Patrón de Diseño Aplicado

**Data Transfer Object (DTO)** + **Adapter Pattern**:
- Las estructuras actúan como DTOs para transferir datos
- Los métodos `to_payload()` adaptan el formato interno al formato de la API externa

Este diseño facilita la integración con servicios de pago manteniendo el código

File: src\db\conection.rs
# Análisis de `src\db\conection.rs`

## 1. Propósito General del Archivo

Este archivo gestiona la **configuración y creación del pool de conexiones a la base de datos MySQL**. Proporciona una abstraction para manejar múltiples conexiones concurrentes a la base de datos de manera eficiente mediante el patrón de pool de conexiones, utilizando las bibliotecas Diesel (ORM) y R2D2 (gestor de pools).

## 2. Campos/Variables y sus Propósitos

### Variables Locales en `create_pool()`:

| Variable | Tipo | Propósito |
|----------|------|-----------|
| `db_url` | `String` | Almacena la URL de conexión a la base de datos MySQL obtenida de la variable de entorno `DATABASE_URL` |
| `manager` | `ConnectionManager<MysqlConnection>` | Gestor de conexiones que maneja el ciclo de vida de las conexiones individuales a MySQL |

## 3. Definiciones de Funciones

### `create_pool()`

```rust
pub fn create_pool() -> MySqlPool
```

**Entradas:** Ninguna (pero requiere variable de entorno `DATABASE_URL`)

**Salida:** `MySqlPool` - Un pool de conexiones a MySQL

**Propósito:** 
- Lee la URL de conexión desde las variables de entorno
- Crea un gestor de conexiones para MySQL
- Construye y retorna un pool de conexiones reutilizables
- Termina el programa con `panic!` si falla la configuración

**Flujo de ejecución:**
1. Obtiene `DATABASE_URL` del entorno (falla si no existe)
2. Crea un `ConnectionManager` con la URL
3. Construye el pool usando el patrón builder
4. Retorna el pool o termina con error

## 4. Structs/Clases y su Importancia

### Type Alias: `MySqlPool`

```rust
pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
```

**Importancia:**
- **Simplificación de tipos**: Crea un alias legible para el tipo complejo del pool
- **Reutilización**: Permite usar `MySqlPool` en lugar del tipo genérico completo en todo el proyecto
- **Mantenibilidad**: Centraliza la definición del tipo de pool, facilitando cambios futuros
- **Claridad**: Hace el código más expresivo y fácil de entender

## 5. Cómo Encaja en el Proyecto

### Rol en la Arquitectura:

```
┌─────────────────────────────────────┐
│   Capa de Aplicación/Handlers       │
│   (Rutas, Controladores)            │
└──────────────┬──────────────────────┘
               │
               ↓ usa MySqlPool
┌─────────────────────────────────────┐
│   src/db/conection.rs               │ ← ESTE ARCHIVO
│   - create_pool()                   │
│   - MySqlPool type alias            │
└──────────────┬──────────────────────┘
               │
               ↓ gestiona conexiones
┌─────────────────────────────────────┐
│   Base de Datos MySQL               │
└─────────────────────────────────────┘
```

### Funciones Clave:

1. **Inicialización de la aplicación**: Se llama típicamente al inicio para crear el pool compartido
2. **Inyección de dependencias**: El pool se pasa a handlers/servicios que necesitan acceso a BD
3. **Gestión de recursos**: Maneja automáticamente la apertura/cierre de conexiones
4. **Concurrencia**: Permite múltiples requests simultáneos sin crear nuevas conexiones cada vez

### Dependencias:

- **Diesel**: ORM para interactuar con bases de datos
- **R2D2**:

File: src\db\mod.rs
# Análisis de `src\db\mod.rs`

## 1. Propósito General del Archivo

Este archivo es un **módulo organizador** (module file) en Rust que actúa como punto de entrada para el submódulo `db` (database). Su función principal es declarar y exponer los submódulos relacionados con la base de datos del proyecto, organizando la funcionalidad de acceso a datos en componentes lógicos separados.

## 2. Lista de Campos/Variables

**No hay campos ni variables definidos en este archivo.** Es puramente declarativo.

## 3. Definiciones de Funciones

**No hay funciones definidas en este archivo.** Solo contiene declaraciones de módulos.

## 4. Structs/Clases

**No hay structs, enums ni traits definidos directamente en este archivo.**

Sin embargo, declara tres submódulos que probablemente contienen estas estructuras:

### Submódulos Declarados:

- **`conection`** (nota: posible error tipográfico, debería ser "connection")
  - Probablemente contiene la lógica para establecer y gestionar conexiones con la base de datos
  - Puede incluir pools de conexiones, configuración de conexión, etc.

- **`types`**
  - Probablemente define tipos personalizados relacionados con la base de datos
  - Puede incluir wrappers de tipos SQL, conversiones de tipos, o tipos específicos del dominio

- **`models`**
  - Probablemente contiene las estructuras de datos (structs) que representan las tablas/entidades de la base de datos
  - Típicamente incluye modelos ORM o estructuras que mapean a registros de base de datos

## 5. Cómo Encaja en el Proyecto

### Arquitectura del Proyecto:

```
src/
├── db/
│   ├── mod.rs          ← Este archivo (punto de entrada del módulo db)
│   ├── conection.rs    ← Gestión de conexiones
│   ├── types.rs        ← Tipos personalizados
│   └── models.rs       ← Modelos de datos
```

### Rol en el Proyecto:

1. **Organización Modular**: Agrupa toda la funcionalidad relacionada con la base de datos en un único namespace `db`

2. **Separación de Responsabilidades**: 
   - Conexión (infraestructura)
   - Tipos (conversiones y definiciones)
   - Modelos (representación de datos)

3. **Punto de Acceso Centralizado**: Otros módulos del proyecto pueden importar funcionalidad de base de datos mediante:
   ```rust
   use crate::db::conection::*;
   use crate::db::models::*;
   use crate::db::types::*;
   ```

4. **Patrón Común en Rust**: Este es el patrón estándar para organizar módulos en proyectos Rust, donde `mod.rs` actúa como el "índice" del módulo.

### Recomendación:

Considera corregir el error tipográfico: `conection` → `connection` para mantener la consistencia con la nomenclatura estándar en inglés.

File: src\db\models.rs
# Análisis de `src/db/models.rs`

## 1. Propósito General del Archivo

Este archivo define los **modelos de datos** (estructuras) que representan las tablas de la base de datos MySQL del sistema. Actúa como la capa de mapeo objeto-relacional (ORM) usando Diesel, permitiendo interactuar con la base de datos de forma tipada y segura. El sistema parece gestionar **cajas registradoras, kioskos y grupos** para procesamiento de pagos (posiblemente con Yappy).

## 2. Lista de Campos/Variables y sus Propósitos

### Importaciones
- `crate::schema::*`: Esquemas de tablas generados por Diesel
- `diesel::prelude::*`: Funcionalidades principales de Diesel ORM
- `serde`: Serialización/deserialización JSON
- `thiserror::Error`: Manejo de errores personalizado
- `CajasEstadoEnum`: Enumeración de estados de caja
- `BigDecimal`: Manejo preciso de valores monetarios
- `serde_json::Value`: Valores JSON dinámicos

### Struct `Kiosko`
- **`id`** (i32): Identificador único del kiosko
- **`id_caja`** (i32): Clave foránea a la caja asociada
- **`nombre`** (String): Nombre descriptivo del kiosko
- **`mac_address`** (String): Dirección MAC para identificación física

### Struct `Caja`
- **`id`** (i32): Identificador único de la caja
- **`id_grupo`** (i32): Clave foránea al grupo propietario
- **`nombre_caja`** (String): Nombre de la caja registradora
- **`tipo`** (String): Tipo/categoría de caja
- **`token_autorizacion`** (Option<String>): Token de seguridad opcional
- **`transaccion_actual`** (Option<String>): ID de transacción en curso
- **`estado`** (CajasEstadoEnum): Estado actual (abierta, cerrada, etc.)

### Struct `Grupo`
- **`id`** (i32): Identificador único del grupo
- **`id_yappy`** (String): Identificador en el sistema Yappy
- **`nombre`** (String): Nombre del grupo/comercio
- **`api_key`** (String): Clave API para autenticación
- **`secret_key`** (String): Clave secreta para firma de peticiones

### Struct `NewCajaCierreResumen`
- **`id_caja`** (i32): Caja a la que pertenece el resumen
- **`tipo`** (String): Tipo de transacción/método de pago
- **`monto`** (BigDecimal): Monto total acumulado
- **`transacciones`** (i32): Cantidad de transacciones

### Struct `NewCajaCierreError`
- **`id_caja`** (i32): Caja donde ocurrió el error
- **`respuesta_json`** (Value): Respuesta completa del error en JSON

### Struct `ErrorResponse`
- **`error`** (String): Mensaje de error para respuestas API

## 3. Definiciones de Funciones

**No hay funciones explícitas** en este archivo. Solo contiene definiciones de estructuras de datos. Las funciones se generan automáticamente por Diesel mediante los traits derivados.

## 4. Structs/Clases y su Importancia

### **`DbError`** (Enum de Error)
- **Propósito**: Manejo centralizado de errores de base de datos
- **Importancia**: Proporciona conversión automática de errores Diesel a errores del dominio
- **Variante**: `DbError` - Envuelve errores de Diesel

### **`ErrorResponse`** (Struct de Respuesta)
-

Directory: src\db
# Resumen del Directorio `src/db`

## Propósito General

El directorio `src/db` constituye la **capa de acceso a datos** del proyecto, encapsulando toda la lógica relacionada con la persistencia y gestión de información en una base de datos MySQL. Implementa un sistema completo de ORM (Object-Relational Mapping) utilizando Diesel, diseñado para gestionar un sistema de **cajas registradoras, kioskos y grupos comerciales** integrados con el sistema de pagos Yappy.

## Arquitectura y Flujo de Trabajo

```
┌─────────────────────────────────────────────────────────────┐
│                    Capa de Aplicación                       │
│              (Handlers, Servicios, Rutas)                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓ Solicita conexión
┌─────────────────────────────────────────────────────────────┐
│  mod.rs - Punto de Entrada del Módulo                      │
│  • Organiza y expone submódulos                             │
│  • Proporciona namespace unificado: crate::db::*            │
└────────┬──────────────┬──────────────┬──────────────────────┘
         │              │              │
         ↓              ↓              ↓
    conection.rs    models.rs      types.rs
    ┌──────────┐   ┌──────────┐   ┌──────────┐
    │ MySqlPool│   │ Structs  │   │ Tipos    │
    │ create_  │   │ • Caja   │   │ Custom   │
    │ pool()   │   │ • Kiosko │   │          │
    │          │   │ • Grupo  │   │          │
    └────┬─────┘   └────┬─────┘   └──────────┘
         │              │
         └──────┬───────┘
                ↓
    ┌───────────────────────┐
    │   Base de Datos MySQL │
    │   • Tabla cajas       │
    │   • Tabla kioskos     │
    │   • Tabla grupos      │
    └───────────────────────┘
```

## Componentes y sus Responsabilidades

### 1. **`mod.rs`** - Organizador del Módulo
- **Rol**: Punto de entrada y organizador
- **Función**: Declara y expone los submódulos (`conection`, `types`, `models`)
- **Importancia**: Permite acceso limpio desde otras partes del proyecto

### 2. **`conection.rs`** - Gestor de Conexiones
- **Rol**: Infraestructura de conexión
- **Responsabilidades**:
  - Crear y configurar el pool de conexiones MySQL
  - Gestionar el ciclo de vida de conexiones
  - Proporcionar conexiones reutilizables para operaciones concurrentes
- **Componentes clave**:
  - `MySqlPool`: Alias de tipo para el pool
  - `create_pool()`: Función de inicialización

### 3. **`models.rs`** - Modelos de Datos
- **Rol**: Capa de mapeo objeto-relacional (ORM)
- **Responsabilidades**:
  - Definir estructuras que representan tablas de BD
  - Proporcionar serialización/deserialización JSON
  - Implementar validaciones y conversiones de tipos
  - Manejar errores de base de datos
- **Entidades principales**:
  - `Caja`: Cajas registradoras con estados y tokens
  - `Kiosko`: Dispositivos físicos identificados por MAC
  - `Grupo`: Comercios con credenciales Yappy
  - Estructuras auxiliares para cierres y err

File: src\db\types\enums.rs
# Análisis de `src/db/types/enums.rs`

## 1. Propósito General del Archivo

Este archivo define tipos enumerados (enums) personalizados que se utilizan como tipos de datos en la base de datos. Específicamente, define estados para el sistema de cajas, permitiendo representar y persistir el estado de una caja registradora o punto de venta en la base de datos.

## 2. Campos/Variables y sus Propósitos

### Variantes del Enum `CajasEstadoEnum`:

- **`Cerrado`**: Representa el estado de una caja cuando está cerrada/inactiva. No se pueden realizar operaciones de venta.

- **`Abierto`**: Representa el estado de una caja cuando está abierta/activa. Permite realizar operaciones de venta y transacciones.

## 3. Definiciones de Funciones

**No hay funciones explícitas definidas en este archivo.** Sin embargo, los derives automáticamente generan métodos:

### Métodos Generados Automáticamente:

- **`Debug`**: Permite imprimir el enum para depuración
- **`Serialize`**: Convierte el enum a formato JSON/serializable
- **`Deserialize`**: Convierte desde JSON al enum
- **`Clone`**: Permite clonar instancias del enum
- **`PartialEq`**: Permite comparar dos instancias del enum
- **`DbEnum`** (de Diesel): Genera código para mapear el enum a tipos de base de datos PostgreSQL/MySQL

## 4. Structs/Clases y su Importancia

### `CajasEstadoEnum` (Enum)

```rust
#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CajasEstadoEnum
```

**Importancia:**
- **Tipo de Base de Datos**: Se mapea directamente a un tipo ENUM en la base de datos
- **Seguridad de Tipos**: Garantiza que solo valores válidos (`Cerrado` o `Abierto`) puedan ser asignados
- **Serialización**: El atributo `rename_all = "lowercase"` asegura que en JSON se represente como "cerrado" o "abierto" (minúsculas)
- **Integración con Diesel ORM**: Permite usar este enum directamente en consultas y modelos de base de datos

**Decoradores/Atributos:**
- `diesel_derive_enum::DbEnum`: Integración con Diesel ORM para tipos de base de datos
- `Debug`: Para debugging
- `Serialize/Deserialize`: Para conversión JSON (APIs REST)
- `Clone`: Para duplicar valores
- `PartialEq`: Para comparaciones
- `serde(rename_all = "lowercase")`: Normaliza la serialización a minúsculas

## 5. Cómo Encaja en el Proyecto

### Contexto en la Arquitectura:

```
Proyecto (Sistema de Punto de Venta/POS)
│
├── src/db/types/enums.rs  ← ESTE ARCHIVO
│   └── Define tipos de datos personalizados para la BD
│
├── src/db/models/
│   └── Usa CajasEstadoEnum en el modelo de Cajas
│
├── src/db/schema.rs
│   └── Define la estructura de tablas que usan estos enums
│
└── src/api/
    └── Serializa/deserializa estos estados en endpoints REST
```

### Integración Específica:

1. **Capa de Base de Datos**: Define el tipo de dato que se almacena en la columna `estado` de la tabla `cajas`

2. **Modelos de Datos**: Probablemente usado en un struct como:
   ```rust
   struct Caja {
       id: i32,
       estado: CajasEstadoEnum,  // ← Usa

File: src\db\types\mod.rs
# Análisis de `src\db\types\mod.rs`

## 1. Propósito General del Archivo

Este es un archivo de **módulo organizador** en Rust que actúa como punto de entrada para el submódulo `types` dentro del directorio `db`. Su función principal es declarar y exponer el submódulo `enums`, haciendo que sus contenidos sean accesibles para otras partes del proyecto.

## 2. Lista de Campos/Variables

**No hay campos ni variables definidos** en este archivo. Solo contiene una declaración de módulo.

## 3. Definiciones de Funciones

**No hay funciones definidas** en este archivo.

## 4. Structs/Clases

**No hay structs, enums ni traits definidos** en este archivo.

## 5. Cómo Encaja en el Proyecto

### Estructura Jerárquica

```
src/
└── db/
    └── types/
        ├── mod.rs          <- Este archivo
        └── enums.rs        <- Módulo declarado aquí
```

### Rol en la Arquitectura

1. **Organización de Tipos de Base de Datos**: Este archivo organiza los tipos personalizados relacionados con la capa de base de datos del proyecto.

2. **Punto de Acceso Centralizado**: Permite que otros módulos importen tipos desde `db::types::enums` en lugar de tener que conocer la estructura interna completa.

3. **Patrón de Módulos Rust**: Sigue el patrón estándar de Rust donde `mod.rs` actúa como el archivo raíz de un directorio-módulo.

### Uso Típico en Otros Archivos

```rust
// Desde otros archivos del proyecto:
use crate::db::types::enums::AlgunEnum;

// O con re-exportación:
// Si mod.rs tuviera: pub use enums::*;
use crate::db::types::AlgunEnum;
```

### Propósito Estratégico

- **Separación de Concerns**: Mantiene los tipos de enumeración relacionados con la base de datos en un módulo dedicado
- **Escalabilidad**: Facilita agregar más submódulos de tipos (como `structs.rs`, `traits.rs`, etc.) en el futuro
- **Encapsulación**: Controla qué tipos son públicos y accesibles desde fuera del módulo `types`

### Contexto del Proyecto

Dado que está en `src/db/types/`, este archivo es parte de la **capa de abstracción de base de datos** del proyecto, probablemente conteniendo:
- Enumeraciones para estados de base de datos
- Tipos personalizados para columnas
- Wrappers de tipos para validación
- Conversiones entre tipos de Rust y tipos de BD

Directory: src\db\types
# Análisis del Directorio `src/db/types`

## Propósito General del Directorio

Este directorio actúa como **capa de definición de tipos personalizados para la base de datos**, proporcionando abstracciones type-safe que mapean directamente a tipos de datos específicos en el sistema de gestión de base de datos (PostgreSQL/MySQL). Su función principal es centralizar y estandarizar los tipos de datos personalizados que se utilizan en todo el proyecto para garantizar consistencia y seguridad de tipos.

## Arquitectura y Organización

```
src/db/types/
│
├── mod.rs                    # Orquestador del módulo
│   └── Declara y expone submódulos
│
└── enums.rs                  # Definiciones de enumeraciones
    └── CajasEstadoEnum       # Estados de caja (Cerrado/Abierto)
```

## Cómo Trabajan en Conjunto los Componentes

### 1. **Flujo de Declaración y Exposición**

```rust
// mod.rs declara:
pub mod enums;

// Esto permite en otros archivos:
use crate::db::types::enums::CajasEstadoEnum;
```

### 2. **Integración con la Capa de Base de Datos**

```
┌─────────────────────────────────────────────────┐
│          CAPA DE APLICACIÓN                     │
│  (Lógica de negocio, APIs, Controladores)       │
└────────────────┬────────────────────────────────┘
                 │
                 ↓ Usa tipos seguros
┌─────────────────────────────────────────────────┐
│       src/db/types/  ← ESTE DIRECTORIO          │
│  ┌──────────────────────────────────┐           │
│  │ enums.rs                          │           │
│  │  - CajasEstadoEnum                │           │
│  │    • Cerrado                      │           │
│  │    • Abierto                      │           │
│  └──────────────────────────────────┘           │
└────────────────┬────────────────────────────────┘
                 │
                 ↓ Mapea a tipos DB
┌─────────────────────────────────────────────────┐
│         CAPA DE PERSISTENCIA                    │
│  (Diesel ORM, Schema, Modelos)                  │
│                                                  │
│  CREATE TYPE cajas_estado AS ENUM               │
│    ('cerrado', 'abierto');                      │
└─────────────────────────────────────────────────┘
```

### 3. **Ejemplo de Uso Completo en el Proyecto**

```rust
// 1. Definición del tipo (enums.rs)
pub enum CajasEstadoEnum {
    Cerrado,
    Abierto,
}

// 2. Uso en el schema (src/db/schema.rs)
table! {
    cajas (id) {
        id -> Int4,
        estado -> CajasEstadoEnum,  // ← Tipo personalizado
        // ...
    }
}

// 3. Uso en modelos (src/db/models/caja.rs)
#[derive(Queryable)]
pub struct Caja {
    pub id: i32,
    pub estado: CajasEstadoEnum,  // ← Type-safe
}

// 4. Uso en lógica de negocio (src/services/caja_service.rs)
fn abrir_caja(caja: &mut Caja) -> Result<()> {
    if caja.estado == CajasEstadoEnum::Cerrado {
        caja.estado = CajasEstadoEnum::Abierto;
        Ok(())
    } else {
        Err("Caja ya está abierta

File: src\schedulers\cajas.rs
# Análisis de `src/schedulers/cajas.rs`

## 1. Propósito General del Archivo

Este archivo implementa un **programador de tareas (scheduler)** que cierra automáticamente las cajas registradoras abiertas a las 11:00 PM (23:00) hora de Panamá. Es parte de un sistema de gestión de cajas que requiere cerrar operaciones diarias de forma automática.

## 2. Campos/Variables y sus Propósitos

### Struct `CajaWithCreds`:
- **`id`** (`i32`): Identificador único de la caja
- **`nombre_caja`** (`String`): Nombre descriptivo de la caja registradora
- **`estado`** (`CajasEstadoEnum`): Estado actual de la caja (Abierto/Cerrado)
- **`api_key`** (`String`): Clave API del grupo al que pertenece la caja
- **`secret_key`** (`String`): Clave secreta para autenticación
- **`token_autorizacion`** (`Option<String>`): Token opcional de autorización

### Variables en `cerrar_cajas_job`:
- **`scheduler`**: Instancia del programador de tareas cron
- **`state`**: Estado compartido de la aplicación (clonado para uso en closures)
- **`cerrar_caja_job`**: Definición del trabajo programado
- **`conn`**: Conexión a la base de datos
- **`cajas_with_keys`**: Vector con todas las cajas abiertas y sus credenciales
- **`now_in_panama`**: Timestamp actual en zona horaria de Panamá

## 3. Definiciones de Funciones

### `cerrar_cajas_job`

**Firma:**
```rust
pub async fn cerrar_cajas_job(state: &AppState) -> Result<(), Box<dyn std::error::Error>>
```

**Entradas:**
- `state: &AppState` - Referencia al estado global de la aplicación (incluye pool de conexiones DB)

**Salidas:**
- `Result<(), Box<dyn std::error::Error>>` - Ok si el scheduler se inició correctamente, Error en caso contrario

**Propósito:**
Configura y ejecuta un trabajo programado que:
1. Se ejecuta diariamente a las 23:00 (11 PM) hora de Panamá
2. Consulta todas las cajas con estado "Abierto"
3. Cierra cada caja llamando a `guardar_datos_caja`
4. Registra en consola las operaciones realizadas

**Lógica interna:**
- Crea un scheduler con zona horaria de Panamá
- Define un cron job con expresión `"0 0 23 * * *"` (23:00 diario)
- En cada ejecución:
  - Obtiene conexión a BD
  - Consulta cajas abiertas con JOIN a tabla grupos
  - Itera sobre cada caja y la cierra
  - Imprime logs con timestamp en formato de Panamá

## 4. Structs/Clases y su Importancia

### `CajaWithCreds`

**Derivaciones:**
- `Queryable`: Permite mapear resultados de consultas Diesel
- `Debug`: Facilita debugging
- `serde::Serialize`: Permite serialización (aunque no se usa directamente aquí)

**Importancia:**
- Estructura de datos que combina información de dos tablas (`cajas` y `grupos`)
- Contiene toda la información necesaria para cerrar una caja:
  - Identificación (id, nombre)
  - Estado actual
  - Credenciales de autenticación (api_key, secret_key, token)
- Facilita el paso de datos entre la consulta SQL y la función de cierre

## 5. Integración en el Proyecto

### Rol en la Arquitectura:

**Capa de Automatización:**
- Este

File: src\schedulers\mod.rs
# Análisis de `src/schedulers/mod.rs`

## 1. Propósito General del Archivo

Este archivo actúa como un **módulo organizador** (module declaration file) en Rust. Su función principal es declarar y exponer el submódulo `cajas` dentro del módulo `schedulers`. Es un archivo de configuración de módulos típico en la estructura de proyectos Rust.

## 2. Lista de Campos/Variables

**No hay campos ni variables definidos** en este archivo. Solo contiene declaraciones de módulos.

## 3. Definiciones de Funciones

**No hay funciones definidas** en este archivo.

## 4. Structs/Clases

**No hay structs, enums, traits ni otras estructuras de datos** definidas en este archivo.

## 5. Cómo Encaja en el Proyecto

### Estructura del Proyecto

```
src/
└── schedulers/
    ├── mod.rs          <- Este archivo
    └── cajas.rs        <- Módulo declarado aquí
```

### Rol en la Arquitectura

1. **Punto de Entrada del Módulo**: Este archivo sirve como el punto de entrada para el módulo `schedulers`. Cuando otras partes del proyecto importan `schedulers`, Rust busca este archivo `mod.rs`.

2. **Organización Jerárquica**: Permite organizar el código relacionado con "schedulers" (planificadores/programadores) en un namespace coherente.

3. **Exposición de Submódulos**: La declaración `pub mod cajas;` hace que el módulo `cajas` sea:
   - **Público** (`pub`): Accesible desde fuera del módulo `schedulers`
   - **Disponible**: Carga el contenido del archivo `cajas.rs`

### Uso Típico

Otras partes del proyecto pueden acceder al módulo `cajas` de estas formas:

```rust
// Desde la raíz del proyecto
use crate::schedulers::cajas;

// O importar elementos específicos
use crate::schedulers::cajas::AlgunaEstructura;
```

### Contexto del Proyecto

Basándome en el nombre del módulo (`schedulers` y `cajas`), este archivo probablemente forma parte de un sistema que:
- Gestiona **planificación o programación** de tareas
- Trabaja con **cajas** (boxes/containers) que necesitan ser programadas o gestionadas
- Podría ser parte de un sistema de logística, almacenamiento o gestión de recursos

### Escalabilidad

Este patrón permite fácil expansión. Si se necesitan más schedulers, simplemente se agregarían más líneas:

```rust
pub mod cajas;
pub mod pedidos;  // Futuro
pub mod rutas;    // Futuro
```

## Resumen

Este es un archivo **minimalista pero esencial** que cumple una función organizativa crítica en la arquitectura del proyecto Rust, permitiendo la modularización y encapsulación del código relacionado con schedulers.

Directory: src\schedulers
# Análisis del Directorio `src/schedulers`

## Propósito General

Este directorio implementa la **capa de automatización y tareas programadas** del sistema. Su función principal es ejecutar operaciones periódicas sin intervención manual, específicamente el **cierre automático de cajas registradoras** al final del día laboral.

## Estructura y Organización

```
src/schedulers/
├── mod.rs          → Punto de entrada y organización del módulo
└── cajas.rs        → Implementación del scheduler de cierre de cajas
```

### Arquitectura de Componentes

```
┌─────────────────────────────────────────────────────────┐
│                    schedulers/                          │
│                                                         │
│  ┌──────────────┐         ┌─────────────────────────┐ │
│  │   mod.rs     │────────▶│      cajas.rs           │ │
│  │              │         │                         │ │
│  │ - Expone     │         │ - CajaWithCreds         │ │
│  │   módulos    │         │ - cerrar_cajas_job()    │ │
│  └──────────────┘         │ - Cron: 23:00 diario    │ │
│                           └─────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
                    │
                    ▼
        ┌───────────────────────────┐
        │   Integración Externa     │
        ├───────────────────────────┤
        │ • Base de Datos (Diesel)  │
        │ • AppState (Pool de DB)   │
        │ • Zona horaria: Panamá    │
        │ • Función: guardar_datos  │
        └───────────────────────────┘
```

## Flujo de Trabajo

### 1. **Inicialización** (al arrancar la aplicación)
```rust
// En main.rs o similar
use schedulers::cajas::cerrar_cajas_job;

cerrar_cajas_job(&app_state).await?;
```

### 2. **Ejecución Programada** (diariamente a las 23:00)
```
23:00 Panamá
    │
    ▼
┌─────────────────────────────────────┐
│ Scheduler se activa                 │
└─────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────┐
│ Consulta BD: Cajas ABIERTAS         │
│ JOIN con tabla grupos               │
│ SELECT id, nombre, estado,          │
│        api_key, secret_key, token   │
└─────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────┐
│ Para cada caja abierta:             │
│   1. Obtener credenciales           │
│   2. Llamar guardar_datos_caja()    │
│   3. Cambiar estado a CERRADO       │
│   4. Log de operación               │
└─────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────┐
│ Fin del ciclo - Esperar próximo día │
└─────────────────────────────────────┘
```

## Componentes Clave

### **mod.rs** - Organizador
- **Responsabilidad**: Exponer el módulo `cajas` públicamente
- **Patrón**: Module declaration pattern de Rust
- **Escalabilidad**: Permite agregar más schedulers fácilmente

### **cajas.rs** - Implementación
- **Responsabilidad**: Lógica de cierre automático de cajas
- **Tecnolog

File: src\utils\cajas_utils.rs
# Análisis de `src/utils/cajas_utils.rs`

## 1. Propósito General del Archivo

Este archivo contiene utilidades para gestionar el ciclo de vida de las cajas registradoras integradas con el sistema de pagos **Yappy**. Maneja operaciones como:
- Apertura de cajas
- Cierre de cajas con resumen de transacciones
- Gestión de respuestas de transacciones
- Registro de errores de cierre
- Actualización de estados de cajas en la base de datos

## 2. Campos/Variables y sus Propósitos

### Variables de Entorno
- **`YAPPY_ENDPOINT`**: URL base del API de Yappy para realizar operaciones de sesión y transacciones

### Variables Locales Comunes
- **`conn`**: Conexión a la base de datos obtenida del pool
- **`client`**: Cliente HTTP de `reqwest` para hacer peticiones al API de Yappy
- **`url`**: URL construida para endpoints específicos de Yappy
- **`response`**: Respuesta HTTP del servidor de Yappy
- **`response_json`**: Respuesta parseada como JSON

## 3. Definiciones de Funciones

### `guardar_datos_caja`
```rust
pub async fn guardar_datos_caja(
    state: AppState,
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
    caja_id: i32,
    nombre_caja: String,
) -> Result<Value, (StatusCode, Json<Value>)>
```

**Propósito**: Cierra una caja en Yappy y guarda los datos del cierre en la base de datos.

**Entradas**:
- `state`: Estado de la aplicación con pool de conexiones
- `api_key`: Clave API para autenticación con Yappy
- `secret_key`: Clave secreta para autenticación
- `auth_token`: Token de autorización opcional
- `caja_id`: ID de la caja en la base de datos
- `nombre_caja`: Nombre descriptivo de la caja

**Salidas**:
- `Ok(Value)`: JSON con la respuesta de Yappy
- `Err((StatusCode, Json<Value>))`: Error con código de estado y detalles

**Lógica**:
1. Llama a `cerrar_caja_en_yappy` para cerrar la sesión
2. Si el código de respuesta es "YP-0000" (éxito):
   - Extrae el resumen de transacciones (`summary`)
   - Inserta cada entrada en `caja_cierre_resumen` (tipo, monto, transacciones)
   - Actualiza el estado de la caja a `Cerrado` y limpia el token
3. Si hay error:
   - Guarda la respuesta completa en `caja_cierre_errores`

---

### `cerrar_caja_en_yappy`
```rust
pub async fn cerrar_caja_en_yappy(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
) -> Result<Value, (StatusCode, Json<Value>)>
```

**Propósito**: Realiza la petición HTTP DELETE al API de Yappy para cerrar una sesión de dispositivo.

**Entradas**:
- `api_key`: Clave API
- `secret_key`: Clave secreta
- `auth_token`: Token de autorización de la sesión

**Salidas**:
- `Ok(Value)`: Respuesta JSON de Yappy
- `Err`: Error de red o configuración

**Lógica**:
1. Construye URL: `{YAPPY_ENDPOINT}/session/device`
2. Envía DELETE con headers de autenticación
3. Parsea y retorna la respuesta JSON

---

### `abrir_caja_and_return_value`
```rust

File: src\utils\mod.rs
# Análisis de `src\utils\mod.rs`

## 1. Propósito General del Archivo

Este es un **archivo de módulo** (`mod.rs`) en Rust que actúa como punto de entrada y organizador para el módulo `utils`. Su función principal es declarar y exponer submódulos relacionados con utilidades del proyecto. Es un archivo de configuración modular que permite estructurar el código en componentes lógicos separados.

## 2. Lista de Campos/Variables

**No hay campos ni variables definidos en este archivo.** Solo contiene declaraciones de módulos.

## 3. Definiciones de Funciones

**No hay funciones definidas en este archivo.** Es puramente declarativo.

## 4. Structs/Clases

**No hay structs, enums ni traits definidos en este archivo.**

## 5. Estructura Modular

El archivo declara dos submódulos:

### `pub mod utils;`
- **Visibilidad**: Público (`pub`)
- **Ubicación esperada**: `src/utils/utils.rs`
- **Propósito inferido**: Contiene utilidades generales del proyecto

### `pub mod cajas_utils;`
- **Visibilidad**: Público (`pub`)
- **Ubicación esperada**: `src/utils/cajas_utils.rs`
- **Propósito inferido**: Contiene utilidades específicas relacionadas con "cajas" (probablemente estructuras de datos o componentes del dominio del negocio)

## 6. Cómo Encaja en el Proyecto

### Rol Arquitectónico:
- **Organizador de Utilidades**: Centraliza todas las funciones auxiliares y herramientas del proyecto
- **Punto de Acceso**: Permite importar utilidades desde otros módulos usando `use crate::utils::utils::*` o `use crate::utils::cajas_utils::*`
- **Separación de Responsabilidades**: Divide las utilidades en categorías lógicas (generales vs. específicas de cajas)

### Patrón de Diseño:
Sigue el patrón de **módulos jerárquicos** de Rust, donde:
```
src/
  └── utils/
      ├── mod.rs          (este archivo)
      ├── utils.rs        (utilidades generales)
      └── cajas_utils.rs  (utilidades de cajas)
```

### Integración con el Proyecto:
- Otros módulos del proyecto pueden importar estas utilidades
- Facilita el mantenimiento al agrupar funcionalidades relacionadas
- Permite reutilización de código común en diferentes partes del sistema
- Probablemente contiene funciones helper, conversiones, validaciones o lógica auxiliar que no pertenece a la lógica de negocio principal

### Ejemplo de Uso Esperado:
```rust
// Desde otro archivo del proyecto
use crate::utils::utils::alguna_funcion_general;
use crate::utils::cajas_utils::procesar_caja;
```

**Conclusión**: Este archivo es fundamental para la organización del código, actuando como un índice que expone las utilidades del proyecto de manera estructurada y accesible.

File: src\utils\utils.rs
# Análisis de `src/utils/utils.rs`

## 1. Propósito General del Archivo

Este archivo contiene **funciones utilitarias** para el proyecto, proporcionando:
- Manejo estandarizado de errores en formato JSON
- Gestión de headers de autenticación para peticiones HTTP
- Consultas de base de datos para obtener información completa de kioskos mediante su dirección MAC

Es un módulo de soporte que centraliza funcionalidades comunes usadas en toda la aplicación.

---

## 2. Lista de Campos/Variables y sus Propósitos

### Imports y Dependencias:
- **`axum::{Json, http::StatusCode}`**: Para respuestas HTTP y códigos de estado
- **`reqwest::header::{HeaderMap, HeaderValue}`**: Manejo de headers HTTP
- **`serde_json::json`**: Creación de objetos JSON
- **`crate::db::models::{Caja, Grupo, Kiosko}`**: Modelos de base de datos
- **`crate::schema`**: Esquemas Diesel para consultas SQL
- **`diesel::prelude::*`**: ORM para interactuar con la base de datos
- **`CajasEstadoEnum`**: Enum para estados de cajas
- **`AppState`**: Estado compartido de la aplicación

---

## 3. Definiciones de Funciones

### **`json_error<E: std::fmt::Display>`**

**Firma:**
```rust
pub fn json_error<E: std::fmt::Display>(
    status: StatusCode,
    err: E,
) -> (StatusCode, Json<serde_json::Value>)
```

**Entradas:**
- `status`: Código de estado HTTP (ej: 400, 500)
- `err`: Cualquier error que implemente `Display`

**Salida:**
- Tupla con `StatusCode` y `Json` conteniendo:
  ```json
  {
    "success": false,
    "error": "mensaje de error"
  }
  ```

**Propósito:**
Convierte errores genéricos en respuestas JSON estandarizadas compatibles con Axum, facilitando el manejo consistente de errores en toda la API.

---

### **`insert_auth_headers`**

**Firma:**
```rust
pub fn insert_auth_headers(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
) -> HeaderMap
```

**Entradas:**
- `api_key`: Clave API para autenticación
- `secret_key`: Clave secreta para autenticación
- `auth_token`: Token de autorización opcional

**Salida:**
- `HeaderMap` con los headers configurados:
  - `api-key`: Clave API
  - `secret-key`: Clave secreta
  - `authorization`: Token (si está presente)

**Propósito:**
Construye un mapa de headers HTTP con credenciales de autenticación, usado para realizar peticiones autenticadas a servicios externos (probablemente Yappy).

---

### **`get_info_by_mac_address`**

**Firma:**
```rust
pub fn get_info_by_mac_address(
    state: &AppState, 
    mac_address: &str
) -> Result<KioskoInfo, diesel::result::Error>
```

**Entradas:**
- `state`: Referencia al estado de la aplicación (contiene pool de conexiones DB)
- `mac_address`: Dirección MAC del kiosko a buscar

**Salida:**
- `Result<KioskoInfo, diesel::result::Error>`: 
  - **Ok**: Información completa del kiosko
  - **Err**: Error de Diesel si falla la consulta

**Propósito:**
Realiza una **consulta en cascada** a través de tres tablas:
1. Busca el **kiosko** por MAC address
2. Obtiene la **caja** asociada

Directory: src\utils
# Resumen del Directorio `src/utils`

## Propósito General

El directorio `src/utils` es un **módulo de utilidades centralizado** que proporciona funcionalidades auxiliares y de soporte para toda la aplicación. Se enfoca en tres áreas principales:

1. **Gestión de cajas registradoras con Yappy** (sistema de pagos)
2. **Utilidades generales** (manejo de errores, autenticación HTTP, consultas DB)
3. **Organización modular** del código utilitario

---

## Estructura y Componentes

### 📁 **mod.rs** - Organizador Modular
- **Rol**: Punto de entrada del módulo `utils`
- **Función**: Declara y expone los submódulos:
  - `utils` → Utilidades generales
  - `cajas_utils` → Utilidades específicas de cajas Yappy

---

### 🛠️ **utils.rs** - Utilidades Generales

Proporciona funciones transversales usadas en toda la aplicación:

| Función | Propósito |
|---------|-----------|
| `json_error()` | Estandariza respuestas de error en formato JSON |
| `insert_auth_headers()` | Construye headers HTTP con credenciales de autenticación |
| `get_info_by_mac_address()` | Consulta información completa de kioskos por dirección MAC |

**Características clave:**
- Manejo consistente de errores HTTP
- Abstracción de autenticación para APIs externas
- Consultas complejas a base de datos con joins

---

### 💰 **cajas_utils.rs** - Gestión de Cajas Yappy

Maneja el **ciclo de vida completo** de cajas registradoras integradas con Yappy:

| Función | Operación |
|---------|-----------|
| `abrir_caja_and_return_value()` | Abre sesión en Yappy y registra en DB |
| `cerrar_caja_en_yappy()` | Cierra sesión mediante API DELETE |
| `guardar_datos_caja()` | Cierra caja, guarda resumen de transacciones o errores |

**Flujo de trabajo:**
```
1. Apertura → POST a Yappy → Guarda token en DB
2. Operación → Transacciones durante sesión activa
3. Cierre → DELETE a Yappy → Guarda resumen/errores en DB
```

---

## Cómo Trabajan en Conjunto

### 🔗 **Integración entre Módulos**

```rust
// cajas_utils.rs usa funciones de utils.rs:

// 1. Autenticación HTTP
let headers = insert_auth_headers(api_key, secret_key, auth_token);

// 2. Manejo de errores
return Err(json_error(StatusCode::INTERNAL_SERVER_ERROR, "Error al cerrar caja"));

// 3. Consultas DB (potencialmente)
let kiosko_info = get_info_by_mac_address(&state, mac_address)?;
```

### 📊 **Flujo de Datos Típico**

```
┌─────────────────────────────────────────────────────────┐
│                    API Request                          │
└────────────────────┬────────────────────────────────────┘
                     │
         ┌───────────▼──────────┐
         │   utils.rs           │
         │ - Valida headers     │
         │ - Consulta kiosko    │
         └───────────┬──────────┘
                     │
         ┌───────────▼──────────┐
         │  cajas_utils.rs      │
         │ - Abre/cierra caja   │
         │ - Llama API Yappy    │
         │ - Guarda en DB       │
         └──────────────────────┘

