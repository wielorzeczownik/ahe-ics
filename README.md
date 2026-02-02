
<h1 align="center">AHE-ICS</h1>

<p align="center">
  <img src="https://img.shields.io/badge/works_on-my_machine-brightgreen?style=flat-square" alt="Works on my machine"/>
  <a href="https://github.com/wielorzeczownik/ahe-ics/releases/latest"><img src="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square" alt="Latest Release"/></a>
  <a href="https://hub.docker.com/r/wielorzeczownik/ahe-ics"><img src="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square" alt="Docker Pulls"/></a>
  <a href="https://github.com/wielorzeczownik/ahe-ics/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square" alt="License: MIT"/></a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/assets/logo.png" alt="AHE-ICS logo" width="520" />
</p>


A lightweight service that exposes [AHE](https://www.ahe.lodz.pl) schedule data as a subscribable ICS feed.

## Why this exists

Because the university website is frustrating to use for schedule tracking and
there was no sensible calendar export. AHE-ICS provides one stable URL that can be subscribed to in
normal calendar apps.

## Run with Docker (recommended)

Recommended for most users: easiest setup, reproducible runtime, and quick updates.

Run with Docker:
```bash
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

1) Download the asset for your platform from the latest release page.
2) Extract it.
3) Create `.env` from `.env.example` and set `AHE_USERNAME` / `AHE_PASSWORD`.
4) Start the binary.

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

Release artifact names follow:
- `ahe-ics-<version>-x86_64-unknown-linux-gnu.tar.gz` - Linux (Intel/AMD 64-bit)
- `ahe-ics-<version>-aarch64-unknown-linux-gnu.tar.gz` - Linux (ARM64, e.g. Raspberry Pi 64-bit)
- `ahe-ics-<version>-x86_64-apple-darwin.tar.gz` - macOS on Intel Macs
- `ahe-ics-<version>-aarch64-apple-darwin.tar.gz` - macOS on Apple Silicon (M1/M2/M3)
- `ahe-ics-<version>-x86_64-pc-windows-msvc.zip` - Windows 64-bit (x86_64)

## Environment variables

| Variable | Required | Default | Description |
|---|---|---|---|
| `AHE_USERNAME` | yes | - | [WPS](https://wps.ahe.lodz.pl/) API username |
| `AHE_PASSWORD` | yes | - | [WPS](https://wps.ahe.lodz.pl/) API password |
| `BIND_ADDR` | no | `0.0.0.0:8080` | Bind address for the HTTP server |
| `AHE_CAL_PAST_DAYS` | no | `60` | Default range: days in the past when `from` is not provided |
| `AHE_CAL_FUTURE_DAYS` | no | `60` | Default range: days in the future when `to` is not provided |
| `AHE_CAL_LANG` | no | `pl` | Generated labels language (`pl` or `en`) |
| `AHE_CAL_EXAMS_ENABLED` | no | `true` | Enable or disable exam fetching (`true`/`false`); useful when exam entries are noisy |
| `AHE_CAL_TOKEN` | no | - | Optional access token required for calendar endpoints |
| `RUST_LOG` | no | `info` | Log level (`debug`, `info`, etc.) |

## Endpoints

- `GET /calendar.ics` - primary ICS feed endpoint (`text/calendar`).
- `GET /calendar/me.ics` - alias of `/calendar.ics` (same output).
- `GET /openapi.json` - OpenAPI spec for integrations/tools.

Calendar query params (`/calendar.ics` and `/calendar/me.ics`):
- `from=YYYY-MM-DD` - start date; when omitted, service uses `AHE_CAL_PAST_DAYS`.
- `to=YYYY-MM-DD` - end date; when omitted, service uses `AHE_CAL_FUTURE_DAYS`.
- `token=...` - optional request token if `AHE_CAL_TOKEN` is configured.

Example:
```text
http://localhost:8080/calendar.ics?from=2026-01-01&to=2026-03-01
```

## Calendar subscription notes

- Works with Google Calendar / Apple Calendar / Outlook (ICS subscription).
- Use a public URL for external clients; `localhost` is local-only.
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
