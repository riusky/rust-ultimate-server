# 开发手册

本文档是项目的核心开发指南，涵盖开发流程、架构设计、代码规范等内容。

## 目录

- [一、项目架构](#一项目架构)
- [二、开发环境](#二开发环境)
- [三、开发流程](#三开发流程)
- [四、代码规范](#四代码规范)
- [五、常见问题](#五常见问题)

---

## 一、项目架构

### 1.1 目录结构

```
cmx-server/
├── crates/
│   ├── libs/                    # 核心库
│   │   ├── lib-core/           # 业务核心（Model、BMC）
│   │   ├── lib-auth/           # 认证模块
│   │   ├── lib-web/            # Web 层（中间件、处理器）
│   │   ├── lib-rpc-core/       # RPC 核心
│   │   ├── lib-rest-core/      # REST 核心
│   │   ├── lib-macros/         # 过程宏
│   │   └── lib-valkey-core/    # Valkey/Redis 客户端
│   ├── services/
│   │   └── web-server/         # Web 服务入口
│   └── tools/
│       └── gen-key/            # 密钥生成工具
├── cmx-vue-ultimate-starter/   # 前端项目
├── sql/dev_initial/            # 数据库初始化脚本
├── docs/                       # 开发文档
└── vendor/                     # 本地依赖
```

### 1.2 核心模块职责

| 模块 | 职责 |
|------|------|
| `lib-core` | 数据模型、BMC（Business Model Controller）、权限定义 |
| `lib-auth` | Token 生成与验证、密码加密 |
| `lib-web` | HTTP 中间件、路由处理、错误响应 |
| `lib-rpc-core` | JSON-RPC 协议支持、通用 RPC 宏 |
| `lib-rest-core` | REST API 支持、通用 REST 宏 |
| `lib-macros` | 权限宏、BMC 生成宏等过程宏 |
| `lib-valkey-core` | Redis 兼容的缓存操作 |

### 1.3 请求处理流程

```
HTTP Request
    ↓
mw_req_stamp_resolver    # 请求时间戳
    ↓
CookieManager            # Cookie 管理
    ↓
mw_ctx_resolver          # 用户上下文解析（从 Token）
    ↓
mw_permission_resolver   # 权限加载（可选 Valkey 缓存）
    ↓
mw_ctx_require           # 验证登录状态
    ↓
Route Handler            # 业务处理（RPC/REST）
    ↓
mw_response_map          # 响应格式化
    ↓
HTTP Response
```

---

## 二、开发环境

### 2.1 依赖服务

- **PostgreSQL**: 主数据库
- **Valkey/Redis**: 权限缓存（可选）

### 2.2 本地开发

```bash
# 安装 cargo-watch（热重载）
cargo install cargo-watch

# 终端1 - 启动后端（监听变化）
cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -x "run -p web-server"

# 终端2 - 启动前端
cd cmx-vue-ultimate-starter
bun install
bun run dev

# 终端3（可选）- 运行测试脚本
cargo watch -q -c -w crates/services/web-server/examples/ -x "run -p web-server --example quick_dev"
```

### 2.3 Docker 模式

```bash
docker compose up -d
```

### 2.4 工具命令

```bash
# 生成密钥（用于环境变量配置）
cargo run -p gen-key
```

---

## 三、开发流程

### 3.1 新功能开发步骤

```
1. 数据库表设计
    ↓
2. 创建 SQL 迁移脚本（sql/dev_initial/）
    ↓
3. 定义 Model 结构体（lib-core/src/model/）
    ↓
4. 实现 BMC（使用 generate_common_bmc_fns! 宏）
    ↓
5. 创建 RPC/REST 接口（使用 generate_common_rpc_fns!/generate_common_rest_fns! 宏）
    ↓
6. 注册路由（web-server/src/web/rpcs/mod.rs）
    ↓
7. 生成 TypeScript 类型（cargo test -p lib-core --features with-ts export_ts_types）
    ↓
8. 前端 API 封装与页面开发
```

### 3.2 实体定义示例

```rust
// crates/libs/lib-core/src/model/your_module.rs

#[cfg(feature = "with-ts")]
use ts_rs::TS;

/// 实体结构体
#[derive(Debug, Clone, Serialize, Deserialize, Fields)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/types/generated/your_module/"))]
pub struct YourEntity {
    pub id: i64,
    pub name: String,
    
    // 时间字段需要标注类型映射
    #[cfg_attr(feature = "with-ts", ts(type = "string"))]
    pub ctime: OffsetDateTime,
    
    #[cfg_attr(feature = "with-ts", ts(type = "string"))]
    pub mtime: OffsetDateTime,
}

/// BMC 实现
pub struct YourEntityBmc;

impl DbBmc for YourEntityBmc {
    const TABLE: &'static str = "your_entity";
}

// 使用宏生成 CRUD 方法
generate_common_bmc_fns!(
    Bmc: YourEntityBmc,
    Entity: YourEntity,
    ForCreate: YourEntityForCreate,
    ForUpdate: YourEntityForUpdate,
);
```

### 3.3 RPC 接口示例

```rust
// crates/services/web-server/src/web/rpcs/your_entity_rpc.rs

use lib_core::generate_rpc_routes;
use lib_rpc_core::prelude::*;

pub fn rpc_router_builder() -> RouterBuilder {
    generate_rpc_routes!(
        list_your_entities,
        get_your_entity,
        create_your_entity,
        update_your_entity,
        delete_your_entity,
    )
}

// 使用宏生成 CRUD 接口
generate_common_rpc_fns!(
    Bmc: YourEntityBmc,
    Entity: YourEntity,
    ForCreate: YourEntityForCreate,
    ForUpdate: YourEntityForUpdate,
    Filter: YourEntityFilter,
    Suffix: your_entity,
    ResourceDisplay: "Your Entity",
    ResourceGroup: "Your Entity Management",
    ResourceDescription: "your entity resource"
);
```

---

## 四、代码规范

### 4.1 数据库表规范

所有业务表必须包含审计字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `BIGINT` | 主键，自增 |
| `cid` | `BIGINT` | 创建者 ID |
| `ctime` | `TIMESTAMPTZ` | 创建时间 |
| `mid` | `BIGINT` | 最后修改者 ID |
| `mtime` | `TIMESTAMPTZ` | 最后修改时间 |

### 4.2 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 表名 | 小写蛇形 | `user_info` |
| 实体 | 大驼峰 | `UserInfo` |
| BMC | 实体名 + Bmc | `UserInfoBmc` |
| RPC 方法 | 蛇形，动词+名词 | `list_user_infos`, `get_user_info` |
| REST 路径 | 小写复数 | `/api/rest/user-infos` |

### 4.3 分页参数

| API 类型 | 参数名 | 说明 |
|----------|--------|------|
| REST | `page_size`, `page_number` | 从 1 开始 |
| RPC | `limit`, `offset` | 从 0 开始 |

### 4.4 响应格式

**RPC 响应**：
```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "data": { ... }
    }
}
```

**REST 响应**：
```json
{
    "success": true,
    "data": { ... }
}

// 分页响应
{
    "success": true,
    "data": [...],
    "page_info": {
        "total": 100,
        "page_size": 10,
        "page_number": 1,
        "total_pages": 10,
        "has_more": true
    }
}
```

---

## 五、常见问题

### 5.1 本地开发数据库连接失败

本地运行时，需要将数据库主机从 Docker 服务名改为 `localhost`：

```env
# .env
DATABASE_URL=postgres://user:password@localhost:5432/dbname
```

### 5.2 权限相关错误

1. **接口返回 403**：检查用户是否拥有对应权限，或者接口是否使用了 `#[lib_macros::public]` 标记
2. **启动时权限校验失败**：确保所有接口都使用了 `#[lib_macros::permission(...)]` 或 `#[lib_macros::public]` 标记

### 5.3 TypeScript 类型未生成

```bash
# 确保启用 with-ts feature
cargo test -p lib-core --features with-ts export_ts_types

# 格式化并生成 index.ts
cd cmx-vue-ultimate-starter
bun run gen:types
```

### 5.4 前端显示不正确

1. 检查 API 返回的 `result.data` 是否正确解包
2. 确认使用的是 `RpcDataResult<T>` 类型包装

---

## 相关文档

- [权限系统指南](./PERMISSION.md) - 权限定义、分配与使用
- [错误处理系统设计](./ERROR_DESIGN.md) - 错误分层、错误码与国际化
- [过滤与分页指南](./FILTER_USAGE.md) - REST/RPC 过滤、排序、分页
- [TypeScript 类型导出指南](./TS_EXPORT.md) - 后端类型到前端的自动生成
