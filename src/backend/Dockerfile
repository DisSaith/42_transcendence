FROM rust:1 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN useradd -r -u 1001 appuser
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
USER appuser
EXPOSE 3000
CMD ["backend"]

