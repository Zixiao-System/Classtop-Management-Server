# ClassTop API 文档

ClassTop 提供了完整的 RESTful API 接口，方便集中管理服务器进行远程管理和数据同步。

## 目录

- [快速开始](#快速开始)
- [启用 API 服务器](#启用-api-服务器)
- [身份验证](#身份验证)
- [API 端点](#api-端点)
  - [系统管理](#系统管理)
  - [课程管理](#课程管理)
  - [课程表管理](#课程表管理)
  - [设置管理](#设置管理)
  - [周次管理](#周次管理)
  - [统计信息](#统计信息)
  - [日志管理](#日志管理)
- [数据模型](#数据模型)
- [错误处理](#错误处理)
- [使用示例](#使用示例)

## 快速开始

### 启用 API 服务器

有两种方式启用 API 服务器：

#### 方式 1: 通过数据库直接设置（首次启用推荐）

```bash
# 使用 SQLite 命令行工具
sqlite3 <app_data_dir>/classtop.db

# 执行以下 SQL
UPDATE settings SET value='true' WHERE key='api_server_enabled';
UPDATE settings SET value='0.0.0.0' WHERE key='api_server_host';
UPDATE settings SET value='8765' WHERE key='api_server_port';
```

#### 方式 2: 通过前端界面（TODO: 未来版本）

在设置页面中启用 "API 服务器" 选项。

重启应用后，API 服务器将在配置的地址和端口上启动。

### 验证服务器状态

```bash
curl http://localhost:8765/api/health
```

成功响应：
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2025-10-07T10:30:00",
    "version": "1.0.0"
  }
}
```

### 访问 API 文档

启动 API 服务器后，可以通过以下 URL 访问交互式 API 文档：

- **Swagger UI**: `http://localhost:8765/api/docs`
- **ReDoc**: `http://localhost:8765/api/redoc`

## 身份验证

当前版本的 API **暂未实现身份验证机制**。

**安全建议**：
- 仅在受信任的网络环境中启用 API 服务器
- 使用防火墙限制访问
- 生产环境建议配合反向代理（如 Nginx）实现 HTTPS 和身份验证

未来版本将添加：
- API Key 认证
- JWT Token 认证
- IP 白名单

## API 端点

所有 API 响应遵循统一格式：

**成功响应**：
```json
{
  "success": true,
  "data": { ... }
}
```

**错误响应**：
```json
{
  "detail": "错误信息"
}
```

---

### 系统管理

#### 健康检查

**GET** `/api/health`

检查 API 服务器状态。

**响应示例**：
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2025-10-07T10:30:00",
    "version": "1.0.0"
  }
}
```

#### 根路径

**GET** `/`

获取 API 基本信息。

**响应示例**：
```json
{
  "message": "ClassTop API Server",
  "version": "1.0.0",
  "docs": "/api/docs"
}
```

---

### 课程管理

#### 获取所有课程

**GET** `/api/courses`

获取所有课程列表。

**响应示例**：
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "高等数学",
      "teacher": "张三",
      "location": "教学楼A101",
      "color": "#FF5722"
    },
    {
      "id": 2,
      "name": "大学英语",
      "teacher": "李四",
      "location": "教学楼B203",
      "color": "#4CAF50"
    }
  ]
}
```

#### 获取单个课程

**GET** `/api/courses/{course_id}`

获取指定 ID 的课程信息。

**路径参数**：
- `course_id` (integer): 课程 ID

**响应示例**：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "高等数学",
    "teacher": "张三",
    "location": "教学楼A101",
    "color": "#FF5722"
  }
}
```

**错误响应**：
- `404`: 课程不存在

#### 创建课程

**POST** `/api/courses`

创建新课程。

**请求体**：
```json
{
  "name": "高等数学",
  "teacher": "张三",
  "location": "教学楼A101",
  "color": "#FF5722"
}
```

**必填字段**：
- `name` (string): 课程名称

**可选字段**：
- `teacher` (string): 教师姓名
- `location` (string): 上课地点
- `color` (string): 课程颜色 (十六进制)

**响应示例**：
```json
{
  "success": true,
  "data": {
    "id": 3,
    "name": "高等数学",
    "teacher": "张三",
    "location": "教学楼A101",
    "color": "#FF5722"
  }
}
```

**错误响应**：
- `400`: 缺少必填字段
- `500`: 创建失败

#### 更新课程

**PUT** `/api/courses/{course_id}`

更新指定课程的信息。

**路径参数**：
- `course_id` (integer): 课程 ID

**请求体**：
```json
{
  "name": "高等数学（上）",
  "teacher": "王五",
  "location": "教学楼A102"
}
```

**可更新字段**：
- `name` (string): 课程名称
- `teacher` (string): 教师姓名
- `location` (string): 上课地点
- `color` (string): 课程颜色

**响应示例**：
```json
{
  "success": true,
  "message": "Course updated"
}
```

**错误响应**：
- `404`: 课程不存在
- `500`: 更新失败

#### 删除课程

**DELETE** `/api/courses/{course_id}`

删除指定课程及其所有相关的课程表条目。

**路径参数**：
- `course_id` (integer): 课程 ID

**响应示例**：
```json
{
  "success": true,
  "message": "Course deleted"
}
```

**错误响应**：
- `404`: 课程不存在
- `500`: 删除失败

---

### 课程表管理

#### 获取课程表

**GET** `/api/schedule`

获取课程表条目，可选择按周次过滤。

**查询参数**：
- `week` (integer, 可选): 周次（1-20）

**响应示例**：
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "course_id": 1,
      "course_name": "高等数学",
      "teacher": "张三",
      "location": "教学楼A101",
      "color": "#FF5722",
      "day_of_week": 1,
      "start_time": "08:00",
      "end_time": "09:40",
      "weeks": [1, 2, 3, 4, 5, 6, 7, 8],
      "note": null
    }
  ]
}
```

#### 添加课程表条目

**POST** `/api/schedule`

添加新的课程表条目。

**请求体**：
```json
{
  "course_id": 1,
  "day_of_week": 1,
  "start_time": "08:00",
  "end_time": "09:40",
  "weeks": [1, 2, 3, 4, 5, 6, 7, 8],
  "note": "第一节课"
}
```

**必填字段**：
- `course_id` (integer): 课程 ID
- `day_of_week` (integer): 星期几（1=周一, 7=周日）
- `start_time` (string): 开始时间 (HH:MM 格式)
- `end_time` (string): 结束时间 (HH:MM 格式)

**可选字段**：
- `weeks` (array of integers): 适用的周次列表
- `note` (string): 备注

**响应示例**：
```json
{
  "success": true,
  "data": {
    "id": 10
  }
}
```

**错误响应**：
- `400`: 缺少必填字段或格式错误
- `500`: 创建失败

#### 获取某天的课程表

**GET** `/api/schedule/day/{day_of_week}`

获取指定星期的课程表。

**路径参数**：
- `day_of_week` (integer): 星期几（1=周一, 7=周日）

**查询参数**：
- `week` (integer, 可选): 周次

**响应示例**：
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "高等数学",
      "teacher": "张三",
      "location": "教学楼A101",
      "day_of_week": 1,
      "start_time": "08:00",
      "end_time": "09:40",
      "weeks": [1, 2, 3, 4, 5],
      "color": "#FF5722"
    }
  ]
}
```

**错误响应**：
- `400`: day_of_week 必须在 1-7 之间

#### 获取整周课程表

**GET** `/api/schedule/week`

获取整周的课程表（周一到周日）。

**查询参数**：
- `week` (integer, 可选): 周次

**响应示例**：
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "高等数学",
      "teacher": "张三",
      "location": "教学楼A101",
      "day_of_week": 1,
      "start_time": "08:00",
      "end_time": "09:40",
      "weeks": [1, 2, 3],
      "color": "#FF5722"
    },
    {
      "id": 2,
      "name": "大学英语",
      "teacher": "李四",
      "location": "教学楼B203",
      "day_of_week": 2,
      "start_time": "10:00",
      "end_time": "11:40",
      "weeks": [1, 2, 3],
      "color": "#4CAF50"
    }
  ]
}
```

#### 删除课程表条目

**DELETE** `/api/schedule/{entry_id}`

删除指定的课程表条目。

**路径参数**：
- `entry_id` (integer): 课程表条目 ID

**响应示例**：
```json
{
  "success": true,
  "message": "Schedule entry deleted"
}
```

**错误响应**：
- `404`: 课程表条目不存在
- `500`: 删除失败

---

### 设置管理

#### 获取所有设置

**GET** `/api/settings`

获取所有应用设置。

**响应示例**：
```json
{
  "success": true,
  "data": {
    "client_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "server_url": "",
    "api_server_enabled": "true",
    "api_server_host": "0.0.0.0",
    "api_server_port": "8765",
    "theme_mode": "auto",
    "theme_color": "#6750A4",
    "show_clock": "true",
    "show_schedule": "true",
    "semester_start_date": "2025-09-01"
  }
}
```

#### 获取单个设置

**GET** `/api/settings/{key}`

获取指定键的设置值。

**路径参数**：
- `key` (string): 设置键名

**响应示例**：
```json
{
  "success": true,
  "data": {
    "key": "theme_mode",
    "value": "auto"
  }
}
```

**错误响应**：
- `404`: 设置不存在

#### 批量更新设置

**PUT** `/api/settings`

批量更新多个设置。

**请求体**：
```json
{
  "theme_mode": "dark",
  "show_clock": "false",
  "semester_start_date": "2025-09-01"
}
```

**响应示例**：
```json
{
  "success": true,
  "message": "Settings updated"
}
```

**错误响应**：
- `500`: 更新失败

#### 更新单个设置

**PUT** `/api/settings/{key}`

更新指定键的设置值。

**路径参数**：
- `key` (string): 设置键名

**请求体**：
```json
{
  "value": "dark"
}
```

**响应示例**：
```json
{
  "success": true,
  "message": "Setting updated"
}
```

**错误响应**：
- `400`: 缺少 value 字段
- `500`: 更新失败

#### 重置设置为默认值

**POST** `/api/settings/reset`

重置所有设置为默认值，可选择排除某些设置。

**请求体（可选）**：
```json
{
  "exclude": ["client_uuid", "semester_start_date"]
}
```

**响应示例**：
```json
{
  "success": true,
  "message": "Settings reset to defaults"
}
```

**错误响应**：
- `500`: 重置失败

---

### 周次管理

#### 获取当前周次

**GET** `/api/week/current`

获取当前周次信息。

**响应示例**：
```json
{
  "success": true,
  "data": {
    "week": 5,
    "semester_start_date": "2025-09-01",
    "is_calculated": true
  }
}
```

**字段说明**：
- `week`: 当前周次
- `semester_start_date`: 学期开始日期（空字符串表示未设置）
- `is_calculated`: 是否通过学期开始日期自动计算（false 表示使用手动设置的周次）

#### 设置学期开始日期

**POST** `/api/week/semester-start`

设置学期开始日期，系统将自动计算当前周次。

**请求体**：
```json
{
  "date": "2025-09-01"
}
```

**日期格式**: `YYYY-MM-DD`（传入空字符串清除自动计算）

**响应示例**：
```json
{
  "success": true,
  "data": {
    "semester_start_date": "2025-09-01",
    "calculated_week": 5
  }
}
```

**错误响应**：
- `500`: 设置失败

---

### 统计信息

#### 获取课程表统计

**GET** `/api/statistics`

获取课程表的统计信息。

**响应示例**：
```json
{
  "success": true,
  "data": {
    "total_courses": 8,
    "total_schedule_entries": 24,
    "busiest_day": 3
  }
}
```

**字段说明**：
- `total_courses`: 总课程数
- `total_schedule_entries`: 总课程表条目数
- `busiest_day`: 课程最多的星期几（1=周一, 7=周日）

---

### 日志管理

#### 获取应用日志

**GET** `/api/logs`

获取应用日志的最后 N 行。

**查询参数**：
- `max_lines` (integer, 可选, 默认: 200): 最大行数

**响应示例**：
```json
{
  "success": true,
  "data": {
    "lines": [
      "[2025-10-07 10:30:00] INFO: Application started",
      "[2025-10-07 10:30:01] INFO: Database initialized",
      "[2025-10-07 10:30:02] INFO: API server started on 0.0.0.0:8765"
    ]
  }
}
```

---

## 数据模型

### Course（课程）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | integer | 是 | 课程 ID（自动生成） |
| name | string | 是 | 课程名称 |
| teacher | string | 否 | 教师姓名 |
| location | string | 否 | 上课地点 |
| color | string | 否 | 课程颜色（十六进制，如 #FF5722） |

### ScheduleEntry（课程表条目）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | integer | 是 | 条目 ID（自动生成） |
| course_id | integer | 是 | 关联的课程 ID |
| course_name | string | - | 课程名称（查询时返回） |
| teacher | string | - | 教师姓名（查询时返回） |
| location | string | - | 上课地点（查询时返回） |
| color | string | - | 课程颜色（查询时返回） |
| day_of_week | integer | 是 | 星期几（1=周一, 7=周日） |
| start_time | string | 是 | 开始时间（HH:MM 格式） |
| end_time | string | 是 | 结束时间（HH:MM 格式） |
| weeks | array | 否 | 适用的周次列表（如 [1,2,3,4]） |
| note | string | 否 | 备注信息 |

### Settings（设置）

| 键名 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| client_uuid | string | 自动生成 | 客户端唯一标识 |
| server_url | string | "" | 服务器地址 |
| api_server_enabled | string | "false" | 是否启用 API 服务器 |
| api_server_host | string | "0.0.0.0" | API 服务器监听地址 |
| api_server_port | string | "8765" | API 服务器端口 |
| theme_mode | string | "auto" | 主题模式（auto/dark/light） |
| theme_color | string | "#6750A4" | 主题颜色 |
| show_clock | string | "true" | 是否显示时钟 |
| show_schedule | string | "true" | 是否显示课程表 |
| semester_start_date | string | "" | 学期开始日期（YYYY-MM-DD） |

---

## 错误处理

所有错误响应遵循 FastAPI 标准格式：

```json
{
  "detail": "错误描述信息"
}
```

### 常见 HTTP 状态码

| 状态码 | 说明 |
|--------|------|
| 200 | 请求成功 |
| 400 | 请求参数错误 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

---

## 使用示例

### Python 示例

```python
import requests

BASE_URL = "http://localhost:8765"

# 获取所有课程
response = requests.get(f"{BASE_URL}/api/courses")
courses = response.json()["data"]
print(f"共有 {len(courses)} 门课程")

# 创建新课程
new_course = {
    "name": "计算机网络",
    "teacher": "王教授",
    "location": "实验楼301",
    "color": "#2196F3"
}
response = requests.post(f"{BASE_URL}/api/courses", json=new_course)
course_id = response.json()["data"]["id"]
print(f"创建课程成功，ID: {course_id}")

# 添加课程表
schedule_entry = {
    "course_id": course_id,
    "day_of_week": 3,  # 周三
    "start_time": "14:00",
    "end_time": "15:40",
    "weeks": [1, 2, 3, 4, 5, 6, 7, 8]
}
response = requests.post(f"{BASE_URL}/api/schedule", json=schedule_entry)
entry_id = response.json()["data"]["id"]
print(f"添加课程表成功，ID: {entry_id}")

# 获取本周课程表
response = requests.get(f"{BASE_URL}/api/week/current")
current_week = response.json()["data"]["week"]
response = requests.get(f"{BASE_URL}/api/schedule/week", params={"week": current_week})
weekly_schedule = response.json()["data"]
print(f"本周（第 {current_week} 周）共有 {len(weekly_schedule)} 节课")
```

### JavaScript 示例

```javascript
const BASE_URL = "http://localhost:8765";

// 获取所有课程
async function getCourses() {
  const response = await fetch(`${BASE_URL}/api/courses`);
  const result = await response.json();
  console.log(`共有 ${result.data.length} 门课程`);
  return result.data;
}

// 创建新课程
async function createCourse(courseData) {
  const response = await fetch(`${BASE_URL}/api/courses`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(courseData),
  });
  const result = await response.json();
  console.log(`创建课程成功，ID: ${result.data.id}`);
  return result.data;
}

// 获取本周课程表
async function getWeeklySchedule() {
  // 先获取当前周次
  const weekResponse = await fetch(`${BASE_URL}/api/week/current`);
  const weekData = await weekResponse.json();
  const currentWeek = weekData.data.week;

  // 获取课程表
  const scheduleResponse = await fetch(
    `${BASE_URL}/api/schedule/week?week=${currentWeek}`
  );
  const scheduleData = await scheduleResponse.json();
  console.log(`本周（第 ${currentWeek} 周）共有 ${scheduleData.data.length} 节课`);
  return scheduleData.data;
}

// 使用示例
getCourses();
createCourse({
  name: "数据结构",
  teacher: "李教授",
  location: "教学楼C301",
  color: "#9C27B0",
});
getWeeklySchedule();
```

### cURL 示例

```bash
# 健康检查
curl http://localhost:8765/api/health

# 获取所有课程
curl http://localhost:8765/api/courses

# 创建新课程
curl -X POST http://localhost:8765/api/courses \
  -H "Content-Type: application/json" \
  -d '{
    "name": "操作系统",
    "teacher": "赵教授",
    "location": "教学楼D201",
    "color": "#FF9800"
  }'

# 获取当前周次
curl http://localhost:8765/api/week/current

# 获取本周课程表（假设当前是第5周）
curl "http://localhost:8765/api/schedule/week?week=5"

# 更新设置
curl -X PUT http://localhost:8765/api/settings/theme_mode \
  -H "Content-Type: application/json" \
  -d '{"value": "dark"}'

# 获取日志
curl "http://localhost:8765/api/logs?max_lines=50"
```

---

## 技术细节

### 依赖项

API 服务器基于以下技术栈：

- **FastAPI**: 现代高性能 Web 框架
- **Uvicorn**: ASGI 服务器
- **Pydantic**: 数据验证

### 安装依赖

如果您需要手动安装依赖：

```bash
pip install fastapi uvicorn pydantic
```

### CORS 支持

API 服务器默认启用 CORS，允许所有来源访问。生产环境建议修改 `api_server.py` 中的 CORS 配置：

```python
app.add_middleware(
    CORSMiddleware,
    allow_origins=["https://yourdomain.com"],  # 限制特定域名
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
```

### 性能考虑

- API 服务器运行在独立的守护线程中，不会阻塞主应用
- 使用连接池管理 SQLite 数据库连接
- 建议在生产环境中使用反向代理（Nginx、Caddy）
- 对于高并发场景，考虑使用 Redis 缓存

---

## 常见问题

### Q: 如何更改 API 服务器端口？

修改数据库中的 `api_server_port` 设置：

```sql
UPDATE settings SET value='9000' WHERE key='api_server_port';
```

然后重启应用。

### Q: API 服务器支持 HTTPS 吗？

当前版本不直接支持 HTTPS。建议使用 Nginx 或 Caddy 作为反向代理来实现 HTTPS。

### Q: 如何禁用 API 服务器？

修改数据库中的 `api_server_enabled` 设置：

```sql
UPDATE settings SET value='false' WHERE key='api_server_enabled';
```

然后重启应用。

### Q: 可以远程访问 API 吗？

可以。默认配置 `api_server_host` 为 `0.0.0.0`，允许所有网络接口访问。如果只想本地访问，可以改为 `127.0.0.1`。

---

## 更新日志

### v1.0.0 (2025-10-07)

- 初始版本
- 支持课程、课程表、设置的完整 CRUD 操作
- 提供交互式 API 文档（Swagger/ReDoc）
- 支持周次自动计算
- 提供统计信息和日志查询接口

---

## 许可证

本 API 遵循与 ClassTop 主项目相同的许可证。

---

## 支持

如有问题或建议，请提交 Issue 到项目仓库。
