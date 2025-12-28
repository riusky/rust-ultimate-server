# 错误处理系统设计

本文档详细介绍项目的错误处理架构，包括后端错误分层、客户端错误码设计、前端国际化处理等。

## 目录

- [一、设计理念](#一设计理念)
- [二、后端错误架构](#二后端错误架构)
- [三、错误传递链路](#三错误传递链路)
- [四、客户端错误码](#四客户端错误码)
- [五、API 错误响应格式](#五api-错误响应格式)
- [六、前端错误处理](#六前端错误处理)
- [七、开发指南](#七开发指南)

---

## 一、设计理念

### 1.1 核心原则

1. **分层隔离**：内部错误与客户端错误分离，避免敏感信息泄露
2. **类型安全**：使用 Rust 枚举确保错误处理完整性
3. **自动传递**：通过 `#[from]` 派生宏实现错误自动转换
4. **统一格式**：RPC/REST 使用一致的错误响应结构
5. **多语言支持**：通过 `biz_code` 支持前端国际化错误提示

### 1.2 架构概览

```
┌─────────────────────────────────────────────────────────┐
│                    客户端错误响应                        │
│  { code, biz_code, message, detail }                    │
└─────────────────────────────────────────────────────────┘
                           ↑
┌─────────────────────────────────────────────────────────┐
│                   lib-web::Error                         │
│  顶层错误枚举，转换为 HTTP 状态码 + ClientError         │
└─────────────────────────────────────────────────────────┘
                           ↑
        ┌──────────────────┼──────────────────┐
        ↓                  ↓                  ↓
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│ lib-rpc-core  │  │ lib-rest-core │  │  lib-auth     │
│   ::Error     │  │   ::Error     │  │   ::Error     │
└───────────────┘  └───────────────┘  └───────────────┘
        ↓                  ↓
┌─────────────────────────────────────────────────────────┐
│                   lib-core::Error                        │
│  业务核心错误（Model、Ctx）                              │
└─────────────────────────────────────────────────────────┘
```

---

## 二、后端错误架构

### 2.1 错误模块分层

| 模块 | 路径 | 职责 |
|------|------|------|
| `lib-core::model::Error` | `lib-core/src/model/error.rs` | 数据模型错误 |
| `lib-core::ctx::Error` | `lib-core/src/ctx/error.rs` | 上下文/权限错误 |
| `lib-auth::pwd::Error` | `lib-auth/src/pwd/error.rs` | 密码处理错误 |
| `lib-auth::token::Error` | `lib-auth/src/token/error.rs` | Token 处理错误 |
| `lib-rpc-core::Error` | `lib-rpc-core/src/error.rs` | RPC 层错误 |
| `lib-rest-core::Error` | `lib-rest-core/src/error.rs` | REST 层错误 |
| `lib-web::Error` | `lib-web/src/error.rs` | 顶层统一错误 |

### 2.2 核心错误定义

#### lib-core::model::Error

```rust
#[derive(Debug, Serialize, From)]
pub enum Error {
    // 实体操作
    EntityNotFound { entity: &'static str, id: i64 },
    ListLimitOverMax { max: i64, actual: i64 },
    CountFail,
    
    // 数据库约束
    UserAlreadyExists { username: String },
    UniqueViolation { table: String, constraint: String },
    
    // 模块错误
    #[from]
    Pwd(pwd::Error),
    #[from]
    Dbx(dbx::Error),
    
    // 外部错误
    #[from]
    SeaQuery(sea_query::error::Error),
    #[from]
    ModqlIntoSea(modql::filter::IntoSeaError),
}
```

#### lib-core::ctx::Error

```rust
#[derive(Debug, Serialize, Clone)]
pub enum Error {
    CtxCannotNewRootCtx,
    
    // 权限错误
    PermissionDenied { user_id: i64, permission: String },
    PermissionAnyDenied { user_id: i64, permissions: Vec<String> },
    PermissionsNotLoaded,
}
```

#### lib-rpc-core::Error

```rust
#[derive(Debug, From, Serialize, RpcHandlerError)]
pub enum Error {
    #[from]
    Model(lib_core::model::Error),
    
    #[from]
    Ctx(lib_core::ctx::Error),
    
    #[from]
    SerdeJson(serde_json::Error),
}
```

> **注意**：`RpcHandlerError` 派生宏会自动实现 `rpc_router::IntoRpcHandlerError` trait，使错误能在 RPC 框架中正确传递。

#### lib-rest-core::Error

```rust
#[derive(Debug, From, Serialize, IntoResponseExt)]
pub enum Error {
    #[from]
    Model(lib_core::model::Error),
    
    #[from]
    Ctx(lib_core::ctx::Error),
    
    #[from]
    SerdeJson(serde_json::Error),
}
```

> **注意**：`IntoResponseExt` 是自定义派生宏，将错误转换为 Axum Response 并存入 extensions。

#### lib-web::Error（顶层错误）

```rust
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- 登录相关
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd { user_id: i64 },
    LoginFailPwdNotMatching { user_id: i64 },

    // -- 用户管理
    RegisterFailUsernameEmpty,
    RegisterFailPwdTooShort,
    UserHasNoPwd,
    PwdNotMatching,
    ResetPwdFailUserNotFound,
    NotAuthorized,

    // -- 上下文提取
    #[from]
    CtxExt(middleware::mw_auth::CtxExtError),
    ReqStampNotInReqExt,

    // -- 模块错误（自动转换）
    #[from]
    Model(model::Error),
    #[from]
    Pwd(pwd::Error),
    #[from]
    Token(token::Error),
    #[from]
    Ctx(lib_core::ctx::Error),
    #[from]
    Rpc(lib_rpc_core::Error),
    #[from]
    Rest(lib_rest_core::Error),

    // -- RPC Router 错误
    #[from]
    RpcRequestParsing(rpc_router::RpcRequestParsingError),
    RpcLibRpc(lib_rpc_core::Error),
    RpcHandlerErrorUnhandled(&'static str),
    RpcRouter { id: Box<Value>, method: String, error: rpc_router::Error },

    // -- 外部模块
    #[from]
    SerdeJson(serde_json::Error),
}
```

---

## 三、错误传递链路

### 3.1 自动转换机制

使用 `derive_more::From` 宏实现错误自动转换：

```rust
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Model(lib_core::model::Error),  // model::Error 自动转为 Error::Model
    
    #[from]
    Ctx(lib_core::ctx::Error),      // ctx::Error 自动转为 Error::Ctx
}
```

### 3.2 RPC 错误传递

RPC 处理函数返回的错误经过以下链路：

```
RPC Handler
    ↓ 返回 lib_rpc_core::Error
rpc_router
    ↓ 包装为 rpc_router::Error::Handler(RpcHandlerError)
lib-web::Error
    ↓ From<rpc_router::CallError> 解包
mw_res_map
    ↓ client_status_and_error() 映射
HTTP Response
```

**解包实现**：

```rust
impl From<rpc_router::CallError> for Error {
    fn from(call_error: rpc_router::CallError) -> Self {
        let rpc_router::CallError { id, method, error } = call_error;
        match error {
            rpc_router::Error::Handler(mut rpc_handler_error) => {
                // 尝试提取已知错误类型
                if let Some(lib_rpc_error) = rpc_handler_error.remove::<lib_rpc_core::Error>() {
                    Error::RpcLibRpc(lib_rpc_error)
                } else {
                    Error::RpcHandlerErrorUnhandled(rpc_handler_error.type_name())
                }
            }
            error => Error::RpcRouter { id: Box::new(id.to_value()), method, error },
        }
    }
}
```

### 3.3 REST 错误传递

REST 处理函数返回的错误经过以下链路：

```
REST Handler
    ↓ 返回 lib_rest_core::Error
mw_rest_error
    ↓ 包装为 lib_web::Error::Rest
mw_res_map
    ↓ client_status_and_error() 映射
HTTP Response
```

---

## 四、客户端错误码

### 4.1 错误码格式

格式：`CATEGORY#NNN`

- **CATEGORY**：错误类别（AUTH、USER、PERM、DATA、RPC、SYS）
- **NNN**：3 位数字序号（001-099）

### 4.2 错误码映射表

| 错误码 | 类型名 | 说明 |
|--------|--------|------|
| **AUTH#001** | `LOGIN_FAIL` | 登录失败（用户名或密码错误） |
| **AUTH#002** | `NO_AUTH` | 未登录/Token 无效 |
| **USER#001** | `REGISTER_FAIL_USERNAME_EMPTY` | 用户名不能为空 |
| **USER#002** | `REGISTER_FAIL_PWD_TOO_SHORT` | 密码太短 |
| **USER#003** | `PWD_VALIDATION_FAIL` | 密码验证失败 |
| **USER#004** | `USER_NOT_FOUND` | 用户不存在 |
| **USER#005** | `NOT_AUTHORIZED` | 无权执行此操作 |
| **PERM#001** | `PERMISSION_DENIED` | 权限不足（单个权限） |
| **PERM#002** | `PERMISSION_ANY_DENIED` | 权限不足（多个权限均无） |
| **DATA#001** | `ENTITY_NOT_FOUND` | 数据不存在 |
| **RPC#001** | `RPC_REQUEST_INVALID` | RPC 请求格式错误 |
| **RPC#002** | `RPC_REQUEST_METHOD_UNKNOWN` | RPC 方法不存在 |
| **RPC#003** | `RPC_PARAMS_INVALID` | RPC 参数无效 |
| **SYS#001** | `SERVICE_ERROR` | 系统繁忙 |

### 4.3 ClientError 定义

```rust
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
    
    // Permission
    PERMISSION_DENIED { permission: String },
    PERMISSION_ANY_DENIED { permissions: Vec<String> },
    
    // User Management
    REGISTER_FAIL_USERNAME_EMPTY,
    REGISTER_FAIL_PWD_TOO_SHORT,
    PWD_VALIDATION_FAIL,
    USER_NOT_FOUND,
    NOT_AUTHORIZED,
    
    // RPC
    RPC_REQUEST_INVALID(String),
    RPC_REQUEST_METHOD_UNKNOWN(String),
    RPC_PARAMS_INVALID(String),
    
    SERVICE_ERROR,
}

impl ClientError {
    pub fn biz_code(&self) -> &'static str {
        use ClientError::*;
        match self {
            LOGIN_FAIL => "AUTH#001",
            NO_AUTH => "AUTH#002",
            // ... 完整映射见源码
        }
    }
}
```

### 4.4 错误映射逻辑

`client_status_and_error()` 方法将内部错误映射为 HTTP 状态码和 ClientError：

```rust
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // 登录错误 → 403 Forbidden
            LoginFailUsernameNotFound | LoginFailUserHasNoPwd { .. } | LoginFailPwdNotMatching { .. } 
                => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            
            // 权限错误 → 403 Forbidden
            RpcLibRpc(lib_rpc_core::Error::Ctx(lib_core::ctx::Error::PermissionDenied { permission, .. })) 
                => (StatusCode::FORBIDDEN, ClientError::PERMISSION_DENIED { permission: permission.clone() }),
            
            // 实体不存在 → 404 Not Found
            Model(model::Error::EntityNotFound { entity, id }) 
                => (StatusCode::NOT_FOUND, ClientError::ENTITY_NOT_FOUND { entity, id: *id }),
            
            // 其他 → 500 Internal Server Error
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
        }
    }
}
```

---

## 五、API 错误响应格式

### 5.1 JSON-RPC 错误响应

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "error": {
        "code": 403,
        "biz_code": "PERM#001",
        "message": "PERMISSION_DENIED",
        "data": {
            "req_uuid": "550e8400-e29b-41d4-a716-446655440000",
            "detail": {
                "permission": "user_info:read"
            }
        }
    }
}
```

### 5.2 REST 错误响应

```json
{
    "success": false,
    "error": {
        "code": 404,
        "biz_code": "DATA#001",
        "message": "ENTITY_NOT_FOUND",
        "detail": {
            "entity": "user_info",
            "id": 123
        },
        "resource": "user-infos",
        "action": "get"
    },
    "meta": {
        "req_uuid": "550e8400-e29b-41d4-a716-446655440000"
    }
}
```

### 5.3 响应字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | `number` | HTTP 状态码 |
| `biz_code` | `string` | 业务错误码，用于前端 i18n |
| `message` | `string` | 错误类型名（枚举变体名） |
| `detail` | `object?` | 错误详情（如实体ID、权限名等） |
| `req_uuid` | `string` | 请求唯一标识，用于日志追踪 |

---

## 六、前端错误处理

### 6.1 错误响应类型

```typescript
// services/api/api-client.ts
export interface ApiErrorResponse {
    success: false
    error: {
        code: number
        biz_code: string
        message: string
        detail?: unknown
        resource?: string
        action?: string
    }
    meta?: {
        req_uuid: string
    }
}
```

### 6.2 国际化错误消息

在各语言文件中定义 `bizErrors` 命名空间：

```yaml
# plugins/i18n/locales/zh-CN.yaml
bizErrors:
    # Auth errors (AUTH#001-099)
    AUTH#001: "用户名或密码错误"
    AUTH#002: "请先登录"
    # User errors (USER#001-099)
    USER#001: "用户名不能为空"
    USER#002: "密码太短"
    USER#003: "密码验证失败"
    USER#004: "用户不存在"
    USER#005: "无权执行此操作"
    # Permission errors (PERM#001-099)
    PERM#001: "权限不足"
    PERM#002: "缺少必要权限"
    # Data errors (DATA#001-099)
    DATA#001: "数据不存在"
    # RPC errors (RPC#001-099)
    RPC#001: "请求格式错误"
    RPC#002: "接口不存在"
    RPC#003: "参数无效"
    # System errors (SYS#001-099)
    SYS#001: "系统繁忙，请稍后重试"
    # Fallback
    UNKNOWN: "发生未知错误"
```

### 6.3 错误提示组合函数

```typescript
// composables/use-biz-error.ts
export function useBizError() {
    const { t, te } = useI18n()

    function getBizErrorMessage(bizCode: string | undefined | null): string {
        if (!bizCode) {
            return t('bizErrors.UNKNOWN')
        }

        const key = `bizErrors.${bizCode}`
        if (te(key)) {
            return t(key)
        }

        return t('bizErrors.UNKNOWN')
    }

    return { getBizErrorMessage }
}
```

### 6.4 统一错误处理

```typescript
// services/api/api-client.ts
apiClient.interceptors.response.use(
    (response) => response,
    (error: AxiosError<ApiErrorResponse>) => {
        if (error.response?.data?.error) {
            const { biz_code, message } = error.response.data.error
            
            // 使用 biz_code 获取本地化消息
            const errorMessage = getBizErrorMessage(biz_code)
            toast.error(errorMessage)
            
            // 特殊处理：未登录跳转
            if (biz_code === 'AUTH#002') {
                router.push('/auth/login')
            }
        } else {
            // 网络错误
            toast.error(t('errorMessages.serverError'))
        }
        
        return Promise.reject(error)
    }
)
```

---

## 七、开发指南

### 7.1 添加新错误类型

**Step 1**: 在对应模块添加错误变体

```rust
// lib-core/src/model/error.rs
pub enum Error {
    // ... 现有变体
    
    // 新增：邮箱已存在
    EmailAlreadyExists { email: String },
}
```

**Step 2**: 在 lib-web 添加 ClientError 变体

```rust
// lib-web/src/error.rs
pub enum ClientError {
    // ... 现有变体
    
    EMAIL_ALREADY_EXISTS,
}

impl ClientError {
    pub fn biz_code(&self) -> &'static str {
        match self {
            // ... 现有映射
            EMAIL_ALREADY_EXISTS => "USER#006",
        }
    }
}
```

**Step 3**: 添加错误映射

```rust
// lib-web/src/error.rs
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // ... 现有映射
            
            Model(model::Error::EmailAlreadyExists { .. }) => {
                (StatusCode::CONFLICT, ClientError::EMAIL_ALREADY_EXISTS)
            }
        }
    }
}
```

**Step 4**: 添加前端国际化

```yaml
# 所有语言文件
bizErrors:
    USER#006: "该邮箱已被注册"  # zh-CN
    # USER#006: "Email already exists"  # en
```

### 7.2 错误日志追踪

每个错误响应包含 `req_uuid`，用于日志关联：

```
# 后端日志
2024-01-15T10:30:00Z INFO req_uuid=550e8400-e29b-41d4-a716-446655440000 error=PermissionDenied { permission: "admin:write" }
```

前端可在控制台或错误上报时记录 `req_uuid`，便于问题排查。

### 7.3 敏感信息处理

**原则**：内部错误信息不应暴露给客户端

- `LoginFailUserHasNoPwd { user_id }` → 客户端只看到 `LOGIN_FAIL`
- `Sqlx(sqlx::Error)` → 客户端只看到 `SERVICE_ERROR`
- 具体的堆栈信息仅记录在服务端日志

### 7.4 错误码分配规则

| 类别 | 范围 | 说明 |
|------|------|------|
| AUTH | 001-099 | 认证相关（登录、Token） |
| USER | 001-099 | 用户管理（注册、密码） |
| PERM | 001-099 | 权限相关 |
| DATA | 001-099 | 数据操作（CRUD） |
| RPC | 001-099 | RPC 协议层 |
| SYS | 001-099 | 系统级错误 |

> **新增错误码时**：在对应类别内递增编号，并更新本文档。

---

## 相关文档

- [开发手册](./DEVELOPMENT.md) - 整体开发流程
- [权限系统指南](./PERMISSION.md) - 权限错误详解
