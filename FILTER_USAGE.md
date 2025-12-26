# 过滤功能使用指南

本文档详细介绍项目中 REST API 和 RPC API 的过滤、分页、排序功能用法。

---

## 目录

- [一、Filter 类型定义](#一filter-类型定义)
- [二、过滤操作符](#二过滤操作符)
- [三、REST API 用法](#三rest-api-用法)
- [四、RPC API 用法](#四rpc-api-用法)
- [五、注意事项](#五注意事项)

---

## 一、Filter 类型定义

在 `lib-core/src/model/` 中为每个实体定义对应的 Filter 结构：

```rust
use modql::filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue};

#[derive(Clone, FilterNodes, Default, Deserialize)]
pub struct AgentFilter {
    pub id: Option<OpValsInt64>,        // 整数过滤
    pub name: Option<OpValsString>,     // 字符串过滤
    pub cid: Option<OpValsInt64>,       // 创建者ID
    pub ctime: Option<OpValsValue>,     // 创建时间（通用值过滤）
    pub mid: Option<OpValsInt64>,       // 修改者ID
    pub mtime: Option<OpValsValue>,     // 修改时间
}
```

---

## 二、过滤操作符

项目使用 [modql](https://github.com/jeremychone/rust-modql) 库提供的过滤操作符。

### 2.1 OpValsInt64 / OpValsInt32 (整数类型)

| 操作符 | 说明 | JSON 示例 |
|--------|------|-----------|
| `$eq` | 等于 | `{"id": {"$eq": 123}}` 或简写 `{"id": 123}` |
| `$ne` | 不等于 | `{"id": {"$ne": 123}}` |
| `$gt` | 大于 | `{"id": {"$gt": 100}}` |
| `$gte` | 大于等于 | `{"id": {"$gte": 100}}` |
| `$lt` | 小于 | `{"id": {"$lt": 200}}` |
| `$lte` | 小于等于 | `{"id": {"$lte": 200}}` |
| `$in` | 在列表中 | `{"id": {"$in": [1, 2, 3]}}` |
| `$notIn` | 不在列表中 | `{"id": {"$notIn": [4, 5]}}` |
| `$null` | 是否为空 | `{"id": {"$null": true}}` |

**组合使用**：
```json
{"id": {"$gte": 10, "$lte": 100}}
```

### 2.2 OpValsString (字符串类型)

| 操作符 | 说明 | SQL 等效 | JSON 示例 |
|--------|------|----------|-----------|
| `$eq` | 精确匹配 | `= 'x'` | `{"name": {"$eq": "test"}}` 或 `{"name": "test"}` |
| `$ne` | 不等于 | `!= 'x'` | `{"name": {"$ne": "test"}}` |
| `$contains` | 包含 | `LIKE '%x%'` | `{"name": {"$contains": "test"}}` |
| `$notContains` | 不包含 | `NOT LIKE '%x%'` | `{"name": {"$notContains": "x"}}` |
| `$startsWith` | 以...开头 | `LIKE 'x%'` | `{"name": {"$startsWith": "pre"}}` |
| `$notStartsWith` | 不以...开头 | `NOT LIKE 'x%'` | `{"name": {"$notStartsWith": "x"}}` |
| `$endsWith` | 以...结尾 | `LIKE '%x'` | `{"name": {"$endsWith": "fix"}}` |
| `$notEndsWith` | 不以...结尾 | `NOT LIKE '%x'` | `{"name": {"$notEndsWith": "x"}}` |
| `$in` | 在列表中 | `IN (...)` | `{"name": {"$in": ["a", "b"]}}` |
| `$notIn` | 不在列表中 | `NOT IN (...)` | `{"name": {"$notIn": ["c"]}}` |
| `$null` | 是否为空 | `IS NULL` | `{"name": {"$null": false}}` |
| `$containsIn` | 包含任一 | 多个 LIKE OR | `{"name": {"$containsIn": ["a", "b"]}}` |

### 2.3 OpValsValue (通用类型)

用于日期时间等通用值过滤：

| 操作符 | 说明 | JSON 示例 |
|--------|------|-----------|
| `$eq` | 等于 | `{"ctime": {"$eq": "2024-01-01T00:00:00Z"}}` |
| `$gt` | 大于 | `{"ctime": {"$gt": "2024-01-01T00:00:00Z"}}` |
| `$gte` | 大于等于 | `{"ctime": {"$gte": "2024-01-01T00:00:00Z"}}` |
| `$lt` | 小于 | `{"ctime": {"$lt": "2024-12-31T23:59:59Z"}}` |
| `$lte` | 小于等于 | `{"ctime": {"$lte": "2024-12-31T23:59:59Z"}}` |

---

## 三、REST API 用法

### 3.1 端点说明

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/rest/{entity}` | 创建实体 |
| `GET` | `/api/rest/{entity}` | 列表查询（简单） |
| `GET` | `/api/rest/{entity}/paged` | 列表查询（带分页信息） |
| `GET` | `/api/rest/{entity}/{id}` | 获取单个实体 |
| `PUT` | `/api/rest/{entity}/{id}` | 更新实体 |
| `DELETE` | `/api/rest/{entity}/{id}` | 删除实体 |

### 3.2 查询参数

| 参数 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| `filters` | JSON | 过滤条件 | 无 |
| `page_size` | i64 | 每页数量 | 20 (最大1000) |
| `page_number` | i64 | 页码（从1开始） | 1 |
| `order_bys` | string | 排序字段 | 无 |

### 3.3 排序语法

- 升序：`order_bys=id` 或 `order_bys=ctime`
- 降序：`order_bys=-id` 或 `order_bys=-ctime`（使用 `-` 前缀）
- 多字段：`order_bys=-ctime,id`

### 3.4 过滤示例

#### 基础查询（无过滤）
```bash
GET /api/rest/agents
```

#### 带分页
```bash
GET /api/rest/agents?page_size=10&page_number=1
```

#### 带排序
```bash
# 按创建时间降序
GET /api/rest/agents?order_bys=-ctime

# 多字段排序
GET /api/rest/agents?order_bys=-ctime,id
```

#### 单字段过滤
```bash
# 精确匹配
GET /api/rest/agents?filters={"name":"test-agent"}

# 模糊搜索（包含）
GET /api/rest/agents?filters={"name":{"$contains":"test"}}

# ID 大于某值
GET /api/rest/agents?filters={"id":{"$gt":100}}
```

#### 多字段过滤（AND 关系）
```bash
# 名称包含 "test" 且 ID 大于 10
GET /api/rest/agents?filters={"name":{"$contains":"test"},"id":{"$gt":10}}
```

#### 多条件过滤（OR 关系）
```bash
# 名称是 "agent1" 或 "agent2"（使用数组）
GET /api/rest/agents?filters=[{"name":"agent1"},{"name":"agent2"}]

# 或者使用 $in 操作符
GET /api/rest/agents?filters={"name":{"$in":["agent1","agent2"]}}
```

#### 范围查询
```bash
# ID 在 10-100 之间
GET /api/rest/agents?filters={"id":{"$gte":10,"$lte":100}}

# 最近7天创建的
GET /api/rest/agents?filters={"ctime":{"$gte":"2024-12-20T00:00:00Z"}}
```

#### 组合查询
```bash
GET /api/rest/agents?filters={"name":{"$startsWith":"agent"}}&page_size=20&page_number=1&order_bys=-ctime
```

#### 分页响应端点
```bash
# 返回分页信息（total, total_pages, has_more）
GET /api/rest/agents/paged?page_size=10&page_number=1
```

响应格式：
```json
{
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

### 3.5 URL 格式说明

`filters` 参数直接接受 JSON 字符串，**无需特殊编码**（浏览器会自动处理）：

```bash
# 直接在浏览器中使用
http://localhost:8080/api/rest/agents?filters={"name":{"$contains":"test"}}

# curl 使用
curl "http://localhost:8080/api/rest/agents?filters={\"name\":{\"\$contains\":\"test\"}}"

# 或使用 --data-urlencode
curl -G "http://localhost:8080/api/rest/agents" \
  --data-urlencode 'filters={"name":{"$contains":"test"}}'
```

**支持的格式**：
- 单个 Filter 对象：`filters={"name":"test"}`
- 多个 Filter 数组：`filters=[{"name":"a"},{"name":"b"}]`
```

---

## 四、RPC API 用法

### 4.1 请求格式

```
POST /api/rpc
Content-Type: application/json
```

```json
{
    "id": 1,
    "method": "method_name",
    "params": { ... }
}
```

### 4.2 可用方法

| 方法 | 说明 | params 结构 |
|------|------|-------------|
| `create_{entity}` | 创建 | `{"data": {...}}` |
| `get_{entity}` | 获取单个 | `{"id": 123}` |
| `list_{entity}s` | 列表查询 | `{"filters": [...], "list_options": {...}}` |
| `update_{entity}` | 更新 | `{"id": 123, "data": {...}}` |
| `delete_{entity}` | 删除 | `{"id": 123}` |

### 4.3 list_options 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `limit` | i64 | 返回数量 |
| `offset` | i64 | 偏移量 |
| `order_bys` | string | 排序字段 |

**排序语法**：
- 升序：`"order_bys": "id"` 或 `"order_bys": "ctime"`
- 降序：`"order_bys": "!id"` 或 `"order_bys": "!ctime"`（使用 `!` 前缀）
- 多字段：`"order_bys": "!ctime,id"`

### 4.4 过滤示例

#### 基础列表查询
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {}
}
```

#### 带分页
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "list_options": {
            "limit": 10,
            "offset": 0
        }
    }
}
```

#### 带排序
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "list_options": {
            "limit": 10,
            "order_bys": "!ctime"
        }
    }
}
```

#### 单字段过滤
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "filters": [
            {"name": {"$contains": "test"}}
        ]
    }
}
```

#### 多字段过滤（AND 关系）
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "filters": [
            {
                "name": {"$contains": "test"},
                "id": {"$gt": 10}
            }
        ]
    }
}
```

#### 多条件过滤（OR 关系）
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "filters": [
            {"name": "agent1"},
            {"name": "agent2"}
        ]
    }
}
```

#### 范围查询
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "filters": [
            {"id": {"$gte": 10, "$lte": 100}}
        ]
    }
}
```

#### 日期范围查询
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "filters": [
            {"ctime": {"$gte": "2024-12-01T00:00:00Z", "$lt": "2025-01-01T00:00:00Z"}}
        ]
    }
}
```

#### 完整组合查询
```json
{
    "id": 1,
    "method": "list_agents",
    "params": {
        "filters": [
            {
                "name": {"$startsWith": "agent"},
                "ctime": {"$gte": "2024-01-01T00:00:00Z"}
            }
        ],
        "list_options": {
            "limit": 20,
            "offset": 0,
            "order_bys": "!ctime,id"
        }
    }
}
```

### 4.5 响应格式

成功响应：
```json
{
    "id": 1,
    "result": [
        {"id": 1, "name": "agent1", ...},
        {"id": 2, "name": "agent2", ...}
    ]
}
```

错误响应：
```json
{
    "id": 1,
    "error": {
        "message": "Entity not found",
        "data": {...}
    }
}
```

---

## 五、注意事项

### 5.1 过滤逻辑

- **同一 Filter 对象内多字段**：AND 关系
- **filters 数组中多个 Filter 对象**：OR 关系

```json
// AND: name 包含 "test" 且 id > 10
{"filters": [{"name": {"$contains": "test"}, "id": {"$gt": 10}}]}

