# db-deployer

基于 RSA 签名验证的生产环境数据库部署与升级工具。

## 项目概述

**db-deployer** 是一个专为生产环境设计的数据库部署和升级工具，提供安全、可追溯、可回滚的数据库迁移能力。

### 核心特性

| 特性 | 说明 |
|------|------|
| **RSA 签名验证** | 使用 RSA 私钥签名部署记录，防止篡改 |
| **SQL 文件变更检测** | SHA256 哈希校验，禁止修改或删除已执行的 SQL 文件 |
| **TEMPLATE 复制** | 利用 PostgreSQL TEMPLATE 机制秒级复制数据库 |
| **部署锁机制** | 防止并发部署导致的冲突 |
| **Git 自动提交** | 自动将部署记录提交到 Git 仓库，确保可追溯 |
| **双模式支持** | 支持全新部署（INIT）和增量升级（UPGRADE）两种模式 |
| **版本保留** | 自动保留最近 N 个数据库版本，支持快速回滚 |

---

## 运行流程

### 模式检测

工具启动时会自动检测当前应该使用哪种模式：

```
启动
  │
  ▼
检查 current.json 是否存在？
  │
  ├── 不存在 ──► INIT 模式（全新部署）
  │
  └── 存在 ────► UPGRADE 模式（版本升级）
```

---

### INIT 模式（全新部署）

适用于首次部署或全新服务器环境。

#### 完整流程

```
┌─────────────────────────────────────────────────────────────┐
│                    INIT 模式流程                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Step 1: 前置检查                                           │
│  ├── 检查 RSA 密钥对是否存在                                 │
│  ├── 检查部署锁状态                                         │
│  └── 获取部署锁                                             │
│                                                             │
│  Step 2: 扫描 SQL 文件                                      │
│  ├── 扫描 sql/init/ 目录                                    │
│  ├── 扫描 sql/updates/ 目录                                 │
│  └── 计算每个文件的 SHA256 哈希                              │
│                                                             │
│  Step 3: 创建数据库                                         │
│  ├── 生成新数据库名称（{prefix}_{YYYYMMDD}_{HHMMSS}）        │
│  └── 创建空数据库                                           │
│                                                             │
│  Step 4: 执行初始化 SQL                                     │
│  ├── 按文件名顺序执行 sql/init/ 下的所有 SQL                 │
│  └── 任何执行失败则删除新库并阻断部署                         │
│                                                             │
│  Step 5: 执行增量 SQL                                       │
│  ├── 按文件名顺序执行 sql/updates/ 下的所有 SQL              │
│  └── 任何执行失败则删除新库并阻断部署                         │
│                                                             │
│  Step 6: 保存部署记录                                       │
│  ├── 生成 current.json（包含所有已执行的 SQL 哈希）          │
│  └── 使用 RSA 私钥签名生成 current.json.sig                  │
│                                                             │
│  Step 7: Git 提交                                           │
│  ├── git add db-upgrade/                                    │
│  ├── git commit                                             │
│  └── git push（可选）                                       │
│                                                             │
│  Step 8: 完成                                               │
│  ├── 释放部署锁                                             │
│  └── 输出新数据库连接信息                                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 关键检查点

1. **密钥检查**：必须同时存在私钥和公钥
2. **SQL 目录检查**：`sql/init/` 目录必须存在且不为空
3. **数据库创建**：确保目标数据库名不存在
4. **SQL 执行**：每个 SQL 文件执行失败都会触发回滚

---

### UPGRADE 模式（版本升级）

适用于已有生产数据库的增量升级。

#### 完整流程

```
┌─────────────────────────────────────────────────────────────┐
│                   UPGRADE 模式流程                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Step 1: 前置检查                                           │
│  ├── 检查 RSA 密钥对是否存在                                 │
│  ├── 检查部署锁状态                                         │
│  └── 获取部署锁                                             │
│                                                             │
│  Step 2: 验证部署记录                                       │
│  ├── 加载 current.json                                      │
│  ├── 使用 RSA 公钥验证签名                                  │
│  └── 签名无效则阻断部署                                      │
│                                                             │
│  Step 3: SQL 文件校验                                       │
│  ├── 扫描 sql/updates/ 目录                                 │
│  ├── 对比已记录的 SQL 文件哈希                               │
│  ├── 检测是否有文件被修改 → 阻断                             │
│  ├── 检测是否有文件被删除 → 阻断                             │
│  └── 识别新增的 SQL 文件                                    │
│                                                             │
│  Step 4: 创建新数据库                                       │
│  ├── 生成新数据库名称                                       │
│  ├── 断开源数据库所有连接                                    │
│  └── 使用 TEMPLATE 复制源数据库到新数据库                    │
│                                                             │
│  Step 5: 执行新增 SQL                                       │
│  ├── 按文件名顺序执行新增的 SQL 文件                         │
│  └── 任何执行失败则删除新库并阻断部署                         │
│                                                             │
│  Step 6: 归档历史记录                                       │
│  ├── 将当前 current.json 复制到 history/ 目录               │
│  └── 更新 current.json 中的历史列表                          │
│                                                             │
│  Step 7: 保存新记录                                         │
│  ├── 更新 current.json（新增已执行的 SQL 记录）              │
│  └── 重新签名生成 current.json.sig                           │
│                                                             │
│  Step 8: 清理旧版本                                         │
│  ├── 根据 retain_versions 配置                              │
│  └── 删除超出保留数量的旧数据库                              │
│                                                             │
│  Step 9: Git 提交                                           │
│  ├── git add db-upgrade/                                    │
│  ├── git commit                                             │
│  └── git push（可选）                                       │
│                                                             │
│  Step 10: 完成                                              │
│  ├── 释放部署锁                                             │
│  └── 输出新数据库连接信息                                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 关键检查点

