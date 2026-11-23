# Security

Thanks for taking the time to look. Please report vulnerabilities privately through [GitHub Security Advisories](https://github.com/wielorzeczownik/ahe-ics/security/advisories/new) instead of a public issue, so there's a chance to fix them first. You'll be credited in the release notes unless you'd rather not be. Only the latest release gets fixes.

## In scope

- the HTTP endpoints
- `AHE_CAL_TOKEN` checks
- WPS credentials: env vars in the dedicated variant, encrypted links and `AHE_LINK_KEY` in the shared one
- the `/link` rate limit

Problems in WPS itself are out of scope.

## Running it safely

- Set `AHE_CAL_TOKEN` if the instance is reachable from the internet, preferably as an Argon2id hash (see [README](README.md#access-token)).
- Put it behind a reverse proxy with TLS.
- In the shared variant, keep `AHE_LINK_KEY` secret. Together with any link it reveals that user's WPS password.
