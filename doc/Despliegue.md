# Despliegue

## Heroku

El [`Makefile`](../Makefile) incluye una tarea para desplegar en Heroku,
`deploy`. Funciona asumiendo lo siguiente:

  - El usuario está registrado en Heroku, y tiene la herramienta CLI de Heroku
    instalada.
  - El usuario no ha desplegado la aplicación aún. Para despliegues sucesivos,
    lo único que se requiere es empujar los cambios al remoto `heroku`.
  - El usuario quiere desplegar la rama `master`.

### Uso

Se debe llamar a `make deploy`, proveyendo el nombre de la aplicación que se
quiere desplegar como una variable de entorno, `APP_NAME`.

### Funcionamiento

La ejecución del despliegue se resume en:

  1. Registrar una aplicación nueva en la cuenta de Heroku, con un *buildpack*
     específico para su funcionamiento (Heroku no provee un *buildpack* oficial
     para Rust).
  2. Añadir una base de datos de PostgreSQL gestionada a la aplicación.
  3. Empujar el código a Heroku para el despliegue.
