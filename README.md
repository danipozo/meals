[![Build Status](https://travis-ci.com/danipozo/meals.svg?branch=master)](https://travis-ci.com/danipozo/meals)

# Meals

## Descripción

El servicio dispondrá de una base de datos de recetas e ingredientes, y
permitirá la elaboración de menús a partir de restricciones.

## Uso

`buildtool: Makefile`

Se puede lanzar el microservicio con `make start`, y pararlo con `make stop`.

## Herramientas

El lenguaje de programación que se usará principalmente será
[Rust](https://rust-lang.org).

 - Las dependencias del código en Rust se gestionarán con
   [Cargo](https://doc.rust-lang.org/cargo/index.html).
 - Para definir *tests*, se usará [la funcionalidad que incorpora
   Rust](https://doc.rust-lang.org/book/ch11-00-testing.html) a tal efecto.
 - Para gestionar los *logs*, se planea usar un servicio que puede ser
   [Logstash](https://www.elastic.co/products/logstash).
 - El *framework* que se usará para exponer la API, en principio de tipo REST,
   será [Rocket](https://rocket.rs).
 - Se almacenarán los datos en una base de datos [SQLite](https://sqlite.org).
 - Para interactuar con la base de datos, se hará uso de la [funcionalidad
   específica de Rocket](https://rocket.rs/v0.4/guide/state/#databases) junto
   con el [ORM Diesel](https://diesel.rs).
 - Para confeccionar los menús, se escribirán las restricciones en el lenguaje
   propio de [MiniZinc](https://minizinc.org) y se devolverán los candidatos que
   encuentre que las satisfacen.
