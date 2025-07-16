
#Como correr el programa

### Ir a la carpeta del proyecto y utilizar el comando `cargo run`

#Diesel ORM


### para hacer el setup principal (init)
-- diesel setup

### para generar las migraciones desde el schema (crear primero el schema)
-- diesel migration generate --diff-schema {{nombre_de_migracion}}

### para correr las migraciones
-- diesel migration run

