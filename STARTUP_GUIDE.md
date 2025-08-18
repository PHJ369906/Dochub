# 🚀 项目启动指南

## ⚠️ 重要修复说明

我们已经修复了Spring Security相关的编译错误：
- 移除了 `SecurityConfig.java`
- 移除了 `BCryptPasswordEncoder` 依赖
- 改为使用明文密码（仅用于开发演示）

## 📋 启动前准备

### 1. 环境要求
- ✅ JDK 17+
- ✅ Maven 3.6+
- ✅ Node.js 16+
- ✅ MySQL 8.0+
- ✅ Redis 6.0+

### 2. 检查工具版本
```bash
java -version
mvn -version
node -version
npm -version
```

## 🗄️ 数据库初始化

### 方式一：使用脚本（推荐）
```bash
# Windows
setup-database.bat

# Linux/Mac
./setup-database.sh
```

### 方式二：手动执行
```bash
# 完整初始化（包含示例数据）
mysql -u root -p < sql/init.sql

# 或仅结构初始化
mysql -u root -p vue_springboot_db < sql/schema.sql
```

## 🔧 启动步骤

### 1. 启动依赖服务
```bash
# 使用Docker Compose
docker-compose up -d

# 或手动启动MySQL和Redis
```

### 2. 测试后端编译
```bash
# Windows
test-compile.bat

# 或手动测试
cd backend
mvn clean compile
```

### 3. 启动后端
```bash
cd backend
mvn spring-boot:run
```

**后端启动成功标志：**
- 看到 "Started SpringbootBackendApplication"
- 访问 http://localhost:8080/doc.html 能看到API文档

### 4. 启动前端
```bash
cd frontend
npm install
npm run dev
```

**前端启动成功标志：**
- 看到 "Local: http://localhost:3000"
- 浏览器自动打开登录页面

## 🔐 默认账户

| 用户名 | 密码 | 角色 | 说明 |
|--------|------|------|------|
| admin | admin123 | 管理员 | 完整权限 |
| user | user123 | 普通用户 | 文件管理权限 |
| demo | demo123 | 演示用户 | 测试账户 |

## 🌐 访问地址

- **前端应用**: http://localhost:3000
- **后端API**: http://localhost:8080
- **API文档**: http://localhost:8080/doc.html

## 🐛 常见问题

### 1. 编译错误
```
java: 程序包org.springframework.security.crypto.bcrypt不存在
```
**解决方案**: 已修复，确保使用最新代码

### 2. 数据库连接失败
```
Communications link failure
```
**解决方案**: 
- 检查MySQL是否启动
- 检查数据库配置 `application.yml`
- 确保数据库已创建

### 3. Redis连接失败
```
Unable to connect to Redis
```
**解决方案**:
- 检查Redis是否启动
- 检查端口6379是否被占用

### 4. 端口被占用
```
Port 8080 was already in use
```
**解决方案**:
- 修改 `application.yml` 中的端口
- 或停止占用端口的进程

### 5. 前端依赖安装失败
```
npm ERR! network timeout
```
**解决方案**:
```bash
# 使用淘宝镜像
npm config set registry https://registry.npmmirror.com
npm install
```

## 🔧 开发建议

### 1. IDE配置
- **IDEA**: 安装Lombok插件
- **VSCode**: 安装Java Extension Pack

### 2. 热重载
- 后端：使用 `spring-boot-devtools`
- 前端：Vite自动热重载

### 3. 调试模式
```bash
# 后端调试模式启动
mvn spring-boot:run -Dspring-boot.run.jvmArguments="-Xdebug -Xrunjdwp:transport=dt_socket,server=y,suspend=n,address=5005"

# 前端开发模式
npm run dev
```

## 📝 下一步

1. **测试功能**: 登录系统，上传文件，测试预览
2. **查看文档**: 访问API文档了解接口
3. **开发新功能**: 基于现有架构扩展
4. **部署上线**: 配置生产环境

## 🆘 获取帮助

如果遇到问题：
1. 检查控制台错误信息
2. 查看日志文件
3. 参考 `sql/README.md` 数据库文档
4. 检查网络和防火墙设置

---

**祝您使用愉快！** 🎉