1. **签名验证**：current.json 的签名必须有效
2. **哈希校验**：已记录的 SQL 文件不允许修改或删除
3. **TEMPLATE 复制**：复制期间源库短暂只读
4. **SQL 执行**：失败时自动回滚（删除新创建的数据库）

---

## 配置文件说明

### 配置文件位置

默认配置文件：`db-deployer.toml`

可通过 `-c` 或 `--config` 参数指定其他路径：
```bash
db-deployer -c /path/to/config.toml deploy
```

### 完整配置示例

```toml
# ============================================================
# 数据库连接配置
# ============================================================
[database]
# PostgreSQL 主机地址
host = "localhost"

# PostgreSQL 端口
port = 5432

# 数据库用户名
user = "postgres"

# 数据库密码
password = "your-password-here"

# 数据库名称前缀
# 新创建的数据库将命名为：{name_prefix}_{YYYYMMDD}_{HHMMSS}
# 例如：myapp_20251231_183000
name_prefix = "myapp"

# 管理数据库名称（用于执行 CREATE DATABASE 等管理操作）
admin_db = "postgres"

# ============================================================
# 路径配置
# ============================================================
[paths]
# 初始化 SQL 目录
# 存放数据库初始化脚本，仅在 INIT 模式时执行
# 文件按文件名排序执行，建议使用数字前缀：
#   00-create-schema.sql
#   01-create-tables.sql
#   02-create-indexes.sql
#   99-init-data.sql
init_sql_dir = "sql/init"

# 增量更新 SQL 目录
# 存放版本升级脚本，INIT 和 UPGRADE 模式都会执行
# 文件命名规范：drop-{主版本}-{补丁版本}-{序号}-{描述}.sql
# 例如：drop-0001-00-00-add-user-settings.sql
updates_sql_dir = "sql/updates"

# 部署记录目录
# 存放 current.json、签名文件和历史记录
records_dir = "db-upgrade"

# ============================================================
# RSA 密钥配置
# ============================================================
[keys]
# RSA 私钥路径（用于签名，不要提交到 Git）
private_key = "keys/private.pem"

# RSA 公钥路径（用于验签，可以提交到 Git）
public_key = "keys/public.pem"

# ============================================================
# Git 配置
# ============================================================
[git]
# 是否自动提交部署记录
auto_commit = true

# 是否自动推送到远程仓库
auto_push = true

# 提交消息前缀
commit_prefix = "chore(db)"

# ============================================================
# 部署配置
# ============================================================
[deploy]
# 保留的历史版本数量
# 超出此数量的旧数据库将被自动删除
retain_versions = 3

# 是否要求确认后才执行部署
# 设为 false 可跳过确认提示
require_confirmation = true
```

