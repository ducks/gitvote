FROM rust:1.86 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/gitvote /usr/local/bin/gitvote

ENTRYPOINT ["gitvote"]
