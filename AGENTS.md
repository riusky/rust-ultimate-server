# AGENTS.md

## Cursor Cloud specific instructions

### Project Overview

CMX Server is a Rust (Axum) + Vue 3 full-stack web application with user authentication, RBAC, and REST/JSON-RPC dual-protocol APIs. See `README.md` and `docs/DEVELOPMENT.md` for full documentation.

### Services

| Service | Port | Command | Required |
|---------|------|---------|----------|
| **PostgreSQL 17** | 5432 | `sudo pg_ctlcluster 17 main start` | Yes |
| **Rust web-server** | 8080 | `SERVICE_PERMISSION_CACHE_ENABLED=false cargo run -p web-server` | Yes |
| **Vue 3 frontend** | 3000 | `cd cmx-vue-ultimate-starter && bun run dev` | Yes |
| **Valkey/Redis** | 6379 | Not installed by default | No (set `SERVICE_PERMISSION_CACHE_ENABLED=false`) |

### Non-obvious Gotchas

- **Rust edition 2024**: Vendor crates (`vendor/rpc-router`) use Rust edition 2024, requiring Rust 1.85.0+. The environment has stable Rust installed.
- **Disable Valkey**: Since Valkey/Redis is not installed, always set `SERVICE_PERMISSION_CACHE_ENABLED=false` when running the backend or tests. This env var is set to `true` in `.cargo/config.toml`, so it must be overridden.
- **Database auto-init**: The web-server automatically initializes the dev database on first startup (runs `sql/dev_initial/00-recreate-db.sql` and Refinery migrations). Demo users `demo1`/`demo2` are created with password `demo`.
- **TypeScript type generation**: After modifying Rust model structs, regenerate TS types: `cargo test -p lib-core --features with-ts export_ts_types && cd cmx-vue-ultimate-starter && bun run gen:types`
- **PostgreSQL auth**: pg_hba.conf is configured for `trust` (local) and `md5` (TCP). Connection strings use `localhost:5432`.
- **Cargo test scope**: Run `cargo test -p web-server -p lib-core -p lib-auth -p lib-web -p lib-rpc-core -p lib-rest-core -p lib-macros -p lib-valkey-core -p lib-utils -p gen-key` to avoid building unrelated transitive dependencies (sqlx-sqlite).
- **Clippy on vendor**: Vendored crates (`vendor/modql`) produce clippy warnings on newer Rust versions. Target workspace crates explicitly when running clippy.
- **Frontend lint (pre-existing)**: `bun run lint` in `cmx-vue-ultimate-starter` has 4 pre-existing oxlint errors (unused imports). Not an environment issue.
- **libstdc++ symlink**: A symlink `/usr/lib/x86_64-linux-gnu/libstdc++.so` → `libstdc++.so.6` is needed for native C++ builds (cmake-based dependencies).
- **Bun path**: Bun is installed at `$HOME/.bun/bin/bun`. Make sure `$HOME/.bun/bin` is in `PATH`.

### Standard Commands Reference

See `README.md` § "常用命令" and `docs/DEVELOPMENT.md` § "二、开发环境" for lint/test/build/run commands.
