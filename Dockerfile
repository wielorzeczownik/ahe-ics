FROM rust:1.95-bookworm@sha256:225aa827d55fae9816a0492284592827e794a5247c6c6a961c3b471b344295ec AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim@sha256:f9c6a2fd2ddbc23e336b6257a5245e31f996953ef06cd13a59fa0a1df2d5c252

LABEL org.opencontainers.image.source="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.url="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.documentation="https://github.com/wielorzeczownik/ahe-ics#readme" \
  org.opencontainers.image.title="ahe-ics" \
  org.opencontainers.image.description="Self-hosted AHE Łódź class schedule exporter to iCalendar (ICS)" \
  org.opencontainers.image.authors="wielorzeczownik" \
  org.opencontainers.image.vendor="wielorzeczownik"

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ahe-ics /usr/local/bin/ahe-ics

ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD curl -fsS "http://127.0.0.1:${BIND_ADDR##*:}/healthz" || exit 1
CMD ["ahe-ics"]
