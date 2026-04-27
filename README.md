<h1 align="center">AHE-ICS</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/ahe-ics/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Latest Release"/></picture></a> <a href="https://hub.docker.com/r/wielorzeczownik/ahe-ics"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Docker Pulls"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="License: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/assets/logo.png" alt="AHE-ICS logo" width="300" />
</p>

<p align="center">🇬🇧 English | 🇵🇱 <a href="README.pl.md">Polski</a></p>

A lightweight self-hosted service that exports the [Akademia Humanistyczno-Ekonomiczna (AHE) Łódź](https://www.ahe.lodz.pl) class schedule as a subscribable ICS feed – compatible with Google Calendar, Apple Calendar, and Outlook.

Subscribe once with a single URL and your AHE class schedule stays automatically up to date in any calendar app.

## Self-host with Docker (recommended)

Run with Docker:

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
# edit .env and set AHE_USERNAME / AHE_PASSWORD
docker run --rm -p 8080:8080 --env-file .env wielorzeczownik/ahe-ics:latest
```

Run with Docker Compose:

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
curl -fsSL -o docker-compose.yml https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/docker-compose.example.yml
# edit .env and set AHE_USERNAME / AHE_PASSWORD
docker compose pull
docker compose up -d
```

## Run from GitHub Release binaries

Each release includes prebuilt archives for Linux, macOS, and Windows.
Latest release: [GitHub Releases](https://github.com/wielorzeczownik/ahe-ics/releases/latest)

1. Download the asset for your platform from the latest release page.
2. Extract it.
3. Create `.env` from `.env.example` and set `AHE_USERNAME` / `AHE_PASSWORD`.
4. Start the binary.

Example (Linux/macOS):

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
# edit .env
./ahe-ics
```

Example (Windows PowerShell):

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example" -OutFile ".env"
# edit .env
.\ahe-ics.exe
```

Download the latest release asset for your platform:

**Linux (glibc — requires glibc 2.35+):**

- [ahe-ics-x86_64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 64-bit)
- [ahe-ics-aarch64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-gnu.tar.gz) – Linux (ARM64, e.g. Raspberry Pi 64-bit)
- [ahe-ics-armv7-unknown-linux-gnueabihf.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-armv7-unknown-linux-gnueabihf.tar.gz) – Linux (ARM 32-bit, e.g. Raspberry Pi 32-bit)
- [ahe-ics-i686-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 32-bit)

**Linux (musl — fully static, no glibc dependency):**

- [ahe-ics-x86_64-unknown-linux-musl.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-musl.tar.gz) – Linux (Intel/AMD 64-bit)
- [ahe-ics-aarch64-unknown-linux-musl.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-musl.tar.gz) – Linux (ARM64)

**macOS:**

- [ahe-ics-x86_64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-apple-darwin.tar.gz) – macOS on Intel Macs
- [ahe-ics-aarch64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-apple-darwin.tar.gz) – macOS on Apple Silicon (M1/M2/M3/M4)

**Windows:**

- [ahe-ics-x86_64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-pc-windows-msvc.zip) – Windows 64-bit (x86_64)
- [ahe-ics-aarch64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-pc-windows-msvc.zip) – Windows ARM64
- [ahe-ics-i686-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-pc-windows-msvc.zip) – Windows 32-bit (x86)

## Environment variables

| Variable                | Required | Default        | Description                                                                          |
| ----------------------- | -------- | -------------- | ------------------------------------------------------------------------------------ |
| `AHE_USERNAME`          | yes      | -              | [WPS](https://wps.ahe.lodz.pl/) username                                             |
| `AHE_PASSWORD`          | yes      | -              | [WPS](https://wps.ahe.lodz.pl/) password                                             |
| `BIND_ADDR`             | no       | `0.0.0.0:8080` | Bind address for the HTTP server                                                     |
| `AHE_CAL_PAST_DAYS`     | no       | `60`           | Default range: days in the past when `from` is not provided                          |
| `AHE_CAL_FUTURE_DAYS`   | no       | `60`           | Default range: days in the future when `to` is not provided                          |
| `AHE_CAL_LANG`          | no       | `pl`           | Generated labels language (`pl` or `en`)                                             |
| `AHE_CAL_EXAMS_ENABLED` | no       | `true`         | Enable or disable exam fetching (`true`/`false`); useful when exam entries are noisy |
| `AHE_CAL_JSON_ENABLED`  | no       | `true`         | Enable or disable JSON calendar endpoints (`/calendar.json`, `/calendar/me.json`)    |
| `AHE_CAL_TOKEN`         | no       | -              | Optional access token for calendar endpoints (plain string or Argon2id hash)         |
| `REAL_IP_HEADER`        | no       | -              | Header name with client IP (e.g. `CF-Connecting-IP`, `X-Forwarded-For`, `Forwarded`) |
| `RUST_LOG`              | no       | `info`         | Log level (`debug`, `info`, etc.)                                                    |

`AHE_CAL_TOKEN` supports:

- plain token (e.g. `AHE_CAL_TOKEN=my-secret`),
- Argon2id hash (`$argon2id$...`),
- explicit Argon2id mode (`AHE_CAL_TOKEN=argon2:$argon2id$...`).

When hash mode is used, clients still send the normal plain `token=...` (or header), and the server verifies it against the hash.

Example Argon2id generation via Docker:

```bash
docker run --rm -e TOKEN='your-token' python:3.14-alpine sh -lc "pip install --quiet argon2-cffi && python - <<'PY'
import os
from argon2 import PasswordHasher

print(PasswordHasher(time_cost=3, memory_cost=65536, parallelism=1).hash(os.environ['TOKEN']))
PY"
```

## Endpoints

- `GET /calendar.ics` – primary ICS feed endpoint (`text/calendar`).
- `GET /calendar/me.ics` – alias of `/calendar.ics` (same output).
- `GET /calendar.json` – JSON with source data used to render the ICS feed (when `AHE_CAL_JSON_ENABLED=true`).
- `GET /calendar/me.json` – alias of `/calendar.json` (when `AHE_CAL_JSON_ENABLED=true`).
- `GET /healthz` – health check that verifies connectivity to the AHE API (returns `204 No Content`, otherwise `503`).

Calendar query params (`/calendar.ics`, `/calendar/me.ics`, and JSON endpoints when enabled):

- `from=YYYY-MM-DD` – start date; when omitted, service uses `AHE_CAL_PAST_DAYS`.
- `to=YYYY-MM-DD` – end date; when omitted, service uses `AHE_CAL_FUTURE_DAYS`.
- `token=...` – optional request token if `AHE_CAL_TOKEN` is configured.

Example:

```text
http://localhost:8080/calendar.ics?from=2026-01-01&to=2026-03-01
```

## Google Calendar / Apple Calendar / Outlook subscription

Subscribe to the ICS feed URL in your calendar app - the schedule will sync automatically:

- **Google Calendar** → Other calendars → Add by URL
- **Apple Calendar** → File → New Calendar Subscription
- **Outlook** → Add calendar → Subscribe from web

Use a public URL when subscribing from external clients. `localhost` is local-only.

> [!CAUTION]
> If you expose the service on a domain or public IP, set `AHE_CAL_TOKEN` to protect your feed. Then include it in the subscription URL, e.g.: `https://your-domain.example/calendar.ics?token=your-token`

## Exam data limitation

Exam data comes from [WPS](https://wps.ahe.lodz.pl/) endpoints that can sometimes return entries that look valid but are not
the exams you actually want in your calendar (false positives / too many entries). There is no
fully reliable API signal to filter all of these cases.

If this affects your account, disable exams entirely with:

- `AHE_CAL_EXAMS_ENABLED=false`

## Platform support

- Linux: tested and treated as primary runtime.
- Windows: release binaries are published and should work, but runtime is not regularly verified in a dedicated Windows environment.
- macOS: expected to work when running from source (same stack as Linux).

## Disclaimer

This project is community-made, unofficial, and may break if the backend API changes.
