# Contributing to AHE-ICS

Thank you for considering a contribution. This document covers everything you need to get started.

## Overview

A lightweight self-hosted service written in Rust that exports the AHE Łódź class schedule as a subscribable ICS feed, compatible with Google Calendar, Apple Calendar, and Outlook.

## Project structure

```text
.
├── src/                       Rust source code
├── scripts/
│   ├── bump-version.sh        determines next release version from git-cliff and bumps Cargo.toml
│   ├── stage-docker-binaries.sh  lays release binaries out per platform for the release images
│   ├── smoke-test-image.sh    starts a built image and waits for it to serve HTTP
│   └── security-audit.sh      runs cargo audit and attempts cargo audit fix
├── Dockerfile                 Debian-based image, builds from source (local development)
├── Dockerfile.alpine          Alpine-based image, builds from source (local development)
└── Dockerfile.release         published image, copies in a binary from build-artifacts
```

Published images are assembled from the binaries that the release workflow already
cross-compiles natively, so they never compile anything under QEMU emulation. The
`Dockerfile` / `Dockerfile.alpine` variants stay around as the from-source build path
for local development and are what pull requests smoke-build.

## Development setup

```bash
git clone https://github.com/wielorzeczownik/ahe-ics.git
cd ahe-ics
cp .env.example .env
# Edit .env and set AHE_USERNAME / AHE_PASSWORD
cargo run
```

## Running checks locally

### With tools installed locally

```bash
# Rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets --locked
cargo audit

# Dockerfile
hadolint Dockerfile Dockerfile.alpine Dockerfile.release Dockerfile.alpine.release

# Shell
shfmt --diff scripts/

# Markdown
markdownlint-cli2 "**/*.md"
```

### With Docker (no local installs required)

```bash
docker run --rm -v "$(pwd):/src" -w /src hadolint/hadolint hadolint Dockerfile Dockerfile.alpine Dockerfile.release Dockerfile.alpine.release

docker run --rm -v "$(pwd):/src" -w /src mvdan/shfmt --diff scripts/

docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

Common prefixes:

| Prefix      | When to use                         |
| ----------- | ----------------------------------- |
| `feat:`     | New feature or endpoint             |
| `fix:`      | Bug fix                             |
| `test:`     | Adding or updating tests            |
| `chore:`    | Maintenance, dependency updates     |
| `refactor:` | Code change without behavior change |
| `docs:`     | Documentation only                  |
| `style:`    | Formatting, no logic change         |
| `ci:`       | CI/CD changes                       |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- All CI checks must pass: Rust linting, shell linting, Markdown linting, smoke build, and vulnerability scan.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/ahe-ics/issues) and include:

- What you did
- What you expected
- What actually happened
- Relevant logs or error messages
- Your platform and how you're running the service (Docker, binary, from source)

> For security issues, please read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [MIT License](LICENSE).
