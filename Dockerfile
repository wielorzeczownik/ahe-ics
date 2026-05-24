FROM rust:1.95-trixie@sha256:39d8cb39a54e7d1da665c4fabfdd265e532a5f836c11ab5aee27fd5c73891ce4 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:trixie-slim@sha256:109e2c65005bf160609e4ba6acf7783752f8502ad218e298253428690b9eaa4b

LABEL org.opencontainers.image.source="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.url="https://github.com/wielorzeczownik/ahe-ics" \
  org.opencontainers.image.documentation="https://github.com/wielorzeczownik/ahe-ics#readme" \
  org.opencontainers.image.title="ahe-ics" \
  org.opencontainers.image.description="Self-hosted AHE Łódź class schedule exporter to iCalendar (ICS)" \
  org.opencontainers.image.authors="wielorzeczownik" \
  org.opencontainers.image.vendor="wielorzeczownik"

# renovate: datasource=repology depName=debian_13/ca-certificates versioning=loose
ARG CA_CERTIFICATES_VERSION="20250419"
# renovate: datasource=repology depName=debian_13/curl versioning=loose
ARG CURL_VERSION="8.14.1-2+deb13u3"

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
      ca-certificates=${CA_CERTIFICATES_VERSION} \
      curl=${CURL_VERSION} \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ahe-ics /usr/local/bin/ahe-ics

ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD curl -fsS "http://127.0.0.1:${BIND_ADDR##*:}/healthz" || exit 1
CMD ["ahe-ics"]
