# DocHub - 桌面文档管理工具

基于 **Tauri 2 + Vue 3** 构建的跨平台桌面文档管理应用，支持文件上传、分类管理、标签筛选和在线预览。

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2 (Rust) |
| 前端 | Vue 3 + TypeScript |
| UI 组件 | Element Plus |
| 样式 | Tailwind CSS |
| 状态管理 | Pinia |
| 路由 | Vue Router |
| 构建工具 | Vite |

## 功能特性

- 文件上传（支持多文件、拖拽上传）
- 文件分类管理（树形结构）
- 文件标签系统与标签筛选
- 在线文件预览（PDF、图片、文本等）
- 高级搜索与过滤
- 文件下载统计
- 权限控制（公开 / 内部文件）
- 跨平台支持（Windows、macOS、Linux）

## 项目结构

```
├── frontend/                # 前端 + Tauri 应用
│   ├── src/                 # Vue3 源码
│   │   ├── api/             # 接口请求
│   │   ├── components/      # 公共组件
│   │   ├── router/          # 路由配置
│   │   ├── stores/          # Pinia 状态管理
│   │   ├── types/           # TypeScript 类型
│   │   └── views/           # 页面组件
│   ├── src-tauri/           # Tauri Rust 核心
│   │   ├── src/             # Rust 源码
│   │   └── tauri.conf.json  # Tauri 配置
│   └── package.json
└── .github/workflows/       # CI/CD 自动构建
```

## 开发环境

### 依赖

- Node.js >= 18
- Rust（通过 [rustup](https://rustup.rs/) 安装）

### 启动开发服务器

```bash
cd frontend
npm install
npm run tauri:dev
```

## 构建安装包

### 本地构建

```bash
cd frontend
npm install
npm run tauri:build
```

构建产物：
- **Windows**: `src-tauri/target/release/bundle/nsis/*.exe`
- **macOS**: `src-tauri/target/release/bundle/dmg/*.dmg`
- **Linux**: `src-tauri/target/release/bundle/deb/*.deb`

### CI 自动构建（GitHub Actions）

推送 Tag 自动触发多平台构建并生成 Release：

```bash
git tag v0.1.0
git push origin v0.1.0
```

## 默认账户

| 用户名 | 密码 | 角色 |
|--------|------|------|
| admin | admin123 | 管理员 |
| user | user123 | 普通用户 |
| demo | demo123 | 演示用户 |

## 许可证

MIT License
