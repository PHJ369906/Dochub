<div align="center">

# DocHub 文档管理系统

**基于 Tauri 2 + Vue 3 的跨平台桌面文档管理工具**

[![Vue](https://img.shields.io/badge/Vue-3.x-4FC08D?logo=vue.js)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-24C8D8?logo=tauri)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.x-CE422B?logo=rust)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178C6?logo=typescript)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

</div>

---

## 项目简介

DocHub 是一款**本地优先**的桌面文档管理工具，无需服务器、开箱即用。支持文档的上传、分类、标签、搜索与在线预览，内置跨平台 PDF 渲染引擎，同时适用于 macOS 和 Windows。

## 核心特性

- **本地存储** — 数据保存在本地 SQLite，无需联网，数据完全自主可控
- **文档管理** — 支持上传、编辑、删除、批量操作、分页浏览
- **多级分类** — 树形分类结构，支持无限层级，悬停显示完整名称
- **标签系统** — 自定义标签，支持多标签组合筛选
- **在线预览** — 内置 PDF.js 渲染（Mac/Windows 均支持）、图片、文本、Word/Excel 转 HTML 预览
- **高级搜索** — 关键词 + 分类 + 标签 + 发布机关 + 时间范围组合筛选
- **数据导入导出** — 支持文档元数据批量导入导出
- **跨平台** — 基于 Tauri 2，打包为 macOS（dmg）和 Windows（nsis 安装包）

## 技术架构

```
DocHub
├── frontend/                   # 前端（Vue 3 + TypeScript）
│   ├── src/
│   │   ├── api/                # Tauri IPC 调用封装
│   │   ├── components/         # 通用组件
│   │   ├── views/              # 页面视图
│   │   ├── stores/             # Pinia 状态管理
│   │   └── types/              # TypeScript 类型定义
│   └── src-tauri/              # Tauri 后端（Rust）
│       └── src/
│           ├── commands/       # IPC 命令处理
│           ├── services/       # 业务逻辑
│           └── db/             # SQLite 数据库操作
└── sql/                        # 数据库结构参考脚本
```

### 前端技术栈

| 技术 | 版本 | 说明 |
|------|------|------|
| Vue 3 | 3.4+ | 响应式 UI 框架 |
| TypeScript | 5.x | 类型安全 |
| Vite | 5.x | 构建工具 |
| Element Plus | 2.4+ | UI 组件库 |
| Pinia | 2.x | 状态管理 |
| pdfjs-dist | 4.x | PDF 渲染（兼容 WebKit / WebView2）|
| Tailwind CSS | 3.x | 原子化样式 |

### 后端技术栈（Rust）

| 技术 | 说明 |
|------|------|
| Tauri 2 | 跨平台桌面运行时 |
| SQLite（rusqlite）| 嵌入式数据库，零外部依赖 |
| quick-xml | Office 文档 XML 解析 |

## 开发环境准备

### 安装依赖

```bash
# 1. 安装 Rust（如未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安装 Node.js 18+
nvm install 18

# 3. 安装前端依赖
cd frontend && npm install
```

### 启动开发服务器

```bash
cd frontend
npm run tauri:dev
```

> 首次运行需编译 Rust 依赖（约 3-5 分钟），后续启动秒开。

### 生产构建

```bash
cd frontend
npm run tauri:build
```

构建产物位于 `frontend/src-tauri/target/release/bundle/`：
- macOS：`*.dmg`
- Windows：`*_setup.exe`（NSIS 安装包）

## 文件预览支持

| 格式 | 预览方式 |
|------|----------|
| PDF | pdfjs-dist Canvas 渲染，支持翻页 / 缩放 / Retina 高清 |
| 图片（jpg / png / gif / webp 等）| Base64 内嵌 |
| 文本（txt / md / json / xml 等）| 纯文本展示 |
| Word（doc / docx）| XML 解析转 HTML |
| Excel（xls / xlsx）| XML 解析转表格 |
| 视频 / 音频 | HTML5 原生播放器 |
| PPT / 其他 | 调用系统默认应用打开 |

## 默认账户

| 用户名 | 密码 | 角色 |
|--------|------|------|
| admin | admin123 | 管理员（可管理用户和分类）|
| user | user123 | 普通用户 |
| demo | demo123 | 演示账户 |

## 常见问题

**Q: 首次启动很慢？**
A: Tauri 首次运行需编译 Rust 后端，约 3-5 分钟，后续启动秒开。

**Q: Windows 上 PDF 预览空白？**
A: 已修复。WebView2 不内置 PDF 查看器，项目使用 pdfjs-dist 4.x 通过 Canvas 渲染，无需任何插件。

**Q: PDF 预览文字模糊？**
A: 已修复。渲染时按设备像素比（DPR）放大 Canvas 并通过 CSS 缩回逻辑像素，Retina 屏完全清晰。

## Contributing

欢迎提交 Issue 和 Pull Request！

## License

[MIT](LICENSE)
