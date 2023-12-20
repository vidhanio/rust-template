FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/{{ project-name }} /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/{{ project-name }}"]