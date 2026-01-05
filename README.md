# CMX Server

基于 Rust + Vue 的全栈 Web 应用框架，提供完整的用户认证、RBAC 权限管理、API 开发等功能。

## 特性

- **后端**: Rust + Axum + PostgreSQL + Valkey
- **前端**: Vue 3 + TypeScript + Vite + shadcn-vue
- **认证**: JWT Token + Cookie
- **权限**: RBAC 模型，代码声明式权限管理
- **API**: REST + JSON-RPC 双协议支持
- **类型安全**: 自动生成 TypeScript 类型定义

## 快速开始

### 依赖服务

- PostgreSQL 15+
- Valkey/Redis（可选，用于权限缓存）

### 本地开发

```bash
# 安装 cargo-watch
cargo install cargo-watch

# 终端1 - 后端服务（热重载）
cargo watch -q -c -w crates/ -x "run -p web-server"

# 终端2 - 前端服务
cd cmx-vue-ultimate-starter
bun install
bun run dev
```

### Docker 模式

```bash


# 0. 首次部署 / 新机器
docker pull rust:1.92.0-slim-bullseye
docker pull debian:bullseye-slim

# 1. 启动所有服务
docker compose up -d

# 2. 重新构建（利用缓存，只改代码时约 20 秒）
docker compose build web-server

# 3. 重启服务
docker compose up -d web-server
```

## 项目结构

```
cmx-server/
├── crates/
│   ├── libs/                # 核心库
│   │   ├── lib-core/       # 业务核心（Model、BMC）
│   │   ├── lib-auth/       # 认证模块
│   │   ├── lib-web/        # Web 层（中间件、处理器）
│   │   ├── lib-rpc-core/   # RPC 核心
│   │   ├── lib-rest-core/  # REST 核心
│   │   ├── lib-macros/     # 过程宏
│   │   └── lib-valkey-core/# Valkey 客户端
│   └── services/
│       └── web-server/     # Web 服务入口
├── cmx-vue-ultimate-starter/  # 前端项目
├── sql/dev_initial/           # 数据库初始化脚本
└── docs/                      # 开发文档
```

## 核心特性

| 特性 | 说明 |
|------|------|
| 声明式宏 | `generate_common_bmc_fns!`、`generate_common_rpc_fns!` 减少样板代码 |
| 权限宏 | `#[permission(...)]` 自动注册权限并检查 |
| 数据库事务 | `ModelManager` 支持按需事务控制 |
| TypeScript 支持 | `with-ts` feature 自动生成前端类型 |
| 权限缓存 | Valkey 缓存用户权限，减少数据库查询 |

## 常用命令

```bash
# 生成密钥
cargo run -p gen-key

# 运行测试
cargo test -- --nocapture

# 生成 TypeScript 类型
cargo test -p lib-core --features with-ts export_ts_types
cd cmx-vue-ultimate-starter && bun run gen:types
```

## 文档索引

| 文档 | 说明 |
|------|------|
| [开发手册](./docs/DEVELOPMENT.md) | 完整开发流程、架构设计、代码规范 |
| [权限系统指南](./docs/PERMISSION.md) | RBAC 权限模型、定义与使用 |
| [认证系统设计](./docs/AUTH_DESIGN.md) | Token 机制、密码哈希多模式切换 |
| [错误处理系统设计](./docs/ERROR_DESIGN.md) | 错误分层、错误码与国际化处理 |
| [过滤与分页指南](./docs/FILTER_USAGE.md) | REST/RPC 过滤、排序、分页功能 |
| [TypeScript 类型导出](./docs/TS_EXPORT.md) | 后端类型自动生成到前端 |

## API 响应格式

### RPC 响应

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "data": { ... }
    }
}
```

### REST 响应

```json
{
    "success": true,
    "data": { ... }
}
```

## 许可证

MIT
