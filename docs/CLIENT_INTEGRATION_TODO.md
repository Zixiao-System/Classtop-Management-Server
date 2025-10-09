# ClassTop 客户端集成任务清单

本文档列出了将 ClassTop 客户端与集中管理服务器集成所需的所有任务。

**客户端项目路径**: `/Users/logos/fleet/classtop`
**服务端项目路径**: `/Users/logos/RustroverProjects/Classtop-Management-Server`

---

## 📋 任务概览

- **阶段 1**: 数据库 Schema 更新
- **阶段 2**: 后端集成开发
- **阶段 3**: 前端界面开发
- **阶段 4**: 测试与验证
- **阶段 5**: 文档和部署

---

## 阶段 1: 数据库 Schema 更新

### 1.1 添加 `location` 字段到 courses 表

**文件**: `/Users/logos/fleet/classtop/src-tauri/python/tauri_app/db.py`

- [ ] 在 `Database` 类中添加 `migrate_database()` 方法
- [ ] 检查 `location` 字段是否已存在
- [ ] 如果不存在，执行 `ALTER TABLE courses ADD COLUMN location TEXT`
- [ ] 在 `__init__()` 或初始化函数中调用迁移方法

**参考代码**:
```python
def migrate_database(self):
    """数据库迁移：添加新字段"""
    cursor = self.conn.cursor()

    # 检查 location 字段是否存在
    cursor.execute("PRAGMA table_info(courses)")
    columns = [column[1] for column in cursor.fetchall()]

    if 'location' not in columns:
        cursor.execute("ALTER TABLE courses ADD COLUMN location TEXT")
        self.conn.commit()
        print("✓ 添加 location 字段到 courses 表")
```

**验证**:
```bash
sqlite3 ~/.local/share/classtop/classtop.db
.schema courses
# 应该看到 location TEXT 字段
```

---

### 1.2 确保设置表包含同步相关字段

**文件**: `/Users/logos/fleet/classtop/src-tauri/python/tauri_app/settings_manager.py`

- [ ] 检查 `DEFAULT_SETTINGS` 字典
- [ ] 确保包含以下键值对：
  - `client_uuid`: 自动生成的 UUID
  - `client_name`: 默认为主机名
  - `server_url`: 服务器地址（空字符串）
  - `sync_enabled`: "false"
  - `sync_interval`: "300"（秒）

**参考代码**:
```python
import uuid
import socket

DEFAULT_SETTINGS = {
    # ... 现有设置 ...

    # 服务器同步相关
    "client_uuid": str(uuid.uuid4()),
    "client_name": socket.gethostname(),
    "server_url": "",
    "sync_enabled": "false",
    "sync_interval": "300",
}
```

**验证**:
```bash
sqlite3 ~/.local/share/classtop/classtop.db
SELECT key, value FROM settings WHERE key LIKE '%sync%' OR key LIKE '%server%' OR key LIKE '%client%';
```

---

## 阶段 2: 后端集成开发

### 2.1 创建同步客户端模块

**文件**: `/Users/logos/fleet/classtop/src-tauri/python/tauri_app/sync_client.py`（新建）

- [ ] 创建 `SyncClient` 类
- [ ] 实现 `__init__(settings_manager, schedule_manager, logger)` 构造函数
- [ ] 实现 `register_client()` 方法 - 向服务器注册客户端
- [ ] 实现 `sync_to_server()` 方法 - 同步数据到服务器
- [ ] 实现 `test_connection()` 方法 - 测试服务器连接
- [ ] 实现 `start_auto_sync()` 方法 - 启动自动同步线程
- [ ] 实现 `stop_auto_sync()` 方法 - 停止自动同步
- [ ] 实现 `_sync_loop()` 私有方法 - 同步循环逻辑

**依赖安装**:
```bash
cd /Users/logos/fleet/classtop
pip install requests
```

**核心功能**:
1. 使用 `requests` 库发送 HTTP 请求
2. 后台 daemon 线程定期同步
3. 异常处理和日志记录
4. 支持配置化的同步间隔

**参考**: 查看 `docs/CLIENT_ADAPTATION.md` 第 2 节完整代码示例

---

### 2.2 扩展 schedule_manager.py

**文件**: `/Users/logos/fleet/classtop/src-tauri/python/tauri_app/schedule_manager.py`

- [ ] 添加 `get_all_courses()` 方法
  - 返回所有课程列表（包含 id, name, teacher, location, color, note）
  - 用于同步到服务器

