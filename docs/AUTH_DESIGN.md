# 认证系统设计

本文档详细介绍 `lib-auth` 模块的设计，包括 Token 机制、密码哈希多模式切换、配置管理等。

## 目录

- [一、设计概览](#一设计概览)
- [二、Token 机制](#二token-机制)
- [三、密码哈希多模式设计](#三密码哈希多模式设计)
- [四、配置管理](#四配置管理)
- [五、扩展指南](#五扩展指南)

---

## 一、设计概览

### 1.1 模块结构

```
lib-auth/
├── src/
│   ├── config.rs           # 认证配置（从环境变量加载）
│   ├── lib.rs              # 模块入口
│   ├── pwd/                # 密码哈希模块
│   │   ├── mod.rs          # 公共接口
│   │   ├── error.rs        # 密码错误
│   │   └── scheme/         # 多模式实现
│   │       ├── mod.rs      # Scheme trait 与调度
│   │       ├── scheme_01.rs  # HMAC-SHA512 方案
│   │       └── scheme_02.rs  # Argon2id 方案（当前默认）
│   └── token/              # Token 模块
│       ├── mod.rs          # Token 生成与验证
│       └── error.rs        # Token 错误
└── Cargo.toml
```

### 1.2 核心特性

| 特性 | 说明 |
|------|------|
| 自定义 Token | 基于 Blake3 签名的轻量级 Token，非标准 JWT |
| 密码多模式 | 支持多种哈希算法，可平滑升级 |
| 异步友好 | 密码哈希通过 `spawn_blocking` 避免阻塞异步运行时 |
| 配置化 | 密钥、有效期等通过环境变量配置 |

---

## 二、Token 机制

### 2.1 Token 格式

Token 采用自定义格式（非标准 JWT），结构如下：

```
{ident_b64u}.{exp_b64u}.{sign_b64u}
```

| 部分 | 说明 | 示例 |
|------|------|------|
| `ident_b64u` | 用户标识（Base64URL 编码） | `dXNlcl9vbmU` (user_one) |
| `exp_b64u` | 过期时间 RFC3339（Base64URL 编码） | `MjAyNC0wMS0xNVQxMjowMDowMFo` |
| `sign_b64u` | Blake3 签名（Base64URL 编码） | `abc123...` |

**完整示例**：
```
ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded
```

### 2.2 签名算法

使用 **Blake3** 哈希算法生成签名：

```rust
fn token_sign_into_b64u(ident: &str, exp: &str, salt: Uuid, key: &[u8]) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
    
    let mut hasher = blake3::Hasher::new();
    hasher.update(content.as_bytes());  // Token 内容
    hasher.update(salt.as_bytes());     // 用户 Salt（每用户唯一）
    hasher.update(key);                 // 服务端密钥
    
    let result = hasher.finalize();
    Ok(b64u_encode(result.as_bytes()))
}
```

**签名输入**：
- Token 内容：`ident_b64u.exp_b64u`
- 用户 Salt：存储在 `user` 表的 `token_salt` 字段
- 服务端密钥：环境变量 `SERVICE_TOKEN_KEY`

### 2.3 Token 生命周期

```
登录请求
    ↓
验证用户名密码
    ↓
generate_web_token(username, token_salt)
    ↓
设置 Cookie（auth-token）
    ↓
后续请求携带 Cookie
    ↓
validate_web_token(token, token_salt)
    ↓
验证签名 + 检查过期时间
```

### 2.4 公共接口

```rust
/// 生成 Web Token
pub fn generate_web_token(user: &str, salt: Uuid) -> Result<Token>

/// 验证 Web Token
pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> Result<()>
```

### 2.5 Token 刷新策略

当前设计不包含 Refresh Token，采用简化方案：

- Token 有效期通过 `SERVICE_TOKEN_DURATION_SEC` 配置
- 过期后需重新登录
- 可通过修改用户的 `token_salt` 强制失效所有该用户的 Token

**Token 失效方式**：

| 场景 | 处理方式 |
|------|---------|
| Token 过期 | 返回 `Error::Expired`，前端跳转登录 |
| 用户登出 | 清除 Cookie |
| 强制下线 | 更新用户 `token_salt`，所有旧 Token 签名验证失败 |
| 密码修改 | 可选：更新 `token_salt` 使旧 Token 失效 |

### 2.6 与标准 JWT 的对比

| 特性 | 本项目 Token | 标准 JWT |
|------|-------------|----------|
| 格式 | 3 段 Base64URL | 3 段 Base64（Header.Payload.Signature） |
| Header | 无 | 包含算法等元信息 |
| Payload | 仅 ident + exp | 支持自定义 Claims |
| 签名算法 | Blake3 | RS256/HS256 等 |
| 服务端状态 | 需要查询用户 Salt | 完全无状态 |
| 强制失效 | 更新 Salt 即可 | 需要黑名单机制 |

---

## 三、密码哈希多模式设计

### 3.1 设计理念

密码哈希算法会随时间演进（更安全的算法出现）。本设计支持：

1. **平滑升级**：新用户使用新算法，旧用户保持旧算法
2. **自动迁移**：验证时检测算法版本，提示需要重新哈希
3. **向后兼容**：支持验证所有历史版本的哈希

### 3.2 哈希格式

密码哈希存储格式：

```
#{scheme_name}#{hashed_content}
```

**示例**：
```
#01#qO9A90161DoewhNXFwVcnAaljRI...  (Scheme01: HMAC-SHA512)
#02#$argon2id$v=19$m=19456...       (Scheme02: Argon2id)
```

### 3.3 Scheme 架构

```rust
/// Scheme trait 定义
#[enum_dispatch]
pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;
    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()>;
}

/// 使用 enum_dispatch 实现动态分发
#[enum_dispatch(Scheme)]
pub enum SchemeDispatcher {
    Scheme01(scheme_01::Scheme01),
    Scheme02(scheme_02::Scheme02),
}

/// 根据 scheme 名称获取实现
pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
    match scheme_name {
        "01" => Ok(SchemeDispatcher::Scheme01(Scheme01)),
        "02" => Ok(SchemeDispatcher::Scheme02(Scheme02)),
        _ => Err(Error::SchemeNotFound(scheme_name.to_string())),
    }
}
```

### 3.4 当前支持的 Scheme

#### Scheme01: HMAC-SHA512（已废弃，仅兼容）

```rust
pub struct Scheme01;

impl Scheme for Scheme01 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key)?;
        hmac_sha512.update(content.as_bytes());
        hmac_sha512.update(salt.as_bytes());
        Ok(b64u_encode(hmac_sha512.finalize().into_bytes()))
    }
}
```

| 属性 | 值 |
|------|-----|
| 算法 | HMAC-SHA512 |
| 状态 | 废弃（仅用于验证旧密码） |
| 安全性 | 不推荐用于新密码 |

#### Scheme02: Argon2id（当前默认）

```rust
pub struct Scheme02;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let argon2 = Argon2::new_with_secret(
            key,
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default(),
        )?;
        Ok(argon2.hash_password(content, &salt)?.to_string())
    }
}
```

| 属性 | 值 |
|------|-----|
| 算法 | Argon2id（抗 GPU/ASIC 攻击） |
| 版本 | V0x13 |
| 参数 | 默认参数（m=19456, t=2, p=1） |
| 状态 | **当前默认** |

### 3.5 公共接口

```rust
/// 哈希密码（使用默认 Scheme）
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String>

/// 验证密码（自动识别 Scheme）
pub async fn validate_pwd(to_hash: ContentToHash, pwd_ref: String) -> Result<SchemeStatus>
```

### 3.6 SchemeStatus 与自动升级

```rust
pub enum SchemeStatus {
    Ok,       // 使用最新 Scheme
    Outdated, // 使用旧 Scheme，建议重新哈希
}
```

**升级流程**：

```rust
// 登录验证后
let status = validate_pwd(content_to_hash, stored_pwd).await?;

if matches!(status, SchemeStatus::Outdated) {
    // 使用新 Scheme 重新哈希并更新数据库
    let new_pwd = hash_pwd(content_to_hash).await?;
    update_user_password(user_id, new_pwd).await?;
}
```

### 3.7 异步处理

密码哈希是 CPU 密集型操作，使用 `spawn_blocking` 避免阻塞：

```rust
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
    tokio::task::spawn_blocking(move || {
        hash_for_scheme(DEFAULT_SCHEME, to_hash)
    })
    .await
    .map_err(|_| Error::FailSpawnBlockForHash)?
}
```

---

## 四、配置管理

### 4.1 环境变量

| 变量名 | 类型 | 说明 |
|--------|------|------|
| `SERVICE_PWD_KEY` | Base64URL | 密码哈希密钥（建议 64 字节） |
| `SERVICE_TOKEN_KEY` | Base64URL | Token 签名密钥（建议 64 字节） |
| `SERVICE_TOKEN_DURATION_SEC` | f64 | Token 有效期（秒） |

### 4.2 配置结构

```rust
pub struct AuthConfig {
    pub PWD_KEY: Vec<u8>,           // 密码密钥
    pub TOKEN_KEY: Vec<u8>,         // Token 密钥
    pub TOKEN_DURATION_SEC: f64,    // Token 有效期
}
```

### 4.3 配置示例

```env
# .env
SERVICE_PWD_KEY=your-base64url-encoded-64-byte-key
SERVICE_TOKEN_KEY=your-base64url-encoded-64-byte-key
SERVICE_TOKEN_DURATION_SEC=86400  # 24 小时
```

### 4.4 密钥生成

使用项目提供的工具生成密钥：

```bash
cargo run -p gen-key
```

输出示例：
```
Generated keys (Base64URL encoded):
PWD_KEY:   abcdefghijklmnop...
TOKEN_KEY: qrstuvwxyz012345...
```

---

## 五、扩展指南

### 5.1 添加新的密码 Scheme

**Step 1**: 创建 Scheme 文件

```rust
// lib-auth/src/pwd/scheme/scheme_03.rs
use super::{Error, Result, Scheme};
use crate::pwd::ContentToHash;

pub struct Scheme03;

impl Scheme for Scheme03 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        // 实现新算法
        todo!()
    }

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
        let new_hash = self.hash(to_hash)?;
        if new_hash == pwd_ref {
            Ok(())
        } else {
            Err(Error::PwdValidate)
        }
    }
}
```

**Step 2**: 注册到调度器

```rust
// lib-auth/src/pwd/scheme/mod.rs
mod scheme_03;

pub const DEFAULT_SCHEME: &str = "03";  // 更新默认 Scheme

#[enum_dispatch(Scheme)]
pub enum SchemeDispatcher {
    Scheme01(scheme_01::Scheme01),
    Scheme02(scheme_02::Scheme02),
    Scheme03(scheme_03::Scheme03),  // 添加新 Scheme
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
    match scheme_name {
        "01" => Ok(SchemeDispatcher::Scheme01(Scheme01)),
        "02" => Ok(SchemeDispatcher::Scheme02(Scheme02)),
        "03" => Ok(SchemeDispatcher::Scheme03(Scheme03)),  // 添加匹配
        _ => Err(Error::SchemeNotFound(scheme_name.to_string())),
    }
}
```

**Step 3**: 验证

```bash
cargo test -p lib-auth
```

### 5.2 切换到标准 JWT

如需使用标准 JWT，可按以下方向修改：

1. 添加 `jsonwebtoken` 依赖
2. 修改 Token 结构支持 Claims
3. 修改签名逻辑使用 RS256/HS256
4. 更新验证逻辑

**注意**：标准 JWT 不支持服务端强制失效，需要额外实现黑名单机制。

### 5.3 添加 Refresh Token

如需支持 Refresh Token：

1. 在 Token 结构中添加 `token_type` 字段
2. 生成时创建 access_token + refresh_token
3. 添加 refresh 端点
4. refresh_token 使用更长有效期

---

## 相关文档

- [开发手册](./DEVELOPMENT.md) - 整体开发流程
- [错误处理系统设计](./ERROR_DESIGN.md) - Token/密码错误处理
