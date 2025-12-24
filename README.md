- 搜索 `mm.dbx().begin_txn()` 可以在 `UserBmc::create` 中找到示例。

- **3) 声明式宏**
	- 为了减少样板代码，此 Rust10x 蓝图现在在 `lib_rpc` 和 `lib_core::model` 层级支持灵活的声明式宏（即 `macro_rules`）。这些宏创建通用的基本 CRUD JSON-RPC 函数和通用的 BMC CRUD 方法。
		- 搜索 `generate_common_bmc_fns` 或 `generate_common_rpc_fns` 查看它们的实际应用。

- **4) 代码更新**
	- 所有 JSON-RPC 响应现在都包含一个 `.data` 字段作为 `result.data` 来表示请求的数据。这增加了灵活性，便于后续在 `result` 对象的根级别添加元数据（JSON-RPC 规范禁止在 JSON 响应的根级别添加任何内容）。
		- 这在 `lib_rpc::response` crate/模块中。
	- 在 `Ctx` 中引入 `conv_id` 为未来的 `访问控制系统` 铺平了道路，该系统将基于权限并与关键容器结构（如 `Org`、`Space`、`Conv`）绑定。

## Rust10x Web App YouTube 视频:

- [第 01 集 - Rust Web App - 生产级基础代码](https://youtube.com/watch?v=3cA_mk4vdWY&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
    - [专题视频 - 代码整洁 - 单元测试中使用 `#[cfg_attr(...)]`](https://www.youtube.com/watch?v=DCPs5VRTK-U&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [专题视频 - 区分 ModelControllers 和 ModelManager 的设计理念](https://www.youtube.com/watch?v=JdLi69mWIIE&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [专题视频 - Base64Url - 理解 Base64URL 的用法和意义](https://www.youtube.com/watch?v=-9K7zNgsbP0&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [第 02 集 - Sea-Query (SQL 构建器) & modql (类 MongoDB 过滤器)](https://www.youtube.com/watch?v=-dMH9UiwKqg&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [第 03 集 - Cargo Workspace (多 crate 项目)](https://www.youtube.com/watch?v=zUxF0kvydJs&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [AI 语音重制版](https://www.youtube.com/watch?v=iCGIqEWWTcA&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [第 04 集 - 多方案密码哈希](https://www.youtube.com/watch?v=3E0zK5h9zEs&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [第 05 集 - JSON-RPC 动态路由](https://www.youtube.com/watch?v=Gc5Nj5LJe1U&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- **第 06 集 - 如有需要请在 [Discord](https://discord.gg/XuKWrNGKpC) 上提出请求**

- 其他相关视频: 
	- [Rust Axum 完整教程](https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)


## 启动数据库

```sh
# 启动 postgresql 服务器 docker 镜像:
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:17

# (可选) 在 pg 上运行 psql 终端
# 在另一个终端（标签页）运行 psql:
docker exec -it -u postgres pg psql

# (可选) 让 pg 打印所有 sql 语句
# 在上面启动的 psql 命令行中执行:
ALTER DATABASE postgres SET log_statement = 'all';
```

## 开发模式 (监听)

> 注意: 使用 `cargo install cargo-watch` 安装 cargo watch。

```sh
# 终端 1 - 运行服务器
cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -x "run -p web-server"

# 终端 2 - 运行 quick_dev
cargo watch -q -c -w crates/services/web-server/examples/ -x "run -p web-server --example quick_dev"
```

## 开发模式

```sh
# 终端 1 - 运行服务器
cargo run -p web-server

# 终端 2 - 运行测试
cargo run -p web-server --example quick_dev
```

## 单元测试 (监听)

```sh
cargo watch -q -c -x "test -- --nocapture"

# 使用过滤器运行特定测试
cargo watch -q -c -x "test -p lib-core test_create -- --nocapture"
```

## 单元测试

```sh
cargo test -- --nocapture

cargo watch -q -c -x "test -p lib-core model::task::tests::test_create -- --nocapture"
```

## 工具

```sh
cargo run -p gen-key
```

<br />

---

更多 [Rust 生产级编程](https://rust10x.com) 资源


[GitHub 仓库地址](https://github.com/rust10x/rust-web-app)
