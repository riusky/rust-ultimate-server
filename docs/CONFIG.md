# Configuration

The service uses layered TOML configuration plus environment variable overrides.

Load order:

```text
config/default.toml
config/{SERVICE_ENV}.toml      # default SERVICE_ENV=development
SERVICE_CONFIG_FILE            # optional extra TOML file
config/local.toml              # optional, ignored by Git
legacy environment variables   # highest priority
```

## Files

- `config/default.toml`: shared safe defaults.
- `config/development.toml`: local development defaults.
- `config/docker.toml`: Docker Compose defaults.
- `config/local.example.toml`: template for local secrets/overrides.
- `config/local.toml`: machine-local overrides, ignored by Git.

## Docker

Docker Compose sets:

```text
SERVICE_ENV=docker
SERVICE_CONFIG_DIR=/app/config
```

Non-sensitive Docker settings are in `config/docker.toml`. Secrets such as
`SERVICE_PWD_KEY`, `SERVICE_TOKEN_KEY`, and `INIT_ADMIN_PASSWORD` stay in
environment variables or a secret manager.

## Local Development

Local cargo commands use `SERVICE_ENV=development` from `.cargo/config.toml`.
`config/development.toml` disables Valkey-backed permission and model caches by
default, so local startup does not require Valkey/Redis.

Run:

```bash
cargo run -p web-server
```

## Environment Overrides

The existing environment variable names still work and override TOML files:

```text
SERVICE_DB_URL
SERVICE_WEB_FOLDER
SERVICE_PWD_KEY
SERVICE_TOKEN_KEY
SERVICE_TOKEN_DURATION_SEC
SERVICE_VALKEY_URL
SERVICE_VALKEY_POOL_MAX_SIZE
SERVICE_VALKEY_POOL_MIN_IDLE
SERVICE_PERMISSION_CACHE_ENABLED
SERVICE_MODEL_CACHE_ENABLED
INIT_ADMIN_USERNAME
INIT_ADMIN_PASSWORD
```

Use environment variables or `config/local.toml` for secrets. Do not put
production secrets in committed TOML files.