- [ ] 添加 `get_all_schedule_entries()` 方法
  - 返回所有课程表条目列表（JOIN courses 表）
  - 包含完整的课程信息
  - 用于同步到服务器

**参考代码**:
```python
def get_all_courses(self) -> List[Dict]:
    """获取所有课程（用于同步）"""
    cursor = self.conn.cursor()
    cursor.execute("""
        SELECT id, name, teacher, location, color, note
        FROM courses
        ORDER BY id
    """)

    courses = []
    for row in cursor.fetchall():
        courses.append({
            "id": row[0],
            "name": row[1],
            "teacher": row[2] or "",
            "location": row[3] or "",
            "color": row[4] or "#6750A4",
            "note": row[5] or ""
        })

    return courses

def get_all_schedule_entries(self) -> List[Dict]:
    """获取所有课程表条目（用于同步）"""
    cursor = self.conn.cursor()
    cursor.execute("""
        SELECT s.id, s.course_id, s.day_of_week, s.start_time, s.end_time, s.weeks,
               c.name, c.teacher, c.location, c.color
        FROM schedule s
        JOIN courses c ON s.course_id = c.id
        ORDER BY s.day_of_week, s.start_time
    """)

    entries = []
    for row in cursor.fetchall():
        entries.append({
            "id": row[0],
            "course_id": row[1],
            "day_of_week": row[2],
            "start_time": row[3],
            "end_time": row[4],
            "weeks": row[5],
            "course_name": row[6],
            "teacher": row[7] or "",
            "location": row[8] or "",
            "color": row[9] or "#6750A4"
        })

    return entries
```

**验证**:
```python
# 在 Python 控制台测试
from tauri_app.schedule_manager import ScheduleManager
manager = ScheduleManager(conn)
courses = manager.get_all_courses()
print(f"共有 {len(courses)} 门课程")
```

---

### 2.3 集成到应用初始化流程

**文件**: `/Users/logos/fleet/classtop/src-tauri/python/tauri_app/__init__.py`

- [ ] 导入 `SyncClient` 类
- [ ] 在全局变量中添加 `sync_client = None`
- [ ] 在 `init()` 函数中初始化 `sync_client`
- [ ] 检查 `sync_enabled` 设置，如果启用则：
  - 调用 `sync_client.register_client()`
  - 调用 `sync_client.start_auto_sync()`

- [ ] 添加 Tauri 命令函数：
  - `test_server_connection()` - 测试服务器连接
  - `sync_now()` - 立即同步
  - `register_to_server()` - 注册到服务器

**参考代码**:
```python
from .sync_client import SyncClient

# 全局变量
sync_client = None

def init():
    """应用初始化"""
    global logger, db_manager, settings_manager, schedule_manager, sync_client

    # ... 现有初始化代码 ...

    # 初始化同步客户端
    sync_client = SyncClient(settings_manager, schedule_manager, logger)

    # 启动时尝试注册并启动自动同步
    sync_enabled = settings_manager.get_setting("sync_enabled", "false")
    if sync_enabled.lower() == "true":
        sync_client.register_client()
        sync_client.start_auto_sync()

    logger.log_message("info", "应用初始化完成")

@export_pyfunction(run_async=True)
def test_server_connection():
    """测试服务器连接"""
    if sync_client:
        return sync_client.test_connection()
    return {"success": False, "message": "同步客户端未初始化"}

@export_pyfunction(run_async=True)
def sync_now():
    """立即同步到服务器"""
    if sync_client:
        success = sync_client.sync_to_server()
        return {"success": success}
    return {"success": False, "message": "同步客户端未初始化"}

@export_pyfunction(run_async=True)
def register_to_server():
    """注册到服务器"""
    if sync_client:
        success = sync_client.register_client()
        return {"success": success}
    return {"success": False, "message": "同步客户端未初始化"}
```

**验证**:
- 启动应用，检查日志是否显示 "同步客户端未初始化" 或 "启动自动同步线程"
- 如果启用了同步，应该看到注册和同步相关日志

---

### 2.4 更新 Tauri 权限配置

**文件**: `/Users/logos/fleet/classtop/src-tauri/capabilities/default.json`

- [ ] 确保新增的 Tauri 命令有权限配置
- [ ] 添加以下命令到 `permissions` 数组：
  - `test_server_connection`
  - `sync_now`
  - `register_to_server`

**参考代码**:
```json
{
  "permissions": [
    "python:default",
    "python:allow-call",
    {
      "identifier": "python:allow-call",
      "allow": [
        // ... 现有命令 ...
        {
          "function": "test_server_connection"
        },
        {
          "function": "sync_now"
        },
        {
          "function": "register_to_server"
        }
      ]
    }
  ]
}
```

