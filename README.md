<h1 align="center">AHE-ICS</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/ahe-ics/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Latest Release"/></picture></a> <a href="https://hub.docker.com/r/wielorzeczownik/ahe-ics"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Docker Pulls"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="License: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/assets/logo.svg" alt="AHE-ICS logo" width="200" />
</p>

<p align="center">🇬🇧 English | 🇵🇱 <a href="README.pl.md">Polski</a></p>

Your [AHE Łódź](https://www.ahe.lodz.pl) class schedule from [WPS](https://wps.ahe.lodz.pl) in Google Calendar, Apple Calendar or Outlook. You add the calendar by URL, and classes and exams stay up to date on their own.

It comes in two variants:

| Variant       | Binary           | Docker image                     | Credentials                           |
| ------------- | ---------------- | -------------------------------- | ------------------------------------- |
| **Dedicated** | `ahe-ics`        | `wielorzeczownik/ahe-ics`        | in env vars, one account per instance |
| **Shared**    | `ahe-ics-shared` | `wielorzeczownik/ahe-ics:shared` | encrypted in each user's link         |

Run the dedicated one for yourself. Run the shared one if other people will use your instance.

## Running

### Docker

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
# set AHE_USERNAME and AHE_PASSWORD in .env
docker run --rm -p 8080:8080 --env-file .env wielorzeczownik/ahe-ics:latest
```

For Docker Compose, also grab [docker-compose.example.yml](docker-compose.example.yml) and run `docker compose up -d`.

For the shared variant, use [.env.shared.example](.env.shared.example) (set `AHE_LINK_KEY` and `AHE_CAL_TOKEN` there), the `:shared` image and [docker-compose.shared.example.yml](docker-compose.shared.example.yml).

### Binaries

Download the archive for your system from [Releases](https://github.com/wielorzeczownik/ahe-ics/releases/latest), extract it, put a `.env` next to the binary and run it.

| System                          | `ahe-ics`                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `ahe-ics-shared`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Linux (glibc 2.35+)             | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-gnu.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-gnu.tar.gz), [ARMv7](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-armv7-unknown-linux-gnueabihf.tar.gz), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-unknown-linux-gnu.tar.gz) | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-unknown-linux-gnu.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-unknown-linux-gnu.tar.gz), [ARMv7](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-armv7-unknown-linux-gnueabihf.tar.gz), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-i686-unknown-linux-gnu.tar.gz) |
| Linux (musl, static)            | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-musl.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-musl.tar.gz)                                                                                                                                                                                                                                            | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-unknown-linux-musl.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-unknown-linux-musl.tar.gz)                                                                                                                                                                                                                                                          |
| macOS (Intel and Apple Silicon) | [universal](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-universal-apple-darwin.tar.gz)                                                                                                                                                                                                                                                                                                                                                                 | [universal](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-universal-apple-darwin.tar.gz)                                                                                                                                                                                                                                                                                                                                                                                      |
| Windows                         | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-pc-windows-msvc.zip), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-pc-windows-msvc.zip), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-pc-windows-msvc.zip)                                                                                                                                           | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-pc-windows-msvc.zip), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-pc-windows-msvc.zip), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-i686-pc-windows-msvc.zip)                                                                                                                                                  |

## Configuration

Settings are read from env vars or from a `.env` file.

| Variable                | Required       | Default        | Meaning                                                                             |
| ----------------------- | -------------- | -------------- | ----------------------------------------------------------------------------------- |
| `AHE_USERNAME`          | dedicated only | -              | WPS login                                                                           |
| `AHE_PASSWORD`          | dedicated only | -              | WPS password                                                                        |
| `AHE_LINK_KEY`          | shared only    | -              | 32-byte base64 key that links are encrypted with (`openssl rand -base64 32`)        |
| `AHE_CAL_TOKEN`         | no             | -              | token required to read the calendar, see [below](#access-token)                     |
| `AHE_CAL_PAST_DAYS`     | no             | `60`           | how many days back to fetch when `from` isn't given                                 |
| `AHE_CAL_FUTURE_DAYS`   | no             | `60`           | how many days ahead to fetch when `to` isn't given                                  |
| `AHE_CAL_LANG`          | no             | `pl`           | language of generated labels, `pl` or `en`                                          |
| `AHE_CAL_EXAMS_ENABLED` | no             | `true`         | `false` leaves exams out, see [Exams](#exams)                                       |
| `AHE_CAL_JSON_ENABLED`  | no             | `true`         | `false` turns off the `.json` endpoints                                             |
| `BIND_ADDR`             | no             | `0.0.0.0:8080` | address the server listens on                                                       |
| `REAL_IP_HEADER`        | no             | -              | header with the client IP behind a proxy, e.g. `CF-Connecting-IP`                   |
| `HTTPS_PROXY`           | no             | -              | proxy for WPS requests, `http://` or `socks5h://` (Tor: `socks5h://127.0.0.1:9050`) |
| `RUST_LOG`              | no             | `info`         | log level                                                                           |

### Access token

Without `AHE_CAL_TOKEN`, anyone who knows the address can read your schedule. Set it whenever the instance is reachable from the internet.

The value can be a plain string or an Argon2id hash (`$argon2id$...`, optionally prefixed with `argon2:`). The client always sends the plain token, as `?token=...`, in the `X-Calendar-Token` header or as `Authorization: Bearer ...`.

<details>
<summary>Generating an Argon2id hash</summary>

```bash
docker run --rm -e TOKEN='your-token' python:3.15-alpine sh -lc "pip install --quiet argon2-cffi && python - <<'PY'
import os
from argon2 import PasswordHasher

print(PasswordHasher(time_cost=3, memory_cost=65536, parallelism=1).hash(os.environ['TOKEN']))
PY"
```

</details>

## Endpoints

| Endpoint                                  | Description                                                                   |
| ----------------------------------------- | ----------------------------------------------------------------------------- |
| `GET /calendar.ics`, `/calendar/me.ics`   | the ICS feed                                                                  |
| `GET /calendar.json`, `/calendar/me.json` | the data the feed is built from                                               |
| `GET /healthz`                            | always `204`, doesn't call WPS                                                |
| `GET /readyz`                             | `204` if the WPS login works, `503` if not (dedicated only)                   |
| `POST /link`                              | creates a subscription link (shared only, see [below](#shared-variant-links)) |

The calendar endpoints accept `from=YYYY-MM-DD`, `to=YYYY-MM-DD` and `token`, e.g. `/calendar.ics?from=2026-10-01&to=2027-02-28`.

## Subscribing

Add the feed URL to your calendar:

- **Google Calendar**: Other calendars → From URL
- **Apple Calendar**: File → New Calendar Subscription
- **Outlook**: Add calendar → Subscribe from web

The calendar app fetches the feed from its own servers, so the address must be public. `localhost` won't work.

## Shared variant links

In the shared variant, each user's WPS login goes into the link, encrypted with `AHE_LINK_KEY`. `POST /link` checks the login against WPS and returns the path:

```bash
curl https://your-domain.example/link -d username=jan.kowalski --data-urlencode password='haslo'
# /calendar.ics?c=AXk3...
```

If `AHE_CAL_TOKEN` is set, also send `-d token=...`. Put your domain in front and subscribe to `https://your-domain.example/calendar.ics?c=...`. `/link` accepts 10 requests per minute in total, so it can't be used to guess WPS passwords.

A link stops working when the WPS password changes or when `AHE_LINK_KEY` changes. Generate a new one then.

### Privacy

- The server keeps only in-memory caches (WPS tokens, student data, rendered calendars). They are gone after a restart. No database, no files.
- Passwords, usernames, query strings and `c` values are never logged. Logs contain the client IP and, when WPS returns an error, its status and body.
- A link can end up in reverse proxy or Cloudflare logs, at your calendar provider and in browser history. Someone who gets it can read your schedule, but not your password, because it can't be decrypted without `AHE_LINK_KEY`.
- The server decrypts your password and logs in to WPS on your behalf, so you have to trust whoever runs it. Anyone with both the key and a link can recover the password.

## Exams

WPS sometimes returns entries that look like exams but aren't the ones you'd want in your calendar, and there's no field to reliably filter them out. If that happens to you, set `AHE_CAL_EXAMS_ENABLED=false`.

## Disclaimer

Not official and not affiliated with AHE. If WPS changes its API, this will stop working until it's fixed here.