// OR: name 是 "a" 或 name 是 "b"
{"filters": [{"name": "a"}, {"name": "b"}]}
```

### 5.2 排序差异

| API 类型 | 降序前缀 | 示例 |
|----------|----------|------|
| REST | `-` | `order_bys=-ctime` |
| RPC | `!` | `"order_bys": "!ctime"` |

### 5.3 分页差异

| API 类型 | 分页参数 |
|----------|----------|
| REST | `page_size`, `page_number`（从1开始） |
| RPC | `limit`, `offset`（从0开始） |

### 5.4 日期格式

使用 ISO 8601 / RFC 3339 格式：
```
2024-12-25T10:30:00Z
2024-12-25T18:30:00+08:00
```

### 5.5 空值处理

```json
// 查找字段为空的记录
{"name": {"$null": true}}

// 查找字段不为空的记录
{"name": {"$null": false}}
```

---

## 六、快速参考

### REST 常用示例

```bash
# 分页列表
GET /api/rest/agents?page_size=10&page_number=1

# 搜索 + 排序
GET /api/rest/agents?filters={"name":{"$contains":"test"}}&order_bys=-ctime

# 获取分页信息
GET /api/rest/agents/paged?page_size=10&page_number=1
```

### RPC 常用示例

```json
// 分页列表
{"method": "list_agents", "params": {"list_options": {"limit": 10, "offset": 0}}}

// 搜索 + 排序
{"method": "list_agents", "params": {"filters": [{"name": {"$contains": "test"}}], "list_options": {"order_bys": "!ctime"}}}
```
