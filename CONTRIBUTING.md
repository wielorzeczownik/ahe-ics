# Contributing to AHE-ICS

Thank you for considering a contribution. This document describes how to get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [shfmt](https://github.com/mvdan/sh)
- [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2)
- [Docker](https://www.docker.com/) (optional, for container testing)
- [hadolint](https://github.com/hadolint/hadolint) (optional, for Dockerfile linting)
- [trivy](https://github.com/aquasecurity/trivy) (optional, for vulnerability scanning)

## Development setup

```bash
git clone https://github.com/wielorzeczownik/ahe-ics.git
cd ahe-ics
cp .env.example .env
# Edit .env and set AHE_USERNAME / AHE_PASSWORD
cargo run
```

## Project structure

- `src/` – Rust source code
- `Dockerfile` – Debian-based container image
- `Dockerfile.alpine` – Alpine-based container image
- `scripts/bump-version.sh` – determines the next release version from git-cliff output and bumps `Cargo.toml`

## Before submitting a PR

Run all checks locally before opening a pull request.

### With tools installed locally

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets --locked
cargo audit
hadolint Dockerfile Dockerfile.alpine
shfmt --diff scripts/
markdownlint-cli2 "**/*.md"
```

### With Docker (no local installs required)

```bash
docker run --rm -v "$(pwd):/src" -w /src hadolint/hadolint hadolint Dockerfile Dockerfile.alpine

docker run --rm -v "$(pwd):/src" -w /src mvdan/shfmt --diff scripts/

docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"
```

The CI runs all of the above plus a Docker smoke build and a vulnerability scan of the resulting image.

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

Common prefixes:

| Prefix      | When to use                         |
| ----------- | ----------------------------------- |
| `feat:`     | New feature or endpoint             |
| `fix:`      | Bug fix                             |
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
