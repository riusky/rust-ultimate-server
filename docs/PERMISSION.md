# 权限系统指南

本文档详细介绍项目的 RBAC（基于角色的访问控制）权限系统。

## 目录

- [一、权限模型](#一权限模型)
- [二、权限定义](#二权限定义)
- [三、权限分配](#三权限分配)
- [四、权限检查](#四权限检查)
- [五、权限缓存](#五权限缓存)
- [六、最佳实践](#六最佳实践)

---

## 一、权限模型

### 1.1 核心概念

```
用户 (User)
    ↓
角色 (Role)         ← 用户可拥有多个角色
    ↓
权限 (Permission)   ← 角色可拥有多个权限
```

### 1.2 数据表结构

| 表名 | 说明 |
|------|------|
| `user` | 用户表 |
| `role` | 角色表 |
| `permission` | 权限表（由代码定义自动同步） |
| `user_role` | 用户-角色关联表 |
| `role_permission` | 角色-权限关联表 |

### 1.3 特殊角色

| 角色 | 说明 |
|------|------|
| `admin` | 管理员角色，自动拥有所有权限，无需单独分配 |
| `root` (user_id=0) | 系统根用户，绕过所有权限检查 |

---

## 二、权限定义

### 2.1 设计原则

> **重要**：权限必须由后端代码声明，不提供接口动态修改。

- 所有 REST/RPC 接口**必须**配置权限，否则启动时会报错
- 支持通过 `#[lib_macros::public]` 标记公开接口（无需权限）
- 权限在服务启动时自动同步到数据库

### 2.2 权限命名规范

```
{resource}:{action}

示例：
- user:create     # 创建用户
- user:read       # 查看用户
- user:update     # 更新用户
- user:delete     # 删除用户
- user:list       # 列表用户
- user_role:set   # 设置用户角色
```

### 2.3 使用 permission 宏定义权限

```rust
use lib_macros::permission;

/// RPC 接口权限定义
#[permission(
    key = "user:create",
    group = "User Management",
    display = "Create User",
    description = "Create new user accounts"  // 必填
)]
pub async fn create_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UserForCreate>,
) -> Result<DataRpcResult<User>> {
    // 权限检查由宏自动注入
    // ...
}

/// REST 接口权限定义（使用 CtxW 包装）
#[lib_macros::rest_permission(
    key = "agent:list",
    group = "Agent Management",
    display = "List Agents",
    description = "View agent list"
)]
pub async fn list_agents(
    ctx: CtxW,
    State(mm): State<ModelManager>,
    // ...
) -> Result<Json<RestListResult<Agent>>> {
    // ...
}
```

### 2.4 参数说明

| 参数 | 必填 | 说明 |
|------|------|------|
| `key` | 是 | 权限键，格式 `resource:action` |
| `group` | 否 | 权限分组，用于管理界面分类 |
| `display` | 否 | 显示名称 |
| `description` / `desc` | **是** | 权限描述，必须提供明确的业务含义 |

### 2.5 公开接口（免权限检查）

```rust
use lib_macros::public;

/// 登录接口无需权限
#[public]
pub async fn api_login_handler(
    // ...
) -> Result<...> {
    // ...
}
```

### 2.6 CRUD 权限批量注册

使用 `generate_common_rpc_fns!` 或 `generate_common_rest_fns!` 宏会自动注册 CRUD 权限：

```rust
generate_common_rpc_fns!(
    Bmc: AgentBmc,
    Entity: Agent,
    // ...
    Suffix: agent,
    ResourceDisplay: "Agent",
    ResourceGroup: "Agent Management",
    ResourceDescription: "agent entity"
);

// 自动注册以下权限：
// - agent:create
// - agent:read
// - agent:update
// - agent:delete
// - agent:list
```

---

## 三、权限分配

### 3.1 给用户分配角色

```rust
// RPC 接口
{
    "method": "set_roles_for_user",
    "params": {
        "user_id": 1000,
        "role_ids": [1, 2, 3]
    }
}
```

### 3.2 给角色分配权限

```rust
// RPC 接口
{
    "method": "set_permissions_for_role",
    "params": {
        "role_id": 1,
        "permission_ids": [10, 20, 30]
    }
}
```

### 3.3 权限同步机制

服务启动时会自动执行权限同步：

1. 扫描所有 `#[permission(...)]` 和 `#[rest_permission(...)]` 标记
2. 代码中存在 → 数据库不存在：**创建**权限
3. 代码中存在 → 数据库存在：**更新**权限
4. 代码中不存在 → 数据库存在：**删除**权限（级联删除角色关联）

---

## 四、权限检查

### 4.1 自动检查（推荐）

使用 `#[permission(...)]` 宏时，权限检查自动注入到函数开头：

```rust
#[permission(key = "user:create", ...)]
pub async fn create_user(ctx: Ctx, ...) {
    // 宏自动注入：ctx.require_permission("user:create")?;
    // ...
}
```

### 4.2 手动检查

在需要细粒度控制时：

```rust
pub async fn complex_operation(ctx: Ctx, ...) {
    // 检查单个权限
    ctx.require_permission("resource:action")?;
    
    // 检查是否拥有任一权限
    ctx.require_any_permission(&["perm:a", "perm:b"])?;
    
    // 检查是否拥有所有权限
    ctx.require_all_permissions(&["perm:a", "perm:b"])?;
    
    // 仅查询，不报错
    if ctx.has_permission("optional:perm") {
        // ...
    }
}
```

### 4.3 Ctx 权限方法

| 方法 | 说明 |
|------|------|
| `require_permission(key)` | 要求单个权限，无权限则返回错误 |
| `require_any_permission(keys)` | 要求任一权限 |
| `require_all_permissions(keys)` | 要求所有权限 |
| `has_permission(key)` | 检查是否拥有权限（不报错） |
| `has_any_permission(keys)` | 检查是否拥有任一权限 |
| `has_all_permissions(keys)` | 检查是否拥有所有权限 |

---

## 五、权限缓存

### 5.1 Valkey 缓存机制

当 `PERMISSION_CACHE_ENABLED=true` 时，用户权限会缓存到 Valkey：

- **缓存键**：`perm:user:{user_id}`
- **默认 TTL**：5 分钟
- **缓存内容**：用户的角色列表和权限列表

### 5.2 缓存失效触发

以下操作会自动清除相关用户的权限缓存：

| 操作 | 失效范围 |
|------|---------|
| `set_roles_for_user` | 该用户 |
| `set_permissions_for_role` | 拥有该角色的所有用户 |

### 5.3 手动清除缓存

```rust
use lib_web::utils::permission_cache::{
    invalidate_user_permissions_cache,
    invalidate_users_permissions_cache,
};

// 清除单个用户
invalidate_user_permissions_cache(&valkey_pool, user_id).await?;

// 批量清除
invalidate_users_permissions_cache(&valkey_pool, &user_ids).await?;
```

---

## 六、最佳实践

### 6.1 权限设计原则

1. **最小权限原则**：只授予完成任务所需的最少权限
2. **职责分离**：不同角色承担不同职责
3. **权限描述必填**：每个权限必须有明确的业务说明

### 6.2 推荐的角色设计

| 角色 | 说明 | 典型权限 |
|------|------|----------|
| `admin` | 管理员 | 全部权限（自动） |
| `operator` | 运维人员 | 系统配置、监控 |
| `user_manager` | 用户管理员 | 用户 CRUD、角色分配 |
| `viewer` | 只读用户 | 仅 `*:read`、`*:list` 权限 |

### 6.3 常见错误处理

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| 启动时报错：接口未配置权限 | 新接口未添加权限宏 | 添加 `#[permission(...)]` 或 `#[public]` |
| 403 Forbidden | 用户无权限 | 检查用户角色是否包含所需权限 |
| 权限不生效 | 缓存未清除 | 修改权限后等待 TTL 或手动清除缓存 |

### 6.4 调试技巧

```rust
// 查看用户当前权限
let permissions = ctx.permissions();
tracing::debug!("User permissions: {:?}", permissions);
```

---

## 相关文档

- [开发手册](./DEVELOPMENT.md) - 整体开发流程
- [过滤与分页指南](./FILTER_USAGE.md) - API 查询功能