### 配置项详解

#### [database] 数据库配置

| 配置项 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `host` | String | 是 | PostgreSQL 服务器地址 |
| `port` | u16 | 是 | PostgreSQL 端口，默认 5432 |
| `user` | String | 是 | 数据库用户名，需要有创建数据库权限 |
| `password` | String | 是 | 数据库密码 |
| `name_prefix` | String | 是 | 新数据库的名称前缀 |
| `admin_db` | String | 否 | 管理数据库，默认 "postgres" |

#### [paths] 路径配置

| 配置项 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `init_sql_dir` | String | 是 | 初始化 SQL 文件目录 |
| `updates_sql_dir` | String | 是 | 增量 SQL 文件目录 |
| `records_dir` | String | 是 | 部署记录存储目录 |

#### [keys] 密钥配置

| 配置项 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `private_key` | String | 是 | RSA 私钥文件路径 |
| `public_key` | String | 是 | RSA 公钥文件路径 |

#### [git] Git 配置

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `auto_commit` | bool | true | 自动提交部署记录 |
| `auto_push` | bool | true | 自动推送到远程 |
| `commit_prefix` | String | "chore(db)" | 提交消息前缀 |

#### [deploy] 部署配置

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `retain_versions` | usize | 3 | 保留的历史版本数量 |
| `require_confirmation` | bool | true | 是否要求确认 |

#### [project] 项目部署配置（可选）

完整部署流程（`full-deploy` 命令）需要此配置。

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `repo_url` | String | - | 项目 Git 仓库地址（必填） |
| `branch` | String | "main" | 部署分支 |
| `project_dir` | String | - | 项目存放目录（必填） |
| `docker_compose_file` | String | "docker-compose.yml" | Docker Compose 文件路径 |
| `sql_source_dir` | String | "sql" | 项目中的 SQL 目录 |
| `service_name` | String | "web-server" | 服务名称 |
| `db_url_env_name` | String | "SERVICE_DB_URL" | 数据库连接环境变量名 |

---

## 使用方法

### 命令概览

```bash
db-deployer [OPTIONS] <COMMAND>

Commands:
  deploy       仅执行数据库部署或升级
  full-deploy  完整部署：更新项目、部署数据库、重启服务
  status       查看当前部署状态
  init-keys    生成 RSA 密钥对

Options:
  -c, --config <FILE>  配置文件路径 [默认: db-deployer.toml]
  -h, --help           显示帮助信息
  -V, --version        显示版本信息
```

---

### init-keys - 生成密钥对

首次使用前需要生成 RSA 密钥对。

```bash
# 生成密钥对
db-deployer init-keys

# 强制覆盖已存在的密钥
db-deployer init-keys --force
```

**参数说明：**

| 参数 | 说明 |
|------|------|
| `-f, --force` | 强制覆盖已存在的密钥文件 |

**输出示例：**

```
============================================================
  RSA Key Generation
============================================================

Generating 2048-bit RSA key pair...
  Private key saved: keys/private.pem
  Public key saved: keys/public.pem

Keys generated successfully!

Important:
  - DO NOT commit the private key (add to .gitignore)
  - The public key can be committed to git
```

---

### status - 查看状态

查看当前部署状态和配置信息。

```bash
# 查看基本状态
db-deployer status

# 查看详细信息
db-deployer status --verbose
```

**参数说明：**

| 参数 | 说明 |
|------|------|
| `-v, --verbose` | 显示详细信息（包括历史记录和 SQL 文件列表） |

**输出示例：**

```
============================================================
  Database Deployment Status
============================================================

  Deployment lock: Available
  RSA Keys: Present

  Current record: Exists
    Signature: Valid
    Mode: Upgrade
    Database: myapp_20251231_183000
    Created: 2025-12-31 18:30:00
    Init files: 4
    Update files: 5
    History: 2 versions
    Retain versions: 3

  SQL Directories:
    Init (sql/init): 4 files
    Updates (sql/updates): 5 files
```

---

### deploy - 执行部署

执行数据库部署或升级。

```bash
# 自动检测模式执行部署
db-deployer deploy

# 强制使用 INIT 模式（全新部署）
db-deployer deploy --init

# 强制使用 UPGRADE 模式
db-deployer deploy --upgrade

# 跳过确认提示
db-deployer deploy -y

# 试运行（不实际执行）
db-deployer deploy --dry-run

# 组合使用
db-deployer deploy --init -y
```

