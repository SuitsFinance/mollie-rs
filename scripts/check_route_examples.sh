#!/usr/bin/env sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

if command -v python3 >/dev/null 2>&1; then
    python3 "$root/scripts/route_examples.py" check --root "$root"
elif command -v python >/dev/null 2>&1; then
    python "$root/scripts/route_examples.py" check --root "$root"
else
    echo "python3 or python is required to check route examples." >&2
    exit 1
fi
