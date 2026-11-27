# syntax=docker/dockerfile:1

ARG APP_NAME={{project-name}}

FROM rust:slim-trixie AS chef
RUN cargo install --locked cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY rust-toolchain.toml rust-toolchain.toml
RUN rustup target add x86_64-unknown-linux-musl \
    && apt-get update \
    && apt-get install --no-install-recommends -y musl-tools musl-dev gcc ca-certificates \
    && rm -rf /var/lib/apt/lists/*
ENV CC_x86_64_unknown_linux_musl=musl-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

ARG APP_NAME
COPY . .
RUN cargo build --release \
    --target x86_64-unknown-linux-musl --bin ${APP_NAME}

FROM scratch AS runtime
ARG APP_NAME
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${APP_NAME} /usr/local/bin/app
USER 10001:10001
ENTRYPOINT ["/usr/local/bin/app"]