**参数说明：**

| 参数 | 说明 |
|------|------|
| `--init` | 强制使用 INIT 模式 |
| `--upgrade` | 强制使用 UPGRADE 模式 |
| `-y, --yes` | 跳过确认提示 |
| `--dry-run` | 试运行，不实际执行任何操作 |

**输出示例（成功）：**

```
Acquiring deployment lock...
  Lock acquired

Detected mode: UPGRADE

Proceed with deployment? [y/N] y

============================================================
  UPGRADE Mode - Database Migration
============================================================

Verifying current record...
  Signature valid
  Current database: myapp_20251230_120000

Validating SQL files...
  2 new SQL files to apply
    - drop-0001-01-00-add-feature.sql
    - drop-0001-01-01-fix-bug.sql

  Source database: myapp_20251230_120000
  Target database: myapp_20251231_183000

Creating database from template...
  Database created via TEMPLATE

Executing new SQL files...
  [1/2] drop-0001-01-00-add-feature.sql... OK
  [2/2] drop-0001-01-01-fix-bug.sql... OK

Archiving current record...
  Archived

Saving new record...
  Record saved and signed

Committing to git...
  Git commit successful

============================================================
  Deployment Successful!
============================================================

  Database: myapp_20251231_183000

  Update your application configuration:
  DATABASE_URL=postgres://postgres:****@localhost:5432/myapp_20251231_183000
```

---

### full-deploy - 完整部署

执行完整的部署流程：更新项目代码 → 同步 SQL → 停止服务 → 部署数据库 → 修改配置 → 启动服务。

**注意**：此命令需要配置 `[project]` 部分。

```bash
# 执行完整部署
db-deployer full-deploy

# 强制使用 INIT 模式
db-deployer full-deploy --init

# 强制使用 UPGRADE 模式
db-deployer full-deploy --upgrade

# 跳过确认提示
db-deployer full-deploy -y

# 试运行（不实际执行）
db-deployer full-deploy --dry-run
```

**参数说明：**

| 参数 | 说明 |
|------|------|
| `--init` | 强制使用 INIT 模式 |
| `--upgrade` | 强制使用 UPGRADE 模式 |
| `-y, --yes` | 跳过确认提示 |
| `--dry-run` | 试运行，不实际执行任何操作 |

**执行流程：**

```
┌─────────────────────────────────────────────────────────────┐
│                 Full Deploy 流程                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Step 1: 权限检查                                              │
│  ├── 检查 Docker 是否可用                                      │
│  ├── 检查本地仓库 Git 权限                                     │
│  └── 检查项目仓库 Git 权限                                     │
│                                                             │
│  Step 2: 更新项目仓库                                         │
│  ├── git fetch origin                                       │
│  ├── git clean -fd (清理未跟踪文件)                            │
│  └── git reset --hard origin/{branch}                       │
│                                                             │
│  Step 3: 同步 SQL 文件                                        │
│  └── 复制 {project_dir}/sql → 部署工具 sql 目录              │
│                                                             │
│  Step 4: 停止服务                                              │
│  └── docker compose stop {service_name}                     │
│                                                             │
│  Step 5: 部署数据库                                            │
│  └── 执行 INIT 或 UPGRADE 模式                                 │
│                                                             │
│  Step 6: 修改 docker-compose.yml                              │
│  └── 更新 SERVICE_DB_URL 为新数据库地址                        │
│  └── 注意：此修改不会提交到 Git，避免泄露数据库地址             │
│                                                             │
│  Step 7: 启动服务                                              │
│  └── docker compose up -d {service_name}                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**输出示例（成功）：**

```
============================================================
  Full Deployment Workflow
============================================================

Deployment Plan:
  1. Check Git permissions
  2. Update project repository (branch: main)
  3. Sync SQL files from project
  4. Stop service: web-server
  5. Deploy/upgrade database
  6. Update docker-compose.yml (SERVICE_DB_URL)
  7. Start service: web-server

WARNING: This will:
  - Discard all local changes in the project repository
  - Stop the running service (downtime expected)
  - Modify docker-compose.yml locally (will NOT be committed)

