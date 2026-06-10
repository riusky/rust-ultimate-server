# TypeScript 类型导出指南

本项目使用 `ts-rs` 从 Rust model 自动生成前端 TypeScript 类型。导出入口已经封装到 `shell/gen-ts-types.sh`，新增类型时不需要手动维护 `tests/ts_export.rs`。

## 快速开始

```bash
bash shell/gen-ts-types.sh
```

脚本会依次执行：

1. `cargo test -p lib-core --features with-ts export_ts_types -- --nocapture`
2. `cd cmx-vue-ultimate-starter && bun run gen:types`

生成目录：

```text
cmx-vue-ultimate-starter/src/services/types/
├── index.ts
├── acs/
├── agent/
├── filter/
└── user/
```

前端使用示例：

```typescript
import type { User, Permission, Role, Agent } from '@/services/types'
```

## 添加新实体导出

在需要导出的 struct 或 enum 上添加 `frontend_type` 标记：

```rust
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "with-ts", lib_macros::frontend_type(dir = "your_module"))]
pub struct YourEntity {
	pub id: i64,
	pub name: String,

	#[cfg_attr(feature = "with-ts", ts(type = "string"))]
	pub ctime: OffsetDateTime,
}
```

`dir` 对应前端子目录，例如：

| `dir` | 输出目录 |
| --- | --- |
| `user` | `cmx-vue-ultimate-starter/src/services/types/user/` |
| `acs` | `cmx-vue-ultimate-starter/src/services/types/acs/` |
| `agent` | `cmx-vue-ultimate-starter/src/services/types/agent/` |

`#[frontend_type]` 会自动：

- 派生 `ts_rs::TS`
- 设置 `ts-rs` 导出目录
- 注册到 `inventory` 导出清单

因此不需要再手动导入 `ts_rs::TS`，也不需要修改 `crates/libs/lib-core/tests/ts_export.rs`。

## 开发监听

如果安装了 `cargo-watch`，可以使用：

```bash
bash shell/watch-ts-types.sh
```

该脚本会监听 `crates/libs/lib-core/src/model` 和 `vendor/modql/src/filter`，变更后自动运行完整生成流程。

## 注意事项

1. `OffsetDateTime`、`Date` 等前端需要字符串表示的字段仍需保留字段级 `#[cfg_attr(feature = "with-ts", ts(type = "..."))]`。
2. `serde_as` 字段如果不能被 `ts-rs` 正确推断，也需要字段级 `ts(type = "...")`。
3. `cmx-vue-ultimate-starter/scripts/post-process-ts-types.ts` 会自动生成各目录 `index.ts`，不要手动维护生成目录下的索引文件。
4. `frontend_type` 目前只用于非泛型 struct/enum。
