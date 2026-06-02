# AGENTS.md

## 项目定位

CMX Server 是一个 Rust + Vue 3 全栈 Web 应用框架，后端使用 Axum、PostgreSQL、Valkey/Redis、Pingora，前端使用 Vue 3、TypeScript、Vite、shadcn-vue。核心能力包括用户认证、RBAC 权限管理、REST API、JSON-RPC API、自动 TypeScript 类型导出。

完整背景参考：

- `README.md`：项目介绍、Docker 模式、常用命令、文档索引
- `docs/DEVELOPMENT.md`：开发流程、架构、命名规范、响应格式
- `docs/PERMISSION.md`：RBAC 权限定义、同步、缓存和检查
- `docs/AUTH_DESIGN.md`：自定义 Token、密码哈希方案、认证配置
- `docs/ERROR_DESIGN.md`：错误分层、客户端错误码、前端 i18n
- `docs/FILTER_USAGE.md`：REST/RPC 过滤、排序、分页语法
- `docs/TS_EXPORT.md`：Rust model 到前端 TypeScript 类型导出

## 项目结构

```text
crates/
  libs/
    lib-core          # 业务模型、BMC、Ctx、权限定义、数据库初始化
    lib-auth          # Token、密码哈希、认证配置
    lib-web           # Axum 中间件、路由辅助、响应映射
    lib-rpc-core      # JSON-RPC 协议和通用 RPC 宏
    lib-rest-core     # REST API 协议和通用 REST 宏
    lib-macros        # 权限宏、错误响应宏等过程宏
    lib-valkey-core   # Valkey/Redis 连接池和缓存操作
    lib-utils         # 通用工具
  services/
    web-server        # Axum Web 服务入口，监听 8080
    pingora-gateway   # Pingora 反向代理网关
  tools/
    gen-key           # 生成认证密钥
cmx-vue-ultimate-starter/ # Vue 3 前端
sql/dev_initial/          # 开发数据库初始化 SQL
vendor/                   # vendored modql 与 rpc-router
```

Cargo workspace 把 `vendor/modql` 和 `vendor/rpc-router` 也纳入成员。注意 `vendor/rpc-router` 使用 Rust edition 2024，要求 Rust 1.85.0+。

## 本地服务

| 服务 | 端口 | 命令 | 必需 |
| --- | --- | --- | --- |
| PostgreSQL 17 | 5432 | `sudo pg_ctlcluster 17 main start` | 是 |
| Rust web-server | 8080 | `SERVICE_PERMISSION_CACHE_ENABLED=false cargo run -p web-server` | 是 |
| Vue 3 frontend | 3000 | `cd cmx-vue-ultimate-starter && bun run dev` | 是 |
| Valkey/Redis | 6379 | 本地默认未安装 | 否 |

本地运行后端或测试时，必须显式设置 `SERVICE_PERMISSION_CACHE_ENABLED=false`。`.cargo/config.toml` 默认把该值设为 `true`，如果本地没有 Valkey/Redis，会导致权限缓存初始化失败。

Docker 模式会启动 PostgreSQL、Valkey、web-server、frontend 和 Pingora gateway。访问入口是 `http://localhost`，其中 `/api/*` 转发到后端，其余路径转发到前端。

## 开发规则

1. 后端新接口必须标注权限。
   - 受保护接口使用 `#[permission(...)]` 或 `#[rest_permission(...)]`。
   - 公开接口使用 `#[public]`。
   - 权限会在服务启动时通过 `PermissionBmc::sync_from_registry` 同步到数据库。

2. 新增业务实体按固定流程走。
   - 设计表和迁移 SQL。
   - 在 `lib-core/src/model/` 定义 model、filter、create/update 类型。
   - 用 BMC 宏生成 CRUD。
   - 用 RPC/REST 宏生成接口。
   - 在 `web-server` 注册路由。
   - 如前端需要类型，运行 TypeScript 导出流程。

3. 数据库业务表应包含审计字段。
   - `id`
   - `cid`
   - `ctime`
   - `mid`
   - `mtime`

4. REST 和 RPC 的分页排序语法不同。
   - REST：`page_size`、`page_number`，降序前缀是 `-`。
   - RPC：`limit`、`offset`，降序前缀是 `!`。

5. 错误处理要走现有分层。
   - 内部错误不要直接暴露给客户端。
   - 客户端错误通过 `biz_code` 映射到前端国际化文案。
   - 新增错误码时同步更新后端映射、前端 i18n 和文档。

6. 认证系统不是标准 JWT。
   - Token 是 `{ident_b64u}.{exp_b64u}.{sign_b64u}` 格式。
   - 签名使用 Blake3、用户 `token_salt` 和 `SERVICE_TOKEN_KEY`。
   - 密码默认使用 Argon2id，旧 HMAC-SHA512 scheme 仅用于兼容。

## 常用命令

```bash
# 后端本地启动
SERVICE_PERMISSION_CACHE_ENABLED=false cargo run -p web-server

# 前端本地启动
cd cmx-vue-ultimate-starter
bun install
bun run dev

# 生成认证密钥
cargo run -p gen-key

# 推荐测试范围，避免构建不相关 transitive dependencies
SERVICE_PERMISSION_CACHE_ENABLED=false cargo test -p web-server -p lib-core -p lib-auth -p lib-web -p lib-rpc-core -p lib-rest-core -p lib-macros -p lib-valkey-core -p lib-utils -p gen-key

# Rust model 变更后导出 TypeScript 类型
bash shell/gen-ts-types.sh

# Docker 启动完整栈
docker compose up -d --build
```

## 已知坑点

- 本地默认没有 Valkey/Redis；后端和测试要覆盖 `SERVICE_PERMISSION_CACHE_ENABLED=false`。
- `web-server` debug 启动会自动初始化开发数据库，执行 `sql/dev_initial/00-recreate-db.sql` 和 Refinery migrations。
- 默认管理员用户是 `admin`，密码是 `admin`。
- PostgreSQL 本地连接使用 `localhost:5432`。
- `.cargo/config.toml` 里的密钥和密码只用于本地开发，不可照搬到生产。
- 修改 Rust model 后如果没有重新导出 TS 类型，前端类型会过期。
- `vendor/modql` 在新 Rust 版本下可能有 clippy 警告；clippy 时优先指定 workspace crate，不要无差别扫 vendor。
- `cmx-vue-ultimate-starter` 的 `bun run lint` 有已知预存 oxlint unused import 错误，不一定是当前改动导致。
- native C++ 依赖构建可能需要 `/usr/lib/x86_64-linux-gnu/libstdc++.so -> libstdc++.so.6` symlink。
- Bun 位于 `$HOME/.bun/bin/bun`，确保 `$HOME/.bun/bin` 在 `PATH`。

## 代理工作准则

- 先读当前文件和相关文档，再改代码；不要只依赖历史上下文。
- 保持改动贴近现有模块边界，避免顺手重构无关文件。
- 不要回滚用户已有改动。
- 涉及 Rust model、权限、错误码、API 响应或前端生成类型时，同步检查对应文档和生成物。
- 外部网络操作默认只读；推送、发布、合并、修改远端资源前需要用户明确同意。
