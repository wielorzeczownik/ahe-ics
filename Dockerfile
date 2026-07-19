FROM rust:1.97-trixie@sha256:44637ff22d0a6571a221bfaf137849711ad02ff4723dbb4736e297538f6a3e60 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:trixie-slim@sha256:020c0d20b9880058cbe785a9db107156c3c75c2ac944a6aa7ab59f2add76a7bd

LABEL org.opencontainers.image.source="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.url="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.documentation="https://github.com/wielorzeczownik/ahe-ics#readme" \
  org.opencontainers.image.title="ahe-ics" \
  org.opencontainers.image.description="Self-hosted AHE Łódź class schedule exporter to iCalendar (ICS)" \
  org.opencontainers.image.authors="wielorzeczownik" \
  org.opencontainers.image.vendor="wielorzeczownik"

# hadolint ignore=DL3008
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ahe-ics /usr/local/bin/ahe-ics

ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD curl -fsS "http://127.0.0.1:${BIND_ADDR##*:}/healthz" || exit 1
CMD ["ahe-ics"]
