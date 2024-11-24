FROM rust:1.73 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim as final
WORKDIR /app
COPY --from=builder /app/target/release/your-server-binary /app/server
RUN apt-get update && apt-get install -y ca-certificates
CMD ["/app/server"]