---

## 阶段 3: 前端界面开发

### 3.1 更新设置页面 UI

**文件**: `/Users/logos/fleet/classtop/src/pages/Settings.vue`

- [ ] 在 `<template>` 中添加 "服务器同步" 卡片
- [ ] 添加以下表单字段：
  - 服务器地址输入框 (`mdui-text-field`)
  - 客户端名称输入框 (`mdui-text-field`)
  - 启用自动同步开关 (`mdui-switch`)
  - 同步间隔输入框 (`mdui-text-field`, type="number")

- [ ] 添加按钮：
  - "测试连接" 按钮
  - "注册到服务器" 按钮
  - "立即同步" 按钮

- [ ] 添加同步状态显示区域
  - 显示成功/失败图标
  - 显示状态消息

**UI 布局建议**:
```vue
<mdui-card variant="outlined" class="setting-card">
  <div class="card-header">
    <mdui-icon name="cloud_sync"></mdui-icon>
    <h3>服务器同步</h3>
  </div>

  <div class="card-content">
    <!-- 输入框 -->
    <mdui-text-field
      v-model="serverUrl"
      label="服务器地址"
      placeholder="http://192.168.1.10:8765"
    ></mdui-text-field>

    <!-- 开关 -->
    <mdui-switch v-model="syncEnabled">启用自动同步</mdui-switch>

    <!-- 按钮组 -->
    <div class="button-group">
      <mdui-button @click="testConnection">测试连接</mdui-button>
      <mdui-button @click="syncNow">立即同步</mdui-button>
    </div>

    <!-- 状态显示 -->
    <div v-if="syncStatus" class="sync-status">
      <mdui-icon :name="syncStatus.success ? 'check_circle' : 'error'"></mdui-icon>
      <span>{{ syncStatus.message }}</span>
    </div>
  </div>
</mdui-card>
```

**参考**: 查看 `docs/CLIENT_ADAPTATION.md` 第 9 节完整代码

---

### 3.2 实现设置页面逻辑

**文件**: `/Users/logos/fleet/classtop/src/pages/Settings.vue` (script 部分)

- [ ] 定义响应式变量：
  - `serverUrl`, `clientName`, `syncEnabled`, `syncInterval`
  - `syncStatus` (用于显示同步结果)

- [ ] 实现 `onMounted` 钩子：
  - 调用 `get_all_settings` 加载现有配置
  - 填充表单字段

- [ ] 实现 `saveSettings()` 方法：
  - 调用 `set_setting` 保存所有配置

- [ ] 实现 `testConnection()` 方法：
  - 保存设置
  - 调用 `test_server_connection` Tauri 命令
  - 显示结果

- [ ] 实现 `registerClient()` 方法：
  - 保存设置
  - 调用 `register_to_server` Tauri 命令
  - 显示结果

- [ ] 实现 `syncNow()` 方法：
  - 保存设置
  - 调用 `sync_now` Tauri 命令
  - 显示结果

**参考代码**:
```javascript
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const serverUrl = ref('');
const syncEnabled = ref(false);
const syncStatus = ref(null);

onMounted(async () => {
  const settings = await invoke('get_all_settings');
  serverUrl.value = settings.server_url || '';
  syncEnabled.value = settings.sync_enabled === 'true';
});

const testConnection = async () => {
  await saveSettings();
  syncStatus.value = { success: false, message: '正在测试...' };

  const result = await invoke('test_server_connection');
  syncStatus.value = result;
};

const syncNow = async () => {
  await saveSettings();
  syncStatus.value = { success: false, message: '正在同步...' };

  const result = await invoke('sync_now');
  syncStatus.value = result.success
    ? { success: true, message: '同步成功' }
    : { success: false, message: '同步失败' };
};
```

---

### 3.3 添加同步状态指示器（可选）

**文件**: `/Users/logos/fleet/classtop/src/TopBar/TopBar.vue` 或 `/Users/logos/fleet/classtop/src/Main.vue`

- [ ] 在合适位置添加同步状态图标
- [ ] 监听同步事件（如果实现了事件系统）
- [ ] 显示状态：
  - 🟢 已同步
  - 🟡 同步中
  - 🔴 同步失败
  - ⚪ 未启用

**参考代码**:
```vue
<div class="sync-indicator">
  <mdui-icon
    :name="syncIcon"
    :style="{ color: syncColor }"
    :title="syncTooltip"
  ></mdui-icon>
</div>
```

