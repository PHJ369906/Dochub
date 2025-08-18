# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目架构

这是一个 Vue3 + Spring Boot 全栈文档管理系统，具有完整的用户认证、文件管理和预览功能。

### 核心架构组件
- **frontend/**: Vue3 + TypeScript + Element Plus + Tailwind CSS 前端应用
- **backend/**: Spring Boot 3.x + Sa-Token + MyBatis-Plus + MySQL 后端服务
- **kkFileView/**: 集成的开源文件预览服务，支持多种文档格式
- **sql/**: 数据库初始化和迁移脚本

## 开发环境设置

### 依赖服务启动
```bash
# 启动 MySQL 和 Redis
docker-compose up -d

# 初始化数据库
mysql -u root -p < sql/init.sql
```

### 前端开发
```bash
cd frontend
npm install
npm run dev        # 开发服务器 (localhost:3000)
npm run build      # 生产构建
npm run lint       # ESLint 代码检查
npm run format     # Prettier 格式化
```

### 后端开发
```bash
cd backend
mvn clean install          # 安装依赖
mvn spring-boot:run       # 开发服务器 (localhost:8080)
mvn clean package         # 生产构建
```

### kkFileView 预览服务
```bash
cd kkFileView/server
# Windows
startup.bat
# Linux/Mac  
./bin/startup.sh
```

## 核心技术栈

### 前端技术
- **Vue 3** + **TypeScript**: 响应式 UI 框架
- **Element Plus**: Vue 3 UI 组件库
- **Pinia**: 状态管理
- **Vue Router**: 路由管理
- **Vite**: 现代构建工具

### 后端技术
- **Spring Boot 3.x**: Java 企业应用框架
- **Sa-Token**: 权限认证框架（JWT + Redis）
- **MyBatis-Plus**: ORM 和代码生成
- **Apache POI + PDFBox**: Office 文档处理
- **Apache Tika**: 文件类型检测和内容提取

### 数据层
- **MySQL 8.0**: 主数据库
- **Redis**: 缓存和会话存储
- 数据库连接池: **HikariCP**

## 服务端口
- 前端开发服务器: `http://localhost:3000`
- 后端 API 服务: `http://localhost:8080`  
- API 文档: `http://localhost:8080/doc.html`
- kkFileView 预览: `http://localhost:8012`
- MySQL: `localhost:3306`
- Redis: `localhost:6379`

## 核心业务模块

### 认证授权 (`backend/src/main/java/com/example/controller/AuthController.java`)
- 基于 Sa-Token 的 JWT 认证
- Redis 存储会话状态
- 角色权限控制

### 文件管理 (`backend/src/main/java/com/example/service/`)
- **PolicyFileService**: 文件元数据管理
- **FileStorageService**: 文件物理存储
- **FileCategoryService**: 文件分类管理（树形结构）
- **FileConvertService**: 文件格式转换

### 文件预览
- 内置预览引擎: PDF、Office、图片、文本等
- 集成 kkFileView: 支持更多文档格式
- 预览策略可配置切换

## 数据库设计
- **sys_user**: 用户和角色信息
- **policy_file**: 文件元数据（UUID 存储策略）
- **file_category**: 文件分类（支持层级结构）
- **file_tag**: 文件标签系统

## 配置文件
- **前端**: `frontend/vite.config.ts`, `frontend/package.json`
- **后端**: `backend/src/main/resources/application.yml`
- **数据库**: `docker-compose.yml`, `sql/init.sql`

## 测试账户
- **admin/admin123**: 管理员账户
- **user/user123**: 普通用户账户
- **demo/demo123**: 演示账户

## Docker 部署

### Linux/Mac 部署
```bash
# 配置环境变量
cp .env.example .env.prod
vim .env.prod

# 一键部署
./deploy.sh

# 带备份部署
./deploy.sh --backup
```

### Windows 部署
```batch
# 批处理脚本部署
copy .env.example .env.prod
notepad .env.prod
deploy.bat

# PowerShell 脚本部署
Copy-Item .env.example .env.prod
notepad .env.prod
.\deploy.ps1 -Backup
```

### 开发环境快速启动（Windows）
```batch
# 一键启动开发环境
start-dev.bat

# 数据库初始化
setup-database.bat

# 编译测试
test-compile.bat
```

### Docker Compose 命令
```bash
# 启动生产环境
docker-compose -f docker-compose.prod.yml up -d

# 查看服务状态
docker-compose -f docker-compose.prod.yml ps

# 查看应用日志
docker-compose -f docker-compose.prod.yml logs -f app
```

## 常用开发命令

### 类型检查
```bash
# 前端 TypeScript 检查
cd frontend && npx vue-tsc

# 后端编译检查
cd backend && mvn compile
```

### 数据库操作
```bash
# 重置数据库
mysql -u root -p < sql/schema.sql

# 查看数据库连接
mysql -u root -p -e "SHOW PROCESSLIST;"

# Docker 数据库备份
docker exec doc-center-mysql mysqldump -u root -p document_center_db > backup.sql
```

## 文件上传和存储
- 文件存储路径: `backend/uploads/` (按日期分层)
- 支持文件类型: PDF, Office, 图片, 视频, 音频, 压缩包等
- 文件命名策略: UUID (防止冲突)
- 最大文件大小: 100MB

## API 架构模式
- RESTful API 设计
- 统一响应格式 (`Result.java`)
- 全局异常处理 (`GlobalExceptionHandler.java`)
- 请求参数验证 (`@Valid` + Bean Validation)

## 前端组件架构
- **AppLayout.vue**: 主布局组件
- **FileManagement.vue**: 文件管理主界面
- **FilePreviewDialog.vue**: 文件预览弹窗
- **AdvancedSearch.vue**: 高级搜索组件