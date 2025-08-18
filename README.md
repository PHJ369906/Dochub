# Vue3 + SpringBoot 全栈项目

这是一个基于 Vue3 + SpringBoot + Sa-Token 的全栈Web应用项目，实现了完整的用户认证和权限管理功能。

## 项目结构

```
├── frontend/          # Vue3前端项目
│   ├── src/
│   │   ├── api/       # API接口
│   │   ├── components/# 组件
│   │   ├── router/    # 路由配置
│   │   ├── stores/    # Pinia状态管理
│   │   ├── types/     # TypeScript类型定义
│   │   ├── views/     # 页面组件
│   │   └── main.ts    # 入口文件
│   ├── package.json
│   └── vite.config.ts
├── backend/           # SpringBoot后端项目
│   ├── src/main/java/com/example/
│   │   ├── config/    # 配置类
│   │   ├── controller/# 控制器
│   │   ├── dto/       # 数据传输对象
│   │   ├── entity/    # 实体类
│   │   ├── repository/# 数据访问层
│   │   ├── service/   # 服务层
│   │   └── exception/ # 异常处理
│   ├── src/main/resources/
│   │   └── application.yml
│   └── pom.xml
└── README.md
```

## 技术栈

### 前端
- **Vue 3** - 渐进式JavaScript框架
- **TypeScript** - JavaScript的超集
- **Vite** - 现代化构建工具
- **Vue Router** - 官方路由管理器
- **Pinia** - 状态管理库
- **Element Plus** - Vue 3 UI组件库
- **Tailwind CSS** - 实用优先的CSS框架
- **Axios** - HTTP客户端

### 后端
- **SpringBoot 3.x** - Java企业级应用框架
- **Sa-Token** - 轻量级权限认证框架
- **Spring Data JPA** - 数据持久化
- **MySQL** - 关系型数据库
- **Redis** - 内存数据库
- **Knife4j** - API文档工具

## 功能特性

### 基础功能
- ✅ 用户注册/登录
- ✅ JWT Token认证
- ✅ 路由权限控制
- ✅ 角色权限管理
- ✅ 用户信息管理
- ✅ 响应式设计
- ✅ API文档集成

### 文件管理功能
- ✅ 文件上传（支持多文件、拖拽上传）
- ✅ 文件分类管理（树形结构）
- ✅ 文件标签系统
- ✅ 在线文件预览（内置预览引擎）
- ✅ 高级搜索和过滤
- ✅ 文件下载统计
- ✅ 权限控制（公开/内部文件）
- ✅ 支持多种文件格式（PDF、Office、图片、视频等）

## 环境要求

### 前端
- Node.js >= 16
- npm 或 yarn

### 后端
- JDK 17+
- Maven 3.6+
- MySQL 8.0+
- Redis

## 快速开始

### 方式一：使用Docker Compose（推荐）

1. **启动依赖服务**
```bash
docker-compose up -d
```

这将启动：
- MySQL数据库（端口3306）
- Redis缓存（端口6379）

2. **初始化数据库**
```bash
# Windows
setup-database.bat

# Linux/Mac
./setup-database.sh
```

3. **启动后端**
```bash
cd backend
mvn spring-boot:run
```

4. **启动前端**
```bash
cd frontend
npm install
npm run dev
```

### 方式二：手动安装

1. **安装MySQL和Redis**
   - MySQL 8.0+
   - Redis 6.0+

2. **创建数据库**
```sql
CREATE DATABASE document_center_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

3. **初始化数据库结构**
```bash
# 完整初始化（包含示例数据）
mysql -u root -p < sql/init.sql

# 或仅初始化结构（生产环境）
mysql -u root -p vue_springboot_db < sql/schema.sql
```

4. **启动Redis**
确保Redis服务正在运行（默认端口6379）

5. **后端启动**
```bash
cd backend
mvn clean install
mvn spring-boot:run
```

6. **前端启动**
```bash
cd frontend
npm install
npm run dev
```

### 访问应用

- 前端应用: http://localhost:3000
- 后端API: http://localhost:8080
- API文档: http://localhost:8080/doc.html

## 默认账户

系统启动后会自动创建以下测试账户：

| 用户名 | 密码 | 角色 | 说明 |
|--------|------|------|------|
| admin | admin123 | 管理员 | 拥有所有权限，可以管理用户和文件分类 |
| user | user123 | 普通用户 | 可以上传、查看、管理自己的文件 |
| demo | demo123 | 演示用户 | 用于演示和测试 |

## 数据库说明

### 数据库结构
- **sys_user**: 用户表，存储用户信息和角色
- **file_category**: 文件分类表，支持树形结构
- **file_tag**: 文件标签表，支持多标签管理
- **policy_file**: 政策文件表，存储文件元数据
- **policy_file_tag**: 文件标签关联表

### 初始化脚本
- `sql/init.sql`: 完整初始化（包含示例数据）
- `sql/schema.sql`: 仅结构初始化（生产环境）
- `sql/README.md`: 详细的数据库说明文档

### 数据库配置
```yaml
spring:
  datasource:
    url: jdbc:mysql://localhost:3306/document_center_db?useUnicode=true&characterEncoding=utf8&useSSL=false&serverTimezone=Asia/Shanghai&allowPublicKeyRetrieval=true&connectionCollation=utf8mb4_unicode_ci
    username: root
    password: 123456
```

## API接口

### 认证相关
- `POST /auth/login` - 用户登录
- `POST /auth/register` - 用户注册
- `POST /auth/logout` - 用户登出
- `GET /auth/userinfo` - 获取当前用户信息
- `GET /auth/users` - 获取用户列表（管理员）
- `GET /auth/check` - 检查登录状态

### 文件管理相关
- `POST /files/upload` - 上传文件
- `GET /files` - 获取文件列表（支持搜索和过滤）
- `GET /files/{id}` - 获取文件详情
- `PUT /files/{id}` - 更新文件信息
- `DELETE /files/{id}` - 删除文件
- `GET /files/download/{id}` - 下载文件
- `GET /files/preview/{id}` - 获取文件预览信息
- `GET /files/popular` - 获取热门文件
- `GET /files/latest` - 获取最新文件

### 分类管理相关
- `GET /categories` - 获取所有分类
- `POST /categories` - 创建分类（管理员）
- `PUT /categories/{id}` - 更新分类（管理员）
- `DELETE /categories/{id}` - 删除分类（管理员）

## 开发说明

### 前端开发
- 使用 `npm run dev` 启动开发服务器
- 使用 `npm run build` 构建生产版本
- 使用 `npm run lint` 进行代码检查

### 后端开发
- 使用 `mvn spring-boot:run` 启动开发服务器
- 使用 `mvn clean package` 构建生产版本
- API文档自动生成，访问 `/doc.html`

## 部署

### 前端部署
```bash
cd frontend
npm run build
# 将 dist 目录部署到Web服务器
```

### 后端部署
```bash
cd backend
mvn clean package
java -jar target/springboot-backend-0.0.1-SNAPSHOT.jar
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
