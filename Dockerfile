FROM lukemathwalker/cargo-chef:latest as planner

WORKDIR /ao-analytics-migrator
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest as builder

ENV SQLX_OFFLINE=true
WORKDIR /ao-analytics-migrator
COPY --from=planner /ao-analytics-migrator/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY .env.prod .env
RUN cargo build --release

FROM ubuntu:latest

EXPOSE 8080
COPY --from=builder /ao-analytics-migrator/target/release/ao-analytics-migrator /usr/local/bin/ao-analytics-migrator

CMD ["ao-analytics-migrator"]