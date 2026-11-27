FROM rust:1 AS builder
RUN rustup target add wasm32-unknown-unknown
ARG TRUNK_VERSION=0.21.14
ARG TARGETARCH
RUN set -eux; \
    case "$TARGETARCH" in \
      amd64) ARCH=x86_64 ;; \
      arm64) ARCH=aarch64 ;; \
      *) echo "architecture non supportee: $TARGETARCH" >&2; exit 1 ;; \
    esac; \
    FILE="trunk-${ARCH}-unknown-linux-musl.tar.gz"; \
    URL="https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}"; \
    cd /tmp; \
    curl -fsSL -O "${URL}/${FILE}"; \
    curl -fsSL -O "${URL}/${FILE}.sha256"; \
    echo "$(cut -d' ' -f1 "${FILE}.sha256")  ${FILE}" | sha256sum -c -; \
    tar -xzf "${FILE}" -C /usr/local/bin trunk; \
    rm "${FILE}" "${FILE}.sha256"; \
    trunk --version
WORKDIR /app
COPY Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release --target wasm32-unknown-unknown
COPY . .
RUN touch src/main.rs && trunk build --release

FROM debian:bookworm-slim
COPY --from=builder /app/dist /app/dist
CMD ["sh", "-c", "rm -rf /dist/* && cp -r /app/dist/* /dist/ && chmod -R a+rX /dist"]
