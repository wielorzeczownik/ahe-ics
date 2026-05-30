<h1 align="center">AHE-ICS</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/ahe-ics/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/ahe-ics/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Najnowsze wydanie"/></picture></a> <a href="https://hub.docker.com/r/wielorzeczownik/ahe-ics"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&color=2ea043"/><img src="https://img.shields.io/docker/pulls/wielorzeczownik/ahe-ics?style=flat-square&labelColor=2d333b&color=3fb950" alt="Docker Pulls"/></picture></a> <a href="https://github.com/wielorzeczownik/ahe-ics/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="Licencja: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/assets/logo.png" alt="AHE-ICS logo" width="300" />
</p>

<p align="center">🇬🇧 <a href="README.md">English</a> | 🇵🇱 Polski</p>

Lekki serwis, który eksportuje plan zajęć [Akademii Humanistyczno-Ekonomicznej (AHE) w Łodzi](https://www.ahe.lodz.pl) z systemu [Wirtualny Pokój Studenta (WPS)](https://wps.ahe.lodz.pl) jako subskrybowalny kanał ICS – kompatybilny z Google Calendar, Apple Calendar i Outlookiem.

Wystarczy dodać jeden adres URL, a plan zajęć AHE będzie automatycznie aktualizowany w dowolnej aplikacji kalendarza.

Dostępne są dwa warianty:

| Wariant           | Binarka          | Obraz Docker                     | Dane logowania                                                       |
| ----------------- | ---------------- | -------------------------------- | -------------------------------------------------------------------- |
| **Dedykowany**    | `ahe-ics`        | `wielorzeczownik/ahe-ics`        | Ustawiane raz w zmiennych środowiskowych – jedno konto na instancję  |
| **Współdzielony** | `ahe-ics-shared` | `wielorzeczownik/ahe-ics:shared` | Przekazywane w każdym żądaniu w URL – bezstanowy, bez przechowywania |

## Wariant dedykowany (`ahe-ics`)

### Uruchomienie przez Docker (zalecane)

Uruchomienie przez Docker:

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
# edytuj .env i ustaw AHE_USERNAME / AHE_PASSWORD
docker run --rm -p 8080:8080 --env-file .env wielorzeczownik/ahe-ics:latest
```

Uruchomienie przez Docker Compose:

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
curl -fsSL -o docker-compose.yml https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/docker-compose.example.yml
# edytuj .env i ustaw AHE_USERNAME / AHE_PASSWORD
docker compose pull
docker compose up -d
```

### Uruchomienie z gotowych plików binarnych

Każde wydanie zawiera gotowe archiwa dla systemu Linux, macOS i Windows.
Najnowsze wydanie: [GitHub Releases](https://github.com/wielorzeczownik/ahe-ics/releases/latest)

1. Pobierz archiwum `ahe-ics-*` dla swojej platformy ze strony wydania.
2. Rozpakuj je.
3. Utwórz plik `.env` na podstawie `.env.example` i ustaw `AHE_USERNAME` / `AHE_PASSWORD`.
4. Uruchom program.

Przykład (Linux/macOS):

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example
# edytuj .env
./ahe-ics
```

Przykład (Windows PowerShell):

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.example" -OutFile ".env"
# edytuj .env
.\ahe-ics.exe
```

Pobierz najnowsze archiwum dla swojej platformy:

**Linux (glibc – wymaga glibc 2.35+):**

- [ahe-ics-x86_64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 64-bit)
- [ahe-ics-aarch64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-gnu.tar.gz) – Linux (ARM64, np. Raspberry Pi 64-bit)
- [ahe-ics-armv7-unknown-linux-gnueabihf.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-armv7-unknown-linux-gnueabihf.tar.gz) – Linux (ARM 32-bit, np. Raspberry Pi 32-bit)
- [ahe-ics-i686-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 32-bit)

**Linux (musl – w pełni statyczne, bez zależności od glibc):**

- [ahe-ics-x86_64-unknown-linux-musl.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-unknown-linux-musl.tar.gz) – Linux (Intel/AMD 64-bit)
- [ahe-ics-aarch64-unknown-linux-musl.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-unknown-linux-musl.tar.gz) – Linux (ARM64)

**macOS:**

- [ahe-ics-x86_64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-apple-darwin.tar.gz) – macOS na Intel
- [ahe-ics-aarch64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-apple-darwin.tar.gz) – macOS na Apple Silicon (M1/M2/M3/M4)

**Windows:**

- [ahe-ics-x86_64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-x86_64-pc-windows-msvc.zip) – Windows 64-bit (x86_64)
- [ahe-ics-aarch64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-aarch64-pc-windows-msvc.zip) – Windows ARM64
- [ahe-ics-i686-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-i686-pc-windows-msvc.zip) – Windows 32-bit (x86)

### Zmienne środowiskowe

| Zmienna                 | Wymagana | Domyślna       | Opis                                                                                            |
| ----------------------- | -------- | -------------- | ----------------------------------------------------------------------------------------------- |
| `AHE_USERNAME`          | tak      | -              | Nazwa użytkownika [WPS](https://wps.ahe.lodz.pl/)                                               |
| `AHE_PASSWORD`          | tak      | -              | Hasło [WPS](https://wps.ahe.lodz.pl/)                                                           |
| `BIND_ADDR`             | nie      | `0.0.0.0:8080` | Adres i port serwera HTTP                                                                       |
| `AHE_CAL_PAST_DAYS`     | nie      | `60`           | Domyślny zakres: liczba dni wstecz, gdy `from` nie jest podane                                  |
| `AHE_CAL_FUTURE_DAYS`   | nie      | `60`           | Domyślny zakres: liczba dni wprzód, gdy `to` nie jest podane                                    |
| `AHE_CAL_LANG`          | nie      | `pl`           | Język etykiet w kalendarzu (`pl` lub `en`)                                                      |
| `AHE_CAL_EXAMS_ENABLED` | nie      | `true`         | Włącz lub wyłącz pobieranie egzaminów (`true`/`false`); przydatne gdy wpisy egzaminów są mylące |
| `AHE_CAL_JSON_ENABLED`  | nie      | `true`         | Włącz lub wyłącz endpointy JSON (`/calendar.json`, `/calendar/me.json`)                         |
| `AHE_CAL_TOKEN`         | nie      | -              | Opcjonalny token dostępu do endpointów kalendarza (zwykły ciąg lub hash Argon2id)               |
| `REAL_IP_HEADER`        | nie      | -              | Nagłówek z adresem IP klienta (np. `CF-Connecting-IP`, `X-Forwarded-For`, `Forwarded`)          |
| `RUST_LOG`              | nie      | `info`         | Poziom logowania (`debug`, `info` itp.)                                                         |

### Endpointy

- `GET /calendar.ics` – główny endpoint kanału ICS (`text/calendar`).
- `GET /calendar/me.ics` – alias `/calendar.ics` (identyczny wynik).
- `GET /calendar.json` – JSON z danymi źródłowymi kalendarza (gdy `AHE_CAL_JSON_ENABLED=true`).
- `GET /calendar/me.json` – alias `/calendar.json` (gdy `AHE_CAL_JSON_ENABLED=true`).
- `GET /healthz` – health check weryfikujący połączenie z API AHE (zwraca `204 No Content`, w przeciwnym razie `503`).

Parametry zapytania (`/calendar.ics`, `/calendar/me.ics` i endpointy JSON):

- `from=RRRR-MM-DD` – data początkowa; gdy pominięta, serwis używa `AHE_CAL_PAST_DAYS`.
- `to=RRRR-MM-DD` – data końcowa; gdy pominięta, serwis używa `AHE_CAL_FUTURE_DAYS`.
- `token=...` – opcjonalny token dostępu, jeśli skonfigurowano `AHE_CAL_TOKEN`.

Przykład:

```text
http://localhost:8080/calendar.ics?from=2026-01-01&to=2026-03-01
```

## Wariant współdzielony (`ahe-ics-shared`)

Wariant współdzielony jest w pełni bezstanowy – żadne dane logowania nie są przechowywane ani logowane po stronie serwera. Każde żądanie przekazuje własną nazwę użytkownika i hasło WPS w parametrach URL. Serwer przechowuje jedynie pamięci podręczną (tokeny dostępu WPS i metadane studenta, kluczowane po nazwie użytkownika), które są kasowane przy restarcie.

### Uruchomienie przez Docker

Uruchomienie przez Docker:

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.shared.example
# edytuj .env i ustaw AHE_CAL_TOKEN
docker run --rm -p 8080:8080 --env-file .env wielorzeczownik/ahe-ics:shared
```

Uruchomienie przez Docker Compose:

```bash
curl -fsSL -o .env https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/.env.shared.example
curl -fsSL -o docker-compose.yml https://raw.githubusercontent.com/wielorzeczownik/ahe-ics/main/docker-compose.shared.example.yml
# edytuj .env i ustaw AHE_CAL_TOKEN
docker compose pull
docker compose up -d
```

Zmienne `AHE_USERNAME` ani `AHE_PASSWORD` nie są potrzebne.

### Uruchomienie z gotowych plików binarnych

Pobierz archiwum `ahe-ics-shared-*` dla swojej platformy z [najnowszego wydania](https://github.com/wielorzeczownik/ahe-ics/releases/latest).

**Linux (glibc – wymaga glibc 2.35+):**

- [ahe-ics-shared-x86_64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 64-bit)
- [ahe-ics-shared-aarch64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-unknown-linux-gnu.tar.gz) – Linux (ARM64)
- [ahe-ics-shared-armv7-unknown-linux-gnueabihf.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-armv7-unknown-linux-gnueabihf.tar.gz) – Linux (ARM 32-bit)
- [ahe-ics-shared-i686-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-i686-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 32-bit)

**Linux (musl – w pełni statyczne, bez zależności od glibc):**

- [ahe-ics-shared-x86_64-unknown-linux-musl.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-unknown-linux-musl.tar.gz) – Linux (Intel/AMD 64-bit)
- [ahe-ics-shared-aarch64-unknown-linux-musl.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-unknown-linux-musl.tar.gz) – Linux (ARM64)

**macOS:**

- [ahe-ics-shared-x86_64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-apple-darwin.tar.gz) – macOS na Intel
- [ahe-ics-shared-aarch64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-apple-darwin.tar.gz) – macOS na Apple Silicon (M1/M2/M3/M4)

**Windows:**

- [ahe-ics-shared-x86_64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-x86_64-pc-windows-msvc.zip) – Windows 64-bit
- [ahe-ics-shared-aarch64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-aarch64-pc-windows-msvc.zip) – Windows ARM64
- [ahe-ics-shared-i686-pc-windows-msvc.zip](https://github.com/wielorzeczownik/ahe-ics/releases/latest/download/ahe-ics-shared-i686-pc-windows-msvc.zip) – Windows 32-bit

### Zmienne środowiskowe

Takie same jak w wariancie dedykowanym **z wyjątkiem** `AHE_USERNAME` i `AHE_PASSWORD` – nie są używane i nie powinny być ustawiane.

| Zmienna                 | Wymagana | Domyślna       | Opis                                                                                            |
| ----------------------- | -------- | -------------- | ----------------------------------------------------------------------------------------------- |
| `BIND_ADDR`             | nie      | `0.0.0.0:8080` | Adres i port serwera HTTP                                                                       |
| `AHE_CAL_PAST_DAYS`     | nie      | `60`           | Domyślny zakres: liczba dni wstecz, gdy `from` nie jest podane                                  |
| `AHE_CAL_FUTURE_DAYS`   | nie      | `60`           | Domyślny zakres: liczba dni wprzód, gdy `to` nie jest podane                                    |
| `AHE_CAL_LANG`          | nie      | `pl`           | Język etykiet w kalendarzu (`pl` lub `en`)                                                      |
| `AHE_CAL_EXAMS_ENABLED` | nie      | `true`         | Włącz lub wyłącz pobieranie egzaminów (`true`/`false`); przydatne gdy wpisy egzaminów są mylące |
| `AHE_CAL_JSON_ENABLED`  | nie      | `true`         | Włącz lub wyłącz endpointy JSON (`/calendar.json`, `/calendar/me.json`)                         |
| `AHE_CAL_TOKEN`         | nie      | -              | Opcjonalny token ograniczający dostęp do endpointów                                             |
| `REAL_IP_HEADER`        | nie      | -              | Nagłówek z adresem IP klienta (np. `CF-Connecting-IP`, `X-Forwarded-For`, `Forwarded`)          |
| `RUST_LOG`              | nie      | `info`         | Poziom logowania (`debug`, `info` itp.)                                                         |

### Endpointy

Takie same jak w wariancie dedykowanym, z dwoma **wymaganymi** dodatkowymi parametrami we wszystkich endpointach kalendarza:

- `username=...` – nazwa użytkownika WPS.
- `password=...` – hasło WPS.

Przykład:

```text
https://twoja-domena.example/calendar.ics?username=jan.kowalski&password=haslo
https://twoja-domena.example/calendar.ics?username=jan.kowalski&password=haslo&from=2026-01-01&to=2026-03-01
```

- `GET /healthz` – zawsze zwraca `204 No Content` (brak sprawdzenia upstream; nie ma stałych danych logowania do testowania).

> [!WARNING]
> Dane logowania pojawiają się w parametrach zapytania URL, co oznacza, że mogą być zapisywane w logach dostępu serwera, historii przeglądarki i logach proxy. Używaj HTTPS, aby zapobiec ich przechwyceniu podczas transmisji. Jeśli Twój reverse proxy loguje pełne adresy URL, rozważ wyłączenie logowania dostępu lub maskowanie parametru `password`.

## Wspólna konfiguracja

### `AHE_CAL_TOKEN`

`AHE_CAL_TOKEN` ogranicza dostęp do endpointów kalendarza. Oba warianty go obsługują.

Obsługiwane formaty:

- zwykły token (np. `AHE_CAL_TOKEN=moj-sekret`),
- hash Argon2id (`$argon2id$...`),
- tryb jawny Argon2id (`AHE_CAL_TOKEN=argon2:$argon2id$...`).

W trybie hasha klient wysyła token jako zwykły tekst (`token=...`), a serwer weryfikuje go względem hasha.

Przykład generowania hasha Argon2id przez Docker:

```bash
docker run --rm -e TOKEN='twoj-token' python:3.14-alpine sh -lc "pip install --quiet argon2-cffi && python - <<'PY'
import os
from argon2 import PasswordHasher

print(PasswordHasher(time_cost=3, memory_cost=65536, parallelism=1).hash(os.environ['TOKEN']))
PY"
```

### Subskrypcja w Google Calendar / Apple Calendar / Outlook

Dodaj adres URL kanału ICS w swojej aplikacji kalendarza – plan zajęć będzie synchronizowany automatycznie:

- **Google Calendar** → Inne kalendarze → Dodaj przez URL
- **Apple Calendar** → Plik → Nowa subskrypcja kalendarza
- **Outlook** → Dodaj kalendarz → Subskrybuj z sieci Web

Przy subskrypcji z zewnętrznych klientów użyj publicznego adresu URL. `localhost` działa tylko lokalnie.

> [!CAUTION]
> Jeśli udostępniasz wariant dedykowanym na domenie lub publicznym IP, ustaw `AHE_CAL_TOKEN`, aby zabezpieczyć swój kanał. Następnie dołącz go do adresu URL subskrypcji, np.: `https://twoja-domena.example/calendar.ics?token=twoj-token`

### Ograniczenia danych egzaminacyjnych

Dane egzaminacyjne pochodzą z endpointów [WPS](https://wps.ahe.lodz.pl/), które mogą zwracać wpisy wyglądające na poprawne, ale niebędące egzaminami, których szukasz (fałszywe trafienia). Nie istnieje w pełni niezawodny sygnał API, który pozwoliłby odfiltrować wszystkie takie przypadki.

Jeśli problem dotyczy Twojego konta, wyłącz egzaminy całkowicie:

- `AHE_CAL_EXAMS_ENABLED=false`

## Obsługiwane platformy

- Linux: testowany, traktowany jako główne środowisko uruchomieniowe.
- Windows: gotowe pliki binarne są publikowane i powinny działać, jednak środowisko Windows nie jest regularnie weryfikowane.
- macOS: powinno działać przy uruchomieniu ze źródeł (ten sam stack co Linux).

## Zastrzeżenie

Projekt jest tworzony przez społeczność, jest nieoficjalny i może przestać działać w przypadku zmiany backendowego API.
