#!/usr/bin/env bash
set -uo pipefail

: "${GITHUB_OUTPUT:?GITHUB_OUTPUT is required}"

report="${REPORT_FILE:-audit-report.txt}"

emit() {
  echo "$1=$2" >>"$GITHUB_OUTPUT"
}

emit_report() {
  {
    echo 'report<<AUDIT_REPORT_EOF'
    if [[ -s "$report" ]]; then
      cat "$report"
    else
      echo 'cargo audit produced no output, see the workflow logs.'
    fi
    echo 'AUDIT_REPORT_EOF'
  } >>"$GITHUB_OUTPUT"
}

if cargo audit 2>&1 | tee "$report"; then
  emit changed false
  emit unresolved false
  emit_report
  exit 0
fi

echo "Advisories found, attempting cargo audit fix"
cargo audit fix || echo "cargo audit fix could not resolve everything"

changed=false
if ! git diff --quiet -- Cargo.lock; then
  changed=true
fi

unresolved=true
if cargo audit 2>&1 | tee "$report"; then
  unresolved=false
fi

emit changed "$changed"
emit unresolved "$unresolved"
emit_report
echo "lockfile changed: $changed, advisories unresolved: $unresolved"
