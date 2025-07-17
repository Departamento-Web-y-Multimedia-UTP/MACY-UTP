# Como descargar rust
descargar el lenguaje de programación en https://www.rust-lang.org/es/learn/get-started eligiendo el sistema operativo correspondiente

# Como correr el programa

Clonar y navegar hasta la carpeta del proyecto  y correr el comando 

`cargo run`

Este comando hara todo lo necesario para descargar e instalar los "crates" (paquetes) y si no hay problema, correra el programa automaticamente

---

# Diesel ORM

### para hacer el setup principal (init)
-- diesel setup

### para generar las migraciones desde el schema (crear primero el schema)
-- diesel migration generate --diff-schema {{nombre_de_migracion}}

### para correr las migraciones
-- diesel migration run

