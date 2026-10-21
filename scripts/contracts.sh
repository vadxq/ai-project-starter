#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
mode="${1:?Use check or generate}"
if [[ "$#" -ne 1 || ! "$mode" =~ ^(check|generate)$ ]]; then
  printf '%s\n' 'Usage: bash scripts/contracts.sh <check|generate>' >&2
  exit 2
fi
mkdir -p "$root/.ailog"
scratch="$(mktemp -d "$root/.ailog/contracts.XXXXXX")"
trap 'rm -rf "$scratch"' EXIT
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.ailog/target}"
(cd "$root/backend" && cargo run --quiet --locked --release --example export-openapi) > "$scratch/openapi.json"

files=(docs/contracts/openapi.json spa/openapi/openapi.json next/openapi/openapi.json android/app/openapi/openapi.json ios/Packages/APIClient/Sources/APIClient/openapi.json)
case "$mode" in
  generate)
    for path in "${files[@]}"; do cp "$scratch/openapi.json" "$root/$path"; done
    for project in spa next; do npm --prefix "$root/$project" run generate:api; done
    ;;
  check)
    for path in "${files[@]}"; do cmp "$scratch/openapi.json" "$root/$path"; done
    for project in spa next; do
      (cd "$root/$project" && npm exec -- openapi-ts --output "$scratch/$project")
      generated="$root/$project/src/api/generated"
      if [[ "$project" == next ]]; then generated="$root/$project/lib/api/generated"; fi
      diff -qr "$generated" "$scratch/$project"
    done
    ;;
  *) printf '%s\n' 'Use check or generate' >&2; exit 2 ;;
esac
