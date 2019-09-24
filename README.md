# Meals

## Descripción

El servicio dispondrá de una base de datos de recetas e ingredientes, y
permitirá la elaboración de menús teniendo en cuenta restricciones editables por
el usuario.

## Herramientas

El lenguaje de programación que se usará principalmente será
[Rust](https://rust-lang.org).

 - Las dependencias del código en Rust se gestionarán con
   [Cargo](https://doc.rust-lang.org/cargo/index.html).
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
