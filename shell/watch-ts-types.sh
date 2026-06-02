#!/usr/bin/env bash
set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd -P)"

if ! cargo watch --version >/dev/null 2>&1; then
	echo "cargo-watch is required. Install with: cargo install cargo-watch" >&2
	exit 1
fi

cd "${REPO_ROOT}"

cargo watch \
	-q \
	-c \
	-w "crates/libs/lib-core/src/model" \
	-w "vendor/modql/src/filter" \
	-s "bash shell/gen-ts-types.sh"
