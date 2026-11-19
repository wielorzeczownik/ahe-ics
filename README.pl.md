<h1 align="center">AHE-ICS</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/ahe-ics/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Najnowsze wydanie"/></picture></a> <a href="https://hub.docker.com/r/wielorzeczownik/ahe-ics"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Docker Pulls"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="Licencja: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/assets/logo.svg" alt="AHE-ICS logo" width="200" />
</p>

<p align="center">🇬🇧 <a href="README.md">English</a> | 🇵🇱 Polski</p>

Twój plan zajęć z [WPS](https://wps.ahe.lodz.pl) [AHE Łódź](https://www.ahe.lodz.pl) w Kalendarzu Google, Kalendarzu Apple albo Outlooku. Dodajesz kalendarz z adresu URL, a zajęcia i egzaminy aktualizują się same.

Są dwa warianty:

| Wariant           | Binarka          | Obraz Docker                     | Dane logowania                                       |
| ----------------- | ---------------- | -------------------------------- | ---------------------------------------------------- |
| **Dedykowany**    | `ahe-ics`        | `wielorzeczownik/ahe-ics`        | w zmiennych środowiskowych, jedno konto na instancję |
| **Współdzielony** | `ahe-ics-shared` | `wielorzeczownik/ahe-ics:shared` | zaszyfrowane w linku każdego użytkownika             |

Dedykowany uruchom dla siebie. Współdzielony, jeśli z twojej instancji mają korzystać inni.

## Uruchomienie

### Docker

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
# ustaw AHE_USERNAME i AHE_PASSWORD w .env
docker run --rm -p 8080:8080 --env-file .env wielorzeczownik/ahe-ics:latest
```

Dla Docker Compose pobierz też [docker-compose.example.yml](docker-compose.example.yml) i uruchom `docker compose up -d`.

Dla wariantu współdzielonego użyj [.env.shared.example](.env.shared.example) (ustaw tam `AHE_LINK_KEY` i `AHE_CAL_TOKEN`), obrazu `:shared` i [docker-compose.shared.example.yml](docker-compose.shared.example.yml).

### Binarki

Pobierz archiwum dla swojego systemu z [Releases](https://github.com/wielorzeczownik/ahe-ics/releases/latest), rozpakuj, połóż `.env` obok binarki i ją uruchom.

| System                        | `ahe-ics`                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `ahe-ics-shared`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Linux (glibc 2.35+)           | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-gnu.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-gnu.tar.gz), [ARMv7](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-armv7-unknown-linux-gnueabihf.tar.gz), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-unknown-linux-gnu.tar.gz) | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-unknown-linux-gnu.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-unknown-linux-gnu.tar.gz), [ARMv7](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-armv7-unknown-linux-gnueabihf.tar.gz), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-i686-unknown-linux-gnu.tar.gz) |
| Linux (musl, statycznie)      | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-musl.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-musl.tar.gz)                                                                                                                                                                                                                                            | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-unknown-linux-musl.tar.gz), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-unknown-linux-musl.tar.gz)                                                                                                                                                                                                                                                          |
| macOS (Intel i Apple Silicon) | [universal](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-universal-apple-darwin.tar.gz)                                                                                                                                                                                                                                                                                                                                                                 | [universal](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-universal-apple-darwin.tar.gz)                                                                                                                                                                                                                                                                                                                                                                                      |
| Windows                       | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-pc-windows-msvc.zip), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-pc-windows-msvc.zip), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-pc-windows-msvc.zip)                                                                                                                                           | [x64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-pc-windows-msvc.zip), [ARM64](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-pc-windows-msvc.zip), [x86](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-i686-pc-windows-msvc.zip)                                                                                                                                                  |

## Konfiguracja

Ustawienia są czytane ze zmiennych środowiskowych albo z pliku `.env`.

| Zmienna                 | Wymagana                | Domyślnie      | Znaczenie                                                                               |
| ----------------------- | ----------------------- | -------------- | --------------------------------------------------------------------------------------- |
| `AHE_USERNAME`          | tylko w dedykowanym     | -              | login do WPS                                                                            |
| `AHE_PASSWORD`          | tylko w dedykowanym     | -              | hasło do WPS                                                                            |
| `AHE_LINK_KEY`          | tylko we współdzielonym | -              | 32-bajtowy klucz base64, którym szyfrowane są linki (`openssl rand -base64 32`)         |
| `AHE_CAL_TOKEN`         | nie                     | -              | token wymagany do odczytu kalendarza, zobacz [niżej](#token-dostępu)                    |
| `AHE_CAL_PAST_DAYS`     | nie                     | `60`           | ile dni wstecz pobierać, gdy nie podano `from`                                          |
| `AHE_CAL_FUTURE_DAYS`   | nie                     | `60`           | ile dni do przodu pobierać, gdy nie podano `to`                                         |
| `AHE_CAL_LANG`          | nie                     | `pl`           | język generowanych etykiet, `pl` albo `en`                                              |
| `AHE_CAL_EXAMS_ENABLED` | nie                     | `true`         | `false` pomija egzaminy, zobacz [Egzaminy](#egzaminy)                                   |
| `AHE_CAL_JSON_ENABLED`  | nie                     | `true`         | `false` wyłącza endpointy `.json`                                                       |
| `BIND_ADDR`             | nie                     | `0.0.0.0:8080` | adres, na którym nasłuchuje serwer                                                      |
| `REAL_IP_HEADER`        | nie                     | -              | nagłówek z IP klienta za proxy, np. `CF-Connecting-IP`                                  |
| `HTTPS_PROXY`           | nie                     | -              | proxy dla zapytań do WPS, `http://` albo `socks5h://` (Tor: `socks5h://127.0.0.1:9050`) |
| `RUST_LOG`              | nie                     | `info`         | poziom logów                                                                            |

### Token dostępu

Bez `AHE_CAL_TOKEN` każdy, kto zna adres, może odczytać twój plan. Ustaw go zawsze, gdy instancja jest dostępna z internetu.

Wartość może być zwykłym tekstem albo hashem Argon2id (`$argon2id$...`, opcjonalnie z przedrostkiem `argon2:`). Klient zawsze wysyła token jawnie: jako `?token=...`, w nagłówku `X-Calendar-Token` albo jako `Authorization: Bearer ...`.

<details>
<summary>Generowanie hasha Argon2id</summary>

```bash
docker run --rm -e TOKEN='twoj-token' python:3.15-alpine sh -lc "pip install --quiet argon2-cffi && python - <<'PY'
import os
from argon2 import PasswordHasher

print(PasswordHasher(time_cost=3, memory_cost=65536, parallelism=1).hash(os.environ['TOKEN']))
PY"
```

</details>

## Endpointy

| Endpoint                                  | Opis                                                                                                    |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `GET /calendar.ics`, `/calendar/me.ics`   | kanał ICS                                                                                               |
| `GET /calendar.json`, `/calendar/me.json` | dane, z których budowany jest kanał                                                                     |
| `GET /healthz`                            | zawsze `204`, nie odpytuje WPS                                                                          |
| `GET /readyz`                             | `204`, jeśli logowanie do WPS działa, `503`, jeśli nie (tylko w dedykowanym)                            |
| `POST /link`                              | tworzy link do subskrypcji (tylko we współdzielonym, zobacz [niżej](#linki-w-wariancie-współdzielonym)) |

Endpointy kalendarza przyjmują `from=YYYY-MM-DD`, `to=YYYY-MM-DD` i `token`, np. `/calendar.ics?from=2026-10-01&to=2027-02-28`.

## Subskrypcja

Dodaj adres kanału do kalendarza:

- **Kalendarz Google**: Inne kalendarze → Z adresu URL
- **Kalendarz Apple**: Plik → Nowa subskrypcja kalendarza
- **Outlook**: Dodaj kalendarz → Subskrybuj z sieci Web

Aplikacja kalendarza pobiera kanał ze swoich serwerów, więc adres musi być publiczny. `localhost` nie zadziała.

## Linki w wariancie współdzielonym

W wariancie współdzielonym login do WPS każdego użytkownika trafia do linku, zaszyfrowany kluczem `AHE_LINK_KEY`. `POST /link` sprawdza login w WPS i zwraca ścieżkę:

```bash
curl https://twoja-domena.example/link -d username=jan.kowalski --data-urlencode password='haslo'
# /calendar.ics?c=AXk3...
```

Jeśli ustawiony jest `AHE_CAL_TOKEN`, dodaj też `-d token=...`. Dopisz swoją domenę z przodu i zasubskrybuj `https://twoja-domena.example/calendar.ics?c=...`. `/link` przyjmuje łącznie 10 zapytań na minutę, więc nie da się nim zgadywać haseł do WPS.

Link przestaje działać po zmianie hasła do WPS albo `AHE_LINK_KEY`. Wtedy wygeneruj nowy.

### Prywatność

- Serwer trzyma tylko cache w pamięci (tokeny WPS, dane studenta, wygenerowane kalendarze). Po restarcie znika. Żadnej bazy, żadnych plików.
- Hasła, loginy, query stringi i wartości `c` nigdy nie trafiają do logów. W logach jest IP klienta, a gdy WPS zwróci błąd, jego status i treść odpowiedzi.
- Link może trafić do logów reverse proxy albo Cloudflare, do dostawcy kalendarza i do historii przeglądarki. Kto go zdobędzie, zobaczy twój plan, ale nie hasło, bo bez `AHE_LINK_KEY` nie da się go odszyfrować.
- Serwer odszyfrowuje hasło i loguje się do WPS w twoim imieniu, więc musisz ufać temu, kto go prowadzi. Kto ma i klucz, i link, odzyska hasło.

## Egzaminy

WPS czasem zwraca wpisy, które wyglądają jak egzaminy, ale nie są tymi, które chcesz mieć w kalendarzu, i nie ma pola, po którym dałoby się je pewnie odfiltrować. Jeśli tak jest u ciebie, ustaw `AHE_CAL_EXAMS_ENABLED=false`.

## Zastrzeżenie

Projekt nieoficjalny i niezwiązany z AHE. Jeśli WPS zmieni API, przestanie działać, dopóki się go tu nie poprawi.
