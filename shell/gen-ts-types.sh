#!/usr/bin/env bash
set -Eeuo pipefail

CURRENT_STEP="initializing"

on_error() {
	local status=$?
	echo "TypeScript type generation failed during: ${CURRENT_STEP}" >&2
	exit "${status}"
}

trap on_error ERR

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd -P)"
FRONTEND_DIR="${REPO_ROOT}/cmx-vue-ultimate-starter"

export SERVICE_PERMISSION_CACHE_ENABLED=false

CURRENT_STEP="Rust ts-rs export"
echo "==> Exporting Rust types with ts-rs"
(
	cd "${REPO_ROOT}"
	cargo test -p lib-core --features with-ts export_ts_types -- --nocapture
)

CURRENT_STEP="frontend type post-processing"
echo "==> Post-processing frontend TypeScript types"
(
	cd "${FRONTEND_DIR}"
	bun run gen:types
)

trap - ERR
echo "==> TypeScript type generation complete"