---

## 阶段 4: 测试与验证

### 4.1 单元测试

- [ ] 测试数据库迁移
  - 运行应用，检查 courses 表是否有 location 字段
  - 检查 settings 表是否有同步相关设置

- [ ] 测试数据获取方法
  - 调用 `get_all_courses()` 验证返回格式
  - 调用 `get_all_schedule_entries()` 验证返回格式
  - 确保 weeks 字段是 JSON 字符串

---

### 4.2 集成测试

#### 准备工作

- [ ] 启动服务端
  ```bash
  cd /Users/logos/RustroverProjects/Classtop-Management-Server
  cargo run --release
  ```

- [ ] 确认服务端可访问
  ```bash
  curl http://localhost:8765/api/health
  ```

#### 测试流程

- [ ] **测试连接**
  1. 启动客户端
  2. 进入设置页面
  3. 输入服务器地址：`http://localhost:8765`
  4. 点击"测试连接"
  5. ✓ 应显示"连接成功"

- [ ] **客户端注册**
  1. 点击"注册到服务器"
  2. ✓ 应显示"注册成功"
  3. 访问服务器管理界面 http://localhost:8765
  4. ✓ 应在"客户端"页面看到新注册的客户端

- [ ] **数据同步**
  1. 在客户端添加测试数据：
     - 课程：高等数学、大学英语
     - 课程表：周一 08:00-09:40 高等数学
  2. 点击"立即同步"
  3. ✓ 应显示"同步成功"
  4. 访问服务器"数据查看"页面
  5. ✓ 应看到同步的课程和课程表数据

- [ ] **自动同步**
  1. 启用"自动同步"开关
  2. 设置同步间隔为 60 秒
  3. 保存设置
  4. 修改一门课程的信息
  5. 等待 60 秒
  6. ✓ 检查服务器，数据应自动更新

- [ ] **错误处理**
  1. 关闭服务端
  2. 点击"立即同步"
  3. ✓ 应显示连接错误，但客户端不崩溃
  4. 重启服务端
  5. 再次同步
  6. ✓ 应恢复正常

---

### 4.3 多客户端测试

- [ ] 在不同机器或不同目录运行多个客户端实例
- [ ] 每个客户端配置不同的名称
- [ ] 所有客户端连接到同一服务器
- [ ] 验证服务器可以区分不同客户端
- [ ] 验证每个客户端的数据独立存储

**注意**: 如果在同一台机器测试多个实例，需要修改客户端使用不同的数据库路径

---

### 4.4 性能测试

- [ ] 测试大量数据同步（100+ 课程，500+ 课程表条目）
- [ ] 记录同步耗时
- [ ] 验证不会阻塞 UI
- [ ] 检查内存使用情况

**性能基准**:
- 100 门课程 + 500 个课程表条目应在 5 秒内完成同步
- UI 应保持响应

---

## 阶段 5: 文档和部署

### 5.1 更新客户端文档

**文件**: `/Users/logos/fleet/classtop/README.md`

- [ ] 添加"集中管理服务器集成"章节
- [ ] 说明如何配置服务器地址
- [ ] 说明如何启用同步功能
- [ ] 添加故障排查指南

---

### 5.2 添加 CHANGELOG

**文件**: `/Users/logos/fleet/classtop/CHANGELOG.md`（如果存在）或 README.md

- [ ] 记录新功能：
  - 添加服务器同步功能
  - 支持集中管理
  - 自动数据同步

**示例**:
```markdown
## [0.2.0] - 2025-10-09

### Added
- 集中管理服务器集成功能
- 自动数据同步（课程和课程表）
- 客户端注册和状态上报
- 服务器连接测试工具
- courses 表添加 location 字段

### Changed
- 设置页面添加服务器配置部分
```

---

### 5.3 创建部署指南

**文件**: `/Users/logos/fleet/classtop/docs/DEPLOYMENT.md`（新建）

- [ ] 说明如何部署服务端
- [ ] 说明如何批量配置客户端
- [ ] 提供配置模板
- [ ] 添加网络配置建议

---

### 5.4 准备发布

- [ ] 更新版本号（package.json, Cargo.toml）
- [ ] 运行完整测试套件
- [ ] 构建发布版本
  ```bash
  cd /Users/logos/fleet/classtop
  npm run tauri build
  ```
- [ ] 测试发布版本
- [ ] 创建 Git tag
- [ ] 推送到远程仓库

---

## 依赖项清单

### Python 依赖

```bash
pip install requests
```

