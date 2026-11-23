# Contributing

Every contribution helps, including a typo fix, a bug report, or a question about how something works. You don't need to be an AHE student to help. For anything bigger than a small fix, opening an issue first is a good idea, so we can agree on the approach before you spend time on it.

## Setup

```bash
git clone https://github.com/wielorzeczownik/ahe-ics.git
cd ahe-ics
cp .env.example .env          # set AHE_USERNAME and AHE_PASSWORD
cargo run --bin ahe-ics
```

For the shared variant, copy `.env.shared.example` instead and run `cargo run --bin ahe-ics-shared`.

`Dockerfile` and `Dockerfile.alpine` build from source and are what pull requests build in CI. They build `ahe-ics` by default, pass `--build-arg BIN=ahe-ics-shared` for the other one. The `*.release` Dockerfiles only package binaries that the release workflow has already built.

## Checks

CI runs these on every pull request. You don't have to run them all locally, since CI will point out anything that's off:

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked

hadolint Dockerfile Dockerfile.alpine Dockerfile.release Dockerfile.alpine.release
shfmt --diff scripts/
shellcheck scripts/*.sh
actionlint
markdownlint-cli2 "**/*.md"
npx prettier --check .
```

Without the tools installed, run them in Docker, e.g.:

```bash
docker run --rm -v "$(pwd):/src" -w /src hadolint/hadolint hadolint Dockerfile
docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"
```

## Tests

Tests sit next to the code they cover, in a `#[cfg(test)] mod tests` in the same file.

## Commits and pull requests

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat: ...`, `fix: ...`), because the changelog and version bumps are generated from them. Don't worry if you get the prefix wrong, it can be fixed before merging.

Smaller PRs that do one thing are easier to review. Tests for behavior changes are very welcome. If you're not sure how to test something, open the PR anyway and we'll work it out together.

Bugs and ideas go to [issues](https://github.com/wielorzeczownik/ahe-ics/issues). Security problems go through [SECURITY.md](SECURITY.md).

Contributions are licensed under the [MIT License](LICENSE).
