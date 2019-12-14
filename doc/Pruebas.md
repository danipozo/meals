# Pruebas

## Pruebas unitarias

Las pruebas unitarias están escritas usando [la funcionalidad estándar de Rust](https://doc.rust-lang.org/book/ch11-00-testing.html).

### Estructuras de datos

El fichero [src/models.rs](../src/models.rs) contiene algunas pruebas destinadas
a comprobar la coherencia entre las estructuras de datos definidas en el código
Rust y el esquema de la base de datos.

Consisten en efectuar una consulta usando estas estructuras de datos usando
Diesel. Por el tipado estático de Diesel, estas pruebas no compilan (y por tanto
fallan) si no existe tal coherencia.

### API

El fichero [src/api_test.rs](../src/api_test.rs) contiene las pruebas destinadas
a comprobar el correcto funcionamiento de la API. Para que estas pruebas
funcionen, hay que ejecutar la migración que incorpora datos de prueba en
[migrations_test/](../migrations_test).

Consisten en ejecutar un servidor en modo de desarrollo y hacerle consultas,
para comprobar que devuelven lo que deberían.
