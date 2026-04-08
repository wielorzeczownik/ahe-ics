FROM rust:1.94-bookworm@sha256:6ae102bdbf528294bc79ad6e1fae682f6f7c2a6e6621506ba959f9685b308a55 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim@sha256:4724b8cc51e33e398f0e2e15e18d5ec2851ff0c2280647e1310bc1642182655d

LABEL org.opencontainers.image.source="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.description="Self-hosted AHE Łódź class schedule exporter to iCalendar (ICS)"

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
