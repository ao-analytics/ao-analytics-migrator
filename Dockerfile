FROM lukemathwalker/cargo-chef:latest AS planner

WORKDIR /ao-analytics-migrator
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest AS builder

RUN apt install pkg-config libssl-dev
ENV SQLX_OFFLINE=true
WORKDIR /ao-analytics-migrator
COPY --from=planner /ao-analytics-migrator/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /ao-analytics-migrator/target/release/ao-analytics-migrator /usr/local/bin/ao-analytics-migrator

CMD ["ao-analytics-migrator"]