或在 `requirements.txt` 中添加：
```
requests>=2.31.0
```

---

## 验证清单

在完成所有任务后，使用以下清单验证集成是否成功：

### 基础功能
- [ ] 客户端可以成功注册到服务器
- [ ] 课程数据正确同步
- [ ] 课程表数据正确同步
- [ ] location 字段正确同步
- [ ] weeks 字段（JSON 数组）正确解析

### 同步功能
- [ ] 手动同步工作正常
- [ ] 自动同步按配置间隔执行
- [ ] 同步失败时不影响客户端正常使用
- [ ] 同步成功后服务器数据与客户端一致

### UI 交互
- [ ] 设置页面所有输入框正常工作
- [ ] 测试连接按钮返回正确状态
- [ ] 立即同步按钮触发同步
- [ ] 状态指示器显示正确（如果实现）

### 错误处理
- [ ] 网络断开时客户端不崩溃
- [ ] 服务器不可用时显示友好错误
- [ ] 无效配置时有明确提示
- [ ] 同步失败记录详细日志

### 多客户端
- [ ] 多个客户端可以同时连接
- [ ] 每个客户端数据独立
- [ ] 服务器正确区分不同客户端
- [ ] UUID 冲突检测（如果实现）

### 性能
- [ ] 同步不阻塞 UI
- [ ] 大数据量同步性能可接受
- [ ] 内存使用正常
- [ ] 后台线程正确管理

---

## 常见问题和解决方案

### Q1: 无法导入 `requests` 模块

**解决方案**:
```bash
# 确保在正确的 Python 环境中安装
which python3
pip3 install requests

# 或者使用项目虚拟环境
cd /Users/logos/fleet/classtop
python3 -m venv venv
source venv/bin/activate
pip install requests
```

### Q2: Tauri 命令调用失败

**解决方案**:
1. 检查 `capabilities/default.json` 是否包含命令权限
2. 确认函数使用了 `@export_pyfunction` 装饰器
3. 重新构建应用
4. 检查 Tauri 控制台的错误信息

### Q3: 数据库迁移未执行

**解决方案**:
1. 确认 `migrate_database()` 在初始化时被调用
2. 检查日志输出
3. 手动执行 SQL：
   ```bash
   sqlite3 ~/.local/share/classtop/classtop.db
   ALTER TABLE courses ADD COLUMN location TEXT;
   ```

### Q4: 同步数据格式错误

**解决方案**:
1. 检查 `weeks` 字段是否为 JSON 字符串
2. 使用 `json.loads()` 解析前验证格式
3. 添加数据验证和错误处理：
   ```python
   try:
       weeks = json.loads(entry["weeks"]) if entry.get("weeks") else []
   except json.JSONDecodeError:
       weeks = []
   ```

### Q5: 服务器返回 400/500 错误

**解决方案**:
1. 检查请求体格式是否符合服务端 API 规范
2. 查看服务端日志获取详细错误
3. 使用 curl 测试 API 端点：
   ```bash
   curl -X POST http://localhost:8765/api/sync \
     -H "Content-Type: application/json" \
     -d '{"client_uuid":"test","courses":[],"schedule_entries":[]}'
   ```

---

## 进度跟踪

**开始日期**: 2025-10-09
**预计完成**: _待填写_

### 里程碑

- [ ] **M1**: 数据库 Schema 更新完成 (预计 1 天)
- [ ] **M2**: 后端集成开发完成 (预计 2-3 天)
- [ ] **M3**: 前端界面开发完成 (预计 1-2 天)
- [ ] **M4**: 测试与验证完成 (预计 1-2 天)
- [ ] **M5**: 文档和部署完成 (预计 1 天)

**总计**: 预计 6-9 天完成

---

## 团队分工建议

- **后端开发** (阶段 1, 2): 熟悉 Python 和数据库的开发者
- **前端开发** (阶段 3): 熟悉 Vue 3 和 MDUI 的开发者
- **测试** (阶段 4): 所有团队成员
- **文档** (阶段 5): 项目负责人

---

## 相关资源

- **客户端适配指南**: `docs/CLIENT_ADAPTATION.md`
- **服务端 API 文档**: `docs/ClassTop-Client-API.md`
- **服务端项目**: `README.md` 和 `CLAUDE.md`
- **Tauri 文档**: https://tauri.app/v2/
- **PyTauri 文档**: https://pytauri.github.io/
- **Requests 文档**: https://requests.readthedocs.io/

---

**版本**: 1.0.0
**最后更新**: 2025-10-09
**维护者**: ClassTop Team