Proceed with full deployment? [y/N] y

Step 1: Checking permissions...
  Docker: OK
  Local Git permission: OK
  Project Git permission: OK

Step 2: Updating project repository...
  Fetching from origin...
  Cleaning untracked files...
  Resetting to origin/main...
  Current commit: abc1234 feat: add new feature
  Repository updated

Step 3: Syncing SQL files...
  Synced init SQL: /opt/apps/project/sql/init -> sql/init
  Synced updates SQL: /opt/apps/project/sql/updates -> sql/updates

Step 4: Stopping service...
  Service stopped

Step 5: Deploying database...
  [UPGRADE mode deployment output]

Step 6: Updating docker-compose.yml...
  Updated SERVICE_DB_URL to use database: myapp_20251231_190000
  NOTE: This change is local only. Do NOT commit this change to git.

Step 7: Starting service...
  Service started

============================================================
  Full Deployment Successful!
============================================================

  New database: myapp_20251231_190000
  Service: web-server (running)

Rollback info:
  If you need to rollback, update docker-compose.yml:
  SERVICE_DB_URL: postgres://user:****@localhost:5432/myapp_20251230_183000
  Then restart the service:
  docker compose -f /opt/apps/project/docker-compose.yml restart web-server
```

---

## 安全机制

### RSA 密钥验证流程

```
┌─────────────────────────────────────────────────────────────┐
│                    签名与验证流程                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  部署完成时（签名）：                                        │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐ │
│  │ current.json│ -> │ RSA 私钥签名 │ -> │ current.json.sig│ │
│  └─────────────┘    └──────────────┘    └─────────────────┘ │
│                                                             │
│  下次部署时（验证）：                                        │
│  ┌─────────────┐    ┌─────────────────┐    ┌─────────────┐  │
│  │ current.json│    │ current.json.sig│    │ RSA 公钥验签│  │
│  └──────┬──────┘    └────────┬────────┘    └──────┬──────┘  │
│         │                    │                    │         │
│         └────────────────────┴────────────────────┘         │
│                              │                              │
│                              ▼                              │
│                    ┌─────────────────┐                      │
│                    │ 签名匹配？       │                      │
│                    └────────┬────────┘                      │
│                    是 │           │ 否                      │
│                       ▼           ▼                         │
│               ┌──────────┐  ┌──────────────┐                │
│               │ 继续部署 │  │ 阻断并报错   │                │
│               └──────────┘  └──────────────┘                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 防篡改机制

工具通过以下机制确保部署记录和 SQL 文件的完整性：

#### 1. SQL 文件哈希校验

```
每次部署时：
1. 扫描 SQL 目录下的所有文件
2. 计算每个文件的 SHA256 哈希
3. 与 current.json 中记录的哈希对比
4. 如果发现：
   - 已记录的文件被修改 → 阻断部署
   - 已记录的文件被删除 → 阻断部署
   - 新增文件 → 添加到待执行列表
```

#### 2. 部署记录签名

```
current.json 包含：
- 当前数据库名称
- 已执行的所有 SQL 文件及其哈希
- 历史版本记录
- 部署时间戳

current.json.sig 是 current.json 的 RSA 签名
- 签名使用 RSA-2048 + SHA256
- 任何对 current.json 的修改都会导致签名验证失败
```

#### 3. Git 版本控制

```
每次部署后自动提交：
- db-upgrade/current.json
- db-upgrade/current.json.sig
- db-upgrade/history/*.json

提交记录包含：
- 新数据库名称
- 执行的 SQL 文件列表
- 上一个版本的数据库名称
```

### 部署锁机制

部署锁用于防止并发部署导致的冲突：

