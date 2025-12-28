# TypeScript 类型导出指南

本项目使用 [ts-rs](https://github.com/Aleph-Alpha/ts-rs) 从 Rust 实体自动生成 TypeScript 类型定义。

## 快速开始

### 完整导出流程

```bash
# 1. 导出 Rust 类型
cargo test -p lib-core --features with-ts export_ts_types

# 2. 后处理（格式化 + 生成 index.ts）
cd cmx-vue-ultimate-starter
npm run gen:types
```

生成的文件结构:

```
cmx-vue-ultimate-starter/src/types/generated/
├── index.ts          # 统一导出
├── user/             # 用户相关类型
│   ├── index.ts
│   ├── User.ts
│   └── UserTyp.ts
├── acs/              # 权限控制类型
│   ├── index.ts
│   ├── Permission.ts
│   ├── PermissionLite.ts
│   ├── Role.ts
│   └── RoleLite.ts
└── agent/            # Agent 相关类型
    ├── index.ts
    └── Agent.ts
```

### 前端使用

```typescript
import type { User, Permission, Role, Agent } from '@/types/generated'
```

## 添加新实体导出

### 步骤 1: 在实体文件添加 TS 宏

```rust
// 1. 添加条件导入
#[cfg(feature = "with-ts")]
use ts_rs::TS;

// 2. 在结构体上添加 derive 和导出配置
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "with-ts", derive(TS))]
#[cfg_attr(feature = "with-ts", ts(export, export_to = "../../../../cmx-vue-ultimate-starter/src/types/generated/"))]
pub struct YourEntity {
    pub id: i64,
    pub name: String,
    
    // 3. 时间字段需要标注类型映射
    #[cfg_attr(feature = "with-ts", ts(type = "string"))]
    pub ctime: OffsetDateTime,
}
```

### 步骤 2: 更新导出测试文件

编辑 `crates/libs/lib-core/tests/ts_export.rs`:

```rust
// 添加导入
use lib_core::model::your_module::YourEntity;

// 在 export_ts_types 函数中添加导出
YourEntity::export_all().expect("Failed to export YourEntity");
```

### 步骤 3: 运行导出命令

```bash
cargo test -p lib-core --features with-ts export_ts_types
```

### 步骤 4: 更新前端索引文件（可选）

编辑 `cmx-vue-ultimate-starter/src/types/generated/index.ts`:

```typescript
export type { YourEntity } from './YourEntity'
```

## 类型映射说明

| Rust 类型 | TypeScript 类型 | 说明 |
|-----------|-----------------|------|
| `i64` | `bigint` | 自动映射 |
| `String` | `string` | 自动映射 |
| `Option<T>` | `T \| null` | 自动映射 |
| `Vec<T>` | `T[]` | 自动映射 |
| `Uuid` | `string` | 需要 `uuid-impl` feature |
| `OffsetDateTime` | 需手动指定 | 使用 `#[ts(type = "string")]` |

## 注意事项

1. **统一路径深度**: ts-rs 的 `export_to` 是相对于 Cargo.toml，所有模块使用相同的基础路径 `../../../../cmx-vue-ultimate-starter/src/types/generated/`，再加上子目录名

2. **serde_as 字段**: 使用 `#[serde_as(as = "...")]` 的字段必须添加 `#[ts(type = "...")]`

3. **枚举类型**: 枚举会自动导出为 TypeScript union 类型

4. **子目录 index.ts**: 每个子目录需要手动创建 `index.ts` 并导出类型

## 已导出的类型

| 目录 | 类型 | 说明 |
|------|------|------|
| `user/` | `User`, `UserTyp` | 用户相关 |
| `acs/` | `Permission`, `PermissionLite`, `Role`, `RoleLite` | 权限控制相关 |
| `agent/` | `Agent` | Agent 相关 |
