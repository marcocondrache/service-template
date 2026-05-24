# syntax=docker/dockerfile:1

FROM rust:1.95 AS builder
WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --locked \
          && cp /app/target/release/service-template /app/service-template

FROM cgr.dev/chainguard/glibc-dynamic@sha256:e9a3236ebb746bbab93bda4ca842e55a6aaea2c812a685646043e842e69220be

COPY --from=builder /app/service-template /usr/local/bin/service-template

ENTRYPOINT ["service-template"]