```
┌─────────────────────────────────────────────────────────────┐
│                      部署锁机制                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  锁文件位置：{records_dir}/deploy.lock                       │
│                                                             │
│  锁文件内容：                                                │
│  {                                                          │
│    "pid": 12345,                                            │
│    "started_at": "2025-12-31T18:30:00Z",                    │
│    "hostname": "deploy-server-01"                           │
│  }                                                          │
│                                                             │
│  获取锁流程：                                                │
│  1. 检查锁文件是否存在                                       │
│  2. 如果存在且内容有效 → 阻断（其他部署进行中）               │
│  3. 如果不存在或内容无效 → 创建锁文件                        │
│                                                             │
│  释放锁流程：                                                │
│  1. 部署完成（成功或失败）                                   │
│  2. 删除锁文件                                              │
│  3. 即使程序异常退出，锁文件也会在 Drop 时自动清理           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 注意事项

### 生产环境部署

1. **私钥安全**
   - 私钥文件 (`keys/private.pem`) 必须妥善保管
   - 添加到 `.gitignore`，**绝对不要提交到版本控制**
   - 考虑使用密钥管理服务或环境变量

2. **数据库权限**
   - 配置的数据库用户需要 `CREATEDB` 权限
   - 需要能够执行 `pg_terminate_backend()` 终止其他连接

3. **TEMPLATE 复制期间**
   - 源数据库会短暂只读（通常几秒钟）
   - 建议在低峰期执行部署

4. **SQL 文件规范**
   - 使用统一的命名规范，确保文件按正确顺序执行
   - 每个 SQL 文件应该是幂等的或包含适当的检查
   - 建议在 SQL 文件中添加注释说明变更内容

### 错误处理和回滚

#### 自动回滚场景

| 场景 | 处理方式 |
|------|---------|
| SQL 执行失败 | 自动删除新创建的数据库 |
| TEMPLATE 复制失败 | 源数据库不受影响 |
| Git 提交失败 | 仅警告，不阻断（本地记录已保存） |

#### 手动回滚步骤

如果需要回滚到之前的数据库版本：

1. 查看历史版本：
   ```bash
   db-deployer status --verbose
   ```

2. 修改应用配置，指向历史版本的数据库：
   ```
   DATABASE_URL=postgres://user:pwd@host:5432/myapp_20251230_120000
   ```

3. 重启应用

#### 异常恢复

如果部署过程中发生异常（如断电、进程被杀）：

1. **锁文件残留**
   - 锁文件可能未被正确清理
   - 手动删除：`rm db-upgrade/deploy.lock`
   - 或检查锁文件中的进程是否还在运行

2. **部分执行的数据库**
   - 可能存在创建但未完成的数据库
   - 使用 `psql` 手动删除：`DROP DATABASE myapp_YYYYMMDD_HHMMSS;`

3. **记录不一致**
   - 如果 current.json 与实际状态不一致
   - 需要手动修复或从 Git 历史恢复

### 兼容性要求

| 组件 | 最低版本 | 说明 |
|------|---------|------|
| PostgreSQL | 9.6+ | 需要支持 TEMPLATE 复制 |
| Rust | 1.70+ | 编译要求 |
| Git | 2.0+ | 用于自动提交 |

### SQL 文件命名规范

#### 初始化 SQL（sql/init/）

```
00-create-schema.sql
01-create-tables.sql
02-create-indexes.sql
03-create-functions.sql
99-init-data.sql
```

#### 增量 SQL（sql/updates/）

```
drop-{主版本}-{补丁}-{序号}-{描述}.sql

例如：
drop-0001-00-00-add-user-settings.sql
drop-0001-00-01-add-project-table.sql
drop-0001-01-00-fix-column-type.sql
drop-0002-00-00-major-refactor.sql
```

**命名规则说明：**
- `主版本`：4 位数字，主要版本号
- `补丁`：2 位数字，补丁版本号
- `序号`：2 位数字，同版本内的执行顺序
- `描述`：简短的变更描述，使用连字符分隔

---

## 文件结构

```
项目根目录/
├── db-deployer.toml          # 配置文件
│
├── keys/                     # RSA 密钥目录
│   ├── private.pem           # 私钥（不提交 Git）
│   └── public.pem            # 公钥（可提交 Git）
│
├── sql/                      # SQL 文件目录
│   ├── init/                 # 初始化 SQL
│   │   ├── 00-create-schema.sql
│   │   └── ...
│   └── updates/              # 增量更新 SQL
│       ├── drop-0001-00-00-xxx.sql
│       └── ...
│
└── db-upgrade/               # 部署记录目录
    ├── current.json          # 当前部署状态
    ├── current.json.sig      # RSA 签名
    ├── deploy.lock           # 部署锁（临时）
    └── history/              # 历史记录
        ├── 20251230_120000.json
        └── 20251230_120000.json.sig
```

---

## License

MIT
