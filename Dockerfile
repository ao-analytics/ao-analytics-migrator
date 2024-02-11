FROM rust as builder

ENV SQLX_OFFLINE=true

WORKDIR /ao-analytics-migrator
COPY . .
COPY .env.prod .env
RUN cargo install --path .

FROM ubuntu:latest

COPY --from=builder /usr/local/cargo/bin/ao-analytics-migrator /usr/local/bin/ao-analytics-migrator

CMD ["aodata-db-tool"]