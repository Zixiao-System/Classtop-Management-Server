# ClassTop Management Server - Frontend

这是 ClassTop Management Server 的前端应用，使用 Vue 3 + Vite + MDUI 2 构建。

## 技术栈

- **框架**: Vue 3 (Composition API)
- **构建工具**: Vite 5
- **UI 组件**: MDUI 2 (Material Design)
- **样式**: CSS + MDUI Design Tokens
- **主题**: 支持亮色/暗色/自动切换

## 开发

### 安装依赖

```bash
npm install
```

### 启动开发服务器

```bash
npm run dev
```

开发服务器会在 http://localhost:5173 启动，API 请求会自动代理到后端服务器 (http://localhost:8765)。

### 构建生产版本

```bash
npm run build
```

构建输出会生成到 `../static/` 目录，供后端服务器使用。

### 预览生产构建

```bash
npm run preview
```

## 项目结构

```
frontend/
├── src/
│   ├── App.vue              # 主应用组件
│   ├── main.js              # 应用入口
│   ├── api.js               # API 请求封装
│   └── components/          # Vue 组件
│       ├── DashboardView.vue    # 仪表板视图
│       ├── ClientsView.vue      # 客户端管理视图
│       └── DataView.vue         # 数据查看视图
├── index.html               # HTML 模板
├── vite.config.js           # Vite 配置
└── package.json             # 依赖配置
```

## 功能特性

### 仪表板
- 服务器统计信息展示
- 客户端统计卡片
- 实时数据刷新

### 客户端管理
- 客户端列表展示
- 注册新客户端
- 查看客户端详情
- 删除客户端
- 状态监控 (在线/离线)

### 数据查看
- 按客户端查看课程列表
- 按客户端查看课程表
- 课程表分天显示

### UI 特性
- Material Design 3 设计语言
- 响应式布局
- 亮色/暗色主题切换
- 跟随系统主题自动切换
- MDUI 组件库 (卡片、按钮、对话框等)

## API 集成

前端通过 `src/api.js` 与后端 API 通信：

- `/api/statistics` - 获取服务器统计
- `/api/statistics/clients` - 获取客户端统计
- `/api/clients` - 客户端管理 (CRUD)
- `/api/clients/{id}/courses` - 获取客户端课程
- `/api/clients/{id}/schedule` - 获取客户端课程表

所有 API 请求都会自动处理错误并通过 MDUI snackbar 提示用户。

## 自定义元素

MDUI 2 使用 Web Components，所有 `mdui-*` 标签都配置为自定义元素 (在 `vite.config.js` 中配置)。

## 开发注意事项

1. **MDUI 组件**: 直接在模板中使用 `<mdui-*>` 标签，Vue 会将它们识别为自定义元素
2. **主题切换**: 使用 `setTheme()` 和 `getTheme()` 函数 (从 `mdui` 导入)
3. **API 代理**: 开发模式下，Vite 会自动代理 `/api/*` 请求到 `http://localhost:8765`
4. **构建输出**: 生产构建会清空并重新生成 `../static/` 目录

## 参考文档

- [Vue 3 文档](https://vuejs.org/)
- [Vite 文档](https://vitejs.dev/)
- [MDUI 2 文档](https://www.mdui.org/zh-cn/docs/2/)
