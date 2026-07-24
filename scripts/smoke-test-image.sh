#!/usr/bin/env bash
set -euo pipefail

: "${IMAGE:?IMAGE is required}"

port="${PORT:-18080}"
container=$(docker run -d -p "$port:8080" \
  -e AHE_USERNAME=smoke-test \
  -e AHE_PASSWORD=smoke-test \
  "$IMAGE")

cleanup() {
  docker logs "$container" 2>&1 | tail -20 || true
  docker rm -f "$container" >/dev/null 2>&1 || true
}
trap cleanup EXIT

for _ in $(seq 1 30); do
  status=$(curl -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:$port/healthz" || echo "000")
  if [[ "$status" != "000" ]]; then
    echo "$IMAGE served /healthz with HTTP $status"
    exit 0
  fi
  sleep 1
done

echo "$IMAGE did not serve HTTP within 30s" >&2
exit 1
