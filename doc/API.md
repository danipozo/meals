# API

El microservicio expone una [API
REST](https://en.wikipedia.org/wiki/Representational_state_transfer) a través de
HTTP.

Todos los mensajes se devuelven en formato JSON.

En el diseño de la API, se ha intentado atender a las recomendaciones [de este
documento](https://docs.microsoft.com/en-us/azure/architecture/best-practices/api-design).

## Rutas


| Ruta                | Método        | Efecto                                                                 |
|---------------------|---------------|------------------------------------------------------------------------|
| `/status`           | `GET`         | Devuelve `{"status":"OK"}`                                             |
| `/menus`            | `GET`         | Lista los menús almacenados                                            |
|                     | `PUT`/`PATCH` | Permite actualizar, a lo sumo, las descripciones de una serie de menús |
|                     | `POST`        | Crea un nuevo menú a partir de una serie de restricciones              |
|                     | `DELETE`      | Borra todos los menús                                                  |
| `/menus/<id>`       | `GET`         | Devuelve la descripción del menú con identificador `id`                |
|                     | `PUT`/`PATCH` | Permite actualizar, a lo sumo, la descripción de un menú               |
|                     | `DELETE`      | Borra el menú con identificador `id`                                   |
| `/ingredients`      | `GET`         | Lista los ingredientes almacenados                                     |
| `/ingredients/<id>` | `GET`         | Devuelve la información relativa al ingrediente con identificador `id` |
| `/recipes`          | `GET`         | Lista las recetas almacenadas                                          |
| `/recipes/<id>`     | `GET`         | Devuelve una representación de la receta con identificador `id`        |
