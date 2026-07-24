---
title: Unresolved security advisories
labels: security-advisory
assignees: {{ env.REPO_OWNER }}
---

`cargo audit` reports advisories that `cargo audit fix` could not resolve
automatically, so the lockfile cannot be updated without a manual decision.

## Audit output

```text
{{{ env.AUDIT_REPORT }}}
```
