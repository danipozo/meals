# Docker

## Dockerfile

El Dockerfile construye una imagen con las dependencias necesarias para
construir y ejecutar el servicio.

### Funcionamiento

Partimos de una imagen de la última versión estable de Ubuntu, a la fecha,
Bionic.

`FROM ubuntu:bionic`

Indicamos que el directorio de trabajo será `/root`. Esto carece de relevancia
más allá de facilitar la depuración de un contenedor.

`WORKDIR /root`

Copiamos los ficheros necesarios para construir el servicio:

```
COPY migrations/ migrations
COPY src/ src
COPY Cargo.toml .
COPY Rocket.toml .
COPY rust-toolchain .
```

Cambiamos la *shell* usada a Bash, para poder usar `source` cómodamente.

`SHELL ["/bin/bash", "-c"]`

Instalamos `rustup`, para instalar los componentes de Rust necesarios y en la
versión requerida para compilar el servicio (especificada en
`[rust-toolchain](../rust-toolchain)`). Además, instalamos `libpq`, necesaria
para compilar Diesel con soporte para PostgreSQL. Finalmente compilamos la
aplicación.

```
RUN apt update
RUN apt install -y curl
RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN apt install -y build-essential
RUN sh rustup.sh -y --default-toolchain "$(cat rust-toolchain)"
RUN apt install -y libpq-dev
RUN source $HOME/.cargo/env && cargo build --release
```

Indicamos que, por defecto, al ejecutar esta imagen, se ejecute la orden que levanta el servicio:

`CMD ROCKET_DATABASES="{ meals={ url=\"$DATABASE_URL\" } }" ROCKET_PORT=$PORT /root/target/release/meals`
