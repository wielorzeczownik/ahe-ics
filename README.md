
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


A lightweight service that exposes AHE schedule data as a subscribable ICS feed.

## Motivation

This project exists because the university website is frustrating to use for schedule tracking and
there was no sensible calendar export. AHE-ICS provides one stable URL that can be subscribed to in
normal calendar apps.

## Docker (recommended)

Prebuilt image is available on Docker Hub:  
`https://hub.docker.com/repository/docker/wielorzeczownik/ahe-ics/general`

Run with Docker:
```bash
docker run --rm -p 8080:8080 --env-file .env wielorzeczownik/ahe-ics:latest
```

Run with Docker Compose:
```bash
docker compose -f docker-compose.example.yml up -d
```

## Endpoints

- `GET /calendar.ics`
- `GET /calendar/me.ics` (alias)
- `GET /openapi.json`

Optional query parameters for calendar endpoints:
- `from=YYYY-MM-DD`
- `to=YYYY-MM-DD`
- `token=...` (when token protection is enabled)

Example:
```text
http://localhost:8080/calendar.ics?from=2026-01-01&to=2026-03-01
```

## Environment variables

| Variable | Required | Default | Description |
|---|---|---|---|
| `AHE_USERNAME` | yes | - | WPS API username |
| `AHE_PASSWORD` | yes | - | WPS API password |
| `BIND_ADDR` | no | `0.0.0.0:8080` | Bind address for the HTTP server |
| `AHE_CAL_PAST_DAYS` | no | `60` | Default range: days in the past when `from` is not provided |
| `AHE_CAL_FUTURE_DAYS` | no | `60` | Default range: days in the future when `to` is not provided |
| `AHE_CAL_LANG` | no | `pl` | Generated labels language (`pl` or `en`) |
| `AHE_CAL_TOKEN` | no | - | Optional access token required for calendar endpoints |
| `RUST_LOG` | no | `info` | Log level (`debug`, `info`, etc.) |

## Calendar subscription notes

- Works with Google Calendar / Apple Calendar / Outlook (ICS subscription).
- Use a public URL for external clients; `localhost` is local-only.
- If `AHE_CAL_TOKEN` is enabled, include it in the subscription URL:
  `https://your-domain.example/calendar.ics?token=your-token`

## Platform support

- Linux: supported and tested.
- Windows: release binaries are produced, so support is intended, but I currently have no Windows environment to run full runtime tests. Treat Windows support as **available but unverified**.

## Disclaimer

This project is community-made, unofficial, and may break if the backend API changes.
