# 本地启动指南（不使用 Docker）

本文档说明如何在本机直接启动 CMX Server 的后端和前端，不使用 Docker Compose、不启动 Pingora 网关容器。

本地开发链路：

```text
Browser -> Vite dev server (:3000) -> /api proxy -> Axum web-server (:8080) -> PostgreSQL (:5432)
```

Valkey/Redis 是可选服务。默认本地不启动 Valkey，因此后端启动时必须关闭权限缓存。

## 1. 前置依赖

确认本机已安装：

- Rust 1.85.0 或更高版本
- PostgreSQL 15 或更高版本，推荐 PostgreSQL 17
- Bun
- Node.js，用于前端脚本和 `npx tsx`

检查命令：

```bash
rustc --version
cargo --version
psql --version
bun --version
node --version
```

如果 Bun 不在 `PATH`，先加入 `$HOME/.bun/bin`。

## 2. 准备 PostgreSQL

后端 debug 启动时会自动初始化开发数据库：

1. 连接 `DEV_POSTGRES_URL` 指向的 `postgres` 数据库。
2. 执行 `sql/dev_initial/00-recreate-db.sql`，重建 `app_db` 和 `app_user`。
3. 通过 Refinery 执行 `crates/libs/lib-core/migrations/` 下的迁移。
4. 创建默认管理员用户，并把 `admin` 的密码设置为 `admin`。

默认连接配置来自 `.cargo/config.toml`：

```text
DEV_POSTGRES_URL=postgres://postgres:dev_only_pwd@localhost:5432/postgres
SERVICE_DB_URL=postgres://app_user:dev_only_pwd@localhost:5432/app_db
```

因此本地 PostgreSQL 需要满足其中一种方式：

- `postgres` 用户密码是 `dev_only_pwd`。
- 或者启动后端前覆盖 `DEV_POSTGRES_URL` 和 `SERVICE_DB_URL`。

### Linux / WSL 示例

```bash
sudo pg_ctlcluster 17 main start
sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'dev_only_pwd';"
```

### Windows PowerShell 示例

服务名可能因安装版本不同而变化，先查看 PostgreSQL 服务：

```powershell
Get-Service *postgres*
```

启动服务：

```powershell
Start-Service postgresql-x64-17
```

设置 `postgres` 用户密码：

```powershell
psql -U postgres -h localhost -d postgres -c "ALTER USER postgres PASSWORD 'dev_only_pwd';"
```

如果本机已有不同密码，不想修改 `postgres` 用户密码，可以在启动后端前覆盖环境变量：

```powershell
$env:DEV_POSTGRES_URL="postgres://postgres:<your-password>@localhost:5432/postgres"
$env:SERVICE_DB_URL="postgres://app_user:dev_only_pwd@localhost:5432/app_db"
```

## 3. 启动后端

本地默认不使用 Valkey/Redis。由于 `.cargo/config.toml` 默认设置了 `SERVICE_PERMISSION_CACHE_ENABLED=true`，启动后端时必须覆盖为 `false`。

### Linux / macOS / WSL

```bash
SERVICE_PERMISSION_CACHE_ENABLED=false cargo run -p web-server
```

### Windows PowerShell

```powershell
$env:SERVICE_PERMISSION_CACHE_ENABLED="false"
cargo run -p web-server
```

成功后端会监听：

```text
http://localhost:8080
```

启动日志中应能看到：

- `FOR-DEV-ONLY - init_dev_db()`
- `Permission cache DISABLED`
- `LISTENING - 0.0.0.0:8080`

## 4. 启动前端

打开第二个终端：

```bash
cd cmx-vue-ultimate-starter
bun install
bun run dev
```

前端默认监听：

```text
http://localhost:3000
```

前端 `.env` 中的关键配置：

```text
VITE_SERVER_API_URL=http://localhost:3000
VITE_SERVER_API_PREFIX=/api
```

Vite 配置会把 `/api` 代理到后端：

```text
/api -> http://localhost:8080
```

因此浏览器只需要访问 `http://localhost:3000`。

## 5. 登录验证

