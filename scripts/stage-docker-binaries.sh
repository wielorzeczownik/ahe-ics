#!/usr/bin/env bash
set -euo pipefail

: "${BIN_NAME:?BIN_NAME is required}"
: "${LIBC:?LIBC is required (gnu or musl)}"
: "${PLATFORMS:?PLATFORMS is required (comma separated buildx platforms)}"

artifact_dir="${ARTIFACT_DIR:-dist}"
context_dir="${CONTEXT_DIR:-docker-context}"

rust_target() {
  case "$1|$LIBC" in
  "linux/amd64|gnu") echo "x86_64-unknown-linux-gnu" ;;
  "linux/arm64|gnu") echo "aarch64-unknown-linux-gnu" ;;
  "linux/arm/v7|gnu") echo "armv7-unknown-linux-gnueabihf" ;;
  "linux/386|gnu") echo "i686-unknown-linux-gnu" ;;
  "linux/amd64|musl") echo "x86_64-unknown-linux-musl" ;;
  "linux/arm64|musl") echo "aarch64-unknown-linux-musl" ;;
  *)
    echo "No release binary is built for platform '$1' with libc '$LIBC'" >&2
    return 1
    ;;
  esac
}

rm -rf "$context_dir"

IFS=',' read -r -a platforms <<<"$PLATFORMS"
for platform in "${platforms[@]}"; do
  target=$(rust_target "$platform")
  archive="$artifact_dir/${BIN_NAME}-${target}.tar.gz"

  if [[ ! -f "$archive" ]]; then
    echo "Missing release archive: $archive" >&2
    exit 1
  fi

  dest="$context_dir/$platform"
  mkdir -p "$dest"
  tar -xzf "$archive" -C "$dest" "$BIN_NAME"
  chmod +x "$dest/$BIN_NAME"
  echo "staged $platform from $target"
done
