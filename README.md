[![Build Status](https://travis-ci.com/danipozo/meals.svg?branch=master)](https://travis-ci.com/danipozo/meals)

# Meals

## Descripción

El servicio dispondrá de una base de datos de recetas e ingredientes, y
permitirá la elaboración de menús a partir de restricciones.

## Uso

`buildtool: Makefile `

Las órdenes para arrancar y parar el servicio, construirlo y ejecutar las
pruebas están escritas en el [`Makefile`](Makefile), y son las siguientes:

- `start`: arranca el servicio.
- `stop`: para el servicio.
- `test`: ejecuta las [pruebas](doc/Pruebas.md).
- `build`: construye el ejecutable.

El `Makefile` delega en [`igniter`](https://github.com/pmarino90/igniter) para
las dos primeras tareas y en [`cargo`](https://doc.rust-lang.org/cargo/) para
las dos restantes.

`cargo` es la herramienta de construcción y gestión de dependencias y pruebas
propia de Rust.  Su uso en el `Makefile` en las tareas `test` y `build` se
limita a ejecutar `cargo test` y `cargo build`.

La orden `test`, además, se encarga de asegurar la presencia de una base de
datos de prueba en `db/test.sqlite`.

Se ha elegido `igniter` para gestionar el arranque y la parada del servicio por
poder instalarse a través de `cargo`. Se configura mediante el archivo
[`.igniterc`](.igniterc), en el que se especifica la orden que lanza el servicio
(`cargo run`).

## API

La documentación sobre las rutas de la API REST puede consultarse [aquí](doc/API.md).