开发数据库初始化后可以使用：

| 用户 | 密码 | 说明 |
| --- | --- | --- |
| `admin` | `admin` | 管理员用户 |

推荐验证步骤：

1. 打开 `http://localhost:3000`。
2. 进入登录页。
3. 使用 `admin` / `admin` 登录。
4. 访问系统用户、角色、权限相关页面，确认接口能通过 Vite 代理访问后端。

## 6. 热重载方式

如果需要后端文件变更后自动重启，先安装 `cargo-watch`：

```bash
cargo install cargo-watch
```

Linux / macOS / WSL：

```bash
SERVICE_PERMISSION_CACHE_ENABLED=false cargo watch -q -c -w crates/ -x "run -p web-server"
```

Windows PowerShell：

```powershell
$env:SERVICE_PERMISSION_CACHE_ENABLED="false"
cargo watch -q -c -w crates/ -x "run -p web-server"
```

前端继续使用：

```bash
cd cmx-vue-ultimate-starter
bun run dev
```

## 7. 常用开发命令

### 运行后端测试

本地没有 Valkey 时同样要关闭权限缓存：

```bash
SERVICE_PERMISSION_CACHE_ENABLED=false cargo test -p web-server -p lib-core -p lib-auth -p lib-web -p lib-rpc-core -p lib-rest-core -p lib-macros -p lib-valkey-core -p lib-utils -p gen-key
```

PowerShell：

```powershell
$env:SERVICE_PERMISSION_CACHE_ENABLED="false"
cargo test -p web-server -p lib-core -p lib-auth -p lib-web -p lib-rpc-core -p lib-rest-core -p lib-macros -p lib-valkey-core -p lib-utils -p gen-key
```

### 生成 TypeScript 类型

修改 Rust model 后运行：

```bash
cargo test -p lib-core --features with-ts export_ts_types
cd cmx-vue-ultimate-starter
bun run gen:types
```

### 前端构建检查

```bash
cd cmx-vue-ultimate-starter
bun run build
```

注意：`bun run lint` 当前有预存 oxlint unused import 错误，不一定是本次改动导致。

## 8. 常见问题

### 后端启动时报 Valkey 或 Redis 连接失败

原因：没有覆盖 `.cargo/config.toml` 中的 `SERVICE_PERMISSION_CACHE_ENABLED=true`。

处理：

```bash
SERVICE_PERMISSION_CACHE_ENABLED=false cargo run -p web-server
```

PowerShell：

```powershell
$env:SERVICE_PERMISSION_CACHE_ENABLED="false"
cargo run -p web-server
```

### 后端启动时报 PostgreSQL 认证失败

检查默认连接是否可用：

```bash
psql "postgres://postgres:dev_only_pwd@localhost:5432/postgres"
```

如果本机 `postgres` 用户不是 `dev_only_pwd`，覆盖 `DEV_POSTGRES_URL`。

### 后端启动时报 `app_db` 或 `app_user` 不存在

debug 启动会自动创建它们。优先确认：

- 当前是 debug 启动，即 `cargo run -p web-server`。
- `DEV_POSTGRES_URL` 能连接到 `postgres` 数据库。
- 当前工作目录在项目根目录，或 Cargo workspace 内。

### 前端页面请求接口失败

确认两个服务都在运行：

```text
http://localhost:3000
http://localhost:8080
```

再确认前端通过 Vite 代理访问 `/api`，不要直接把前端 API 地址改成 Docker 网关地址。

### 端口被占用

默认端口：

- 后端：`8080`
- 前端：`3000`
- PostgreSQL：`5432`

前端 `vite.config.ts` 使用 `strictPort: true`，如果 3000 被占用会直接启动失败。先释放端口再启动。

## 9. 本地启动顺序速查

```text
1. 启动 PostgreSQL
2. 确认 postgres 用户密码或覆盖 DEV_POSTGRES_URL
3. 启动后端，并设置 SERVICE_PERMISSION_CACHE_ENABLED=false
4. 启动前端 bun run dev
5. 浏览器访问 http://localhost:3000
6. 使用 admin / admin 登录
```
