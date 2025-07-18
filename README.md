
### Creador:
- Pablo Lizana
  
# Como descargar rust
descargar el lenguaje de programación en https://www.rust-lang.org/es/learn/get-started eligiendo el sistema operativo correspondiente

# Como correr el programa

Clonar y navegar hasta la carpeta del proyecto  y correr el comando 

`cargo run`

Este comando hara todo lo necesario para descargar e instalar los "crates" (paquetes) y si no hay problema, correra el programa automaticamente

---

# Errores comunes
Si se trata de correr el programa por primera vez, es muy probable que les de este error

```
thread 'main' panicked at C:\Users\soporte\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\mysqlclient-sys-0.4.7\build.rs:101:5:
Did not find a compatible version of libmysqlclient.
Ensure that you installed one and teached mysqlclient-sys how to find it
You have the following options for that:

* Use `pkg_config` to automatically detect the right location
* Use vcpkg to automatically detect the right location.
You also need to set `MYSQLCLIENT_VERSION` to specify which
version of libmysqlclient you are using
* Set the `MYSQLCLIENT_LIB_DIR` and `MYSQLCLIENT_VERSION` environment
variables to point the compiler to the right directory and specify
which version is used
* Make the `mysql_config` binary avaible in the environment that invokes
the compiler
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

para solucionarlo simplemente agregar como variable de entorno los valores `MYSQLCLIENT_VERSION` y `MYSQLCLIENT_LIB_DIR` como en este ejemplo:

<img width="608" height="581" alt="imagen" src="https://github.com/user-attachments/assets/ea1ba7da-7fed-4ea2-93e8-fda26802c4ac" />

---

**Si aun despues de hacer esto, sale un error llamado `STATUS_DLL_NOT_FOUND` entonces descargar e instalar las librerias de OpenSSL mediante este link: https://slproweb.com/products/Win32OpenSSL.html.**

---

# Diesel ORM

### para hacer el setup principal (init)
-- diesel setup

### para generar las migraciones desde el schema (crear primero el schema)
-- diesel migration generate --diff-schema {{nombre_de_migracion}}

### para correr las migraciones
-- diesel migration run

