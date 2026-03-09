# Security Policy

## Supported versions

Only the latest release receives security fixes.

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report vulnerabilities privately via [GitHub Security Advisories](https://github.com/wielorzeczownik/ahe-ics/security/advisories/new).

Include as much detail as possible:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You will receive a response within **7 days**. If the issue is confirmed, a fix will be released as soon as possible and you will be credited in the release notes (unless you prefer to remain anonymous).

## Scope

This project is a self-hosted service. The attack surface includes:

- HTTP endpoints exposed by the service
- Handling of the `AHE_CAL_TOKEN` access token
- Credential handling (`AHE_USERNAME` / `AHE_PASSWORD`)

Issues related to the upstream AHE/WPS API are out of scope.

## Security notes for operators

- Always set `AHE_CAL_TOKEN` when exposing the service on a public network.
- Prefer using an Argon2id hash over a plain token value see [README.md](README.md) for generation instructions.
- Run behind a reverse proxy with TLS termination.
