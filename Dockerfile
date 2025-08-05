FROM rust:latest

WORKDIR /usr/src/hello-world-api

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
COPY src ./src

RUN cargo build --release

CMD ["cargo", "run", "--release"]
