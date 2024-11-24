FROM rust:1.80.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS final
WORKDIR /app
COPY --from=builder /app/target/release/discord-bot /app/bot
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    openssl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
CMD ["/app/bot"]