FROM ubuntu:bionic
WORKDIR /root

COPY migrations/ migrations
COPY src/ src
COPY Cargo.toml .
COPY Rocket.toml .
COPY rust-toolchain .

SHELL ["/bin/bash", "-c"]

RUN apt update
RUN apt install -y curl
RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN apt install -y build-essential
RUN sh rustup.sh -y --default-toolchain "$(cat rust-toolchain)"
RUN apt install -y libpq-dev
RUN source $HOME/.cargo/env && cargo build --release

CMD ROCKET_DATABASES="{ meals={ url=\"$DATABASE_URL\" } }" ROCKET_PORT=$PORT /root/target/release/meals