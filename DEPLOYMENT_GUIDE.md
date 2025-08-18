# 📖 完整部署指南

## 🎯 项目概述

这是一个基于 **Vue 3 + Spring Boot** 的全栈文档管理系统，支持文件上传、在线预览、分类管理、权限控制等企业级功能。

### 🏗️ 技术架构
- **前端**: Vue 3 + TypeScript + Element Plus + Tailwind CSS
- **后端**: Spring Boot 3.x + Sa-Token + MyBatis-Plus + MySQL + Redis  
- **文件预览**: kkFileView 开源预览服务（支持Office、PDF等格式）
- **容器化**: Docker + Docker Compose
- **反向代理**: Nginx（负载均衡、SSL终端）

### 🌟 核心功能
- 📁 文件上传与管理（支持拖拽上传）
- 👁️ 在线文件预览（Office、PDF、图片、视频等）
- 🗂️ 分类管理（树形结构）
- 👥 用户权限管理
- 🏷️ 文件标签系统
- 🔍 全文搜索与高级筛选
- 📊 使用统计与监控

## 服务器要求

### 最低配置
- **CPU**: 2核
- **内存**: 4GB
- **硬盘**: 50GB
- **操作系统**: Ubuntu 20.04+ / CentOS 7+ / Red Hat 8+

### 推荐配置
- **CPU**: 4核
- **内存**: 8GB
- **硬盘**: 100GB SSD
- **操作系统**: Ubuntu 22.04 LTS

## 部署前准备

### 1. 安装 Docker 和 Docker Compose

#### Ubuntu/Debian
```bash
# 更新包列表
sudo apt update

# 安装必要依赖
sudo apt install -y ca-certificates curl gnupg lsb-release

# 添加Docker官方GPG密钥
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

# 设置Docker仓库
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# 安装Docker Engine
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# 启动Docker服务
sudo systemctl enable docker
sudo systemctl start docker

# 添加当前用户到docker组（可选）
sudo usermod -aG docker $USER
```

#### CentOS/RHEL
```bash
# 安装必要依赖
sudo yum update -y
sudo yum install -y yum-utils device-mapper-persistent-data lvm2

# 添加Docker仓库
sudo yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo

# 安装Docker
sudo yum install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# 启动Docker服务
sudo systemctl enable docker
sudo systemctl start docker
```

### 2. 防火墙配置
```bash
# Ubuntu (UFW)
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 22/tcp

# CentOS/RHEL (firewalld)
sudo firewall-cmd --permanent --add-port=80/tcp
sudo firewall-cmd --permanent --add-port=443/tcp
sudo firewall-cmd --reload
```

## 🐳 Docker 部署（生产环境推荐）

### 1. 下载项目代码
```bash
# 使用git克隆（推荐）
git clone https://github.com/your-username/doc-center.git
cd doc-center

# 或者下载压缩包后解压
wget https://github.com/your-username/doc-center/archive/main.zip
unzip main.zip
cd doc-center-main
```

### 📋 服务架构
部署包含以下服务：

| 服务 | 端口 | 说明 |
|------|------|------|
| nginx | 80/443 | 反向代理和静态资源 |
| app | 8080 | Spring Boot 主应用（集成文件预览） |
| mysql | 3306 | MySQL 数据库 |
| redis | 6379 | Redis 缓存 |

### 2. 配置环境变量
```bash
# 复制环境变量模板
cp .env.example .env.prod

# 编辑生产环境配置
nano .env.prod
```

**重要配置项说明：**
```bash
# 数据库密码（必须修改）
MYSQL_ROOT_PASSWORD=your_strong_password_123
MYSQL_PASSWORD=your_db_password_456

# Redis密码（必须修改）
REDIS_PASSWORD=your_redis_password_789

# JWT密钥（必须修改为32位随机字符串）
JWT_SECRET_KEY=your_jwt_secret_key_32_characters

# API地址（修改为服务器实际域名或IP）
API_BASE_URL=http://your-domain.com:8080
```

### 3. 生成强密码（推荐）
```bash
# 生成MySQL root密码
openssl rand -base64 32

# 生成MySQL用户密码
openssl rand -base64 32

# 生成Redis密码
openssl rand -base64 32

# 生成JWT密钥
openssl rand -base64 32 | cut -c1-32
```

### 4. 执行部署

#### Linux/Mac 部署
```bash
# 方式一：使用部署脚本（推荐）
chmod +x deploy.sh
./deploy.sh --backup

# 方式二：手动部署
docker-compose -f docker-compose.prod.yml up -d --build
```

#### Windows 部署

**方式一：使用批处理脚本**
```batch
:: 1. 配置环境变量
copy .env.example .env.prod
notepad .env.prod

:: 2. 一键部署
deploy.bat

:: 3. 带选项部署
deploy.bat --backup  
deploy.bat --clean
```

**方式二：PowerShell**
```powershell
# 1. 配置环境变量
Copy-Item .env.example .env.prod
notepad .env.prod

# 2. 部署服务
docker-compose -f docker-compose.prod.yml up -d --build
```

**Windows 特定注意事项：**
- 确保 Docker Desktop 已正确安装并运行
- 建议使用 WSL 2 后端以获得更好性能
- 文件路径使用正斜杠 `/` 而非反斜杠 `\`
- 如遇防火墙问题，允许 Docker Desktop 通过防火墙

### 5. 验证部署
```bash
# 检查服务状态
docker-compose -f docker-compose.prod.yml ps

# 查看应用日志
docker logs doc-center-app

# 检查健康状态
curl http://localhost/health
```

## 域名和SSL配置

### 1. 域名解析
将你的域名 A 记录指向服务器 IP 地址。

### 2. SSL证书配置（使用 Let's Encrypt）
```bash
# 安装 Certbot
sudo apt install certbot python3-certbot-nginx

# 获取SSL证书
sudo certbot --nginx -d your-domain.com

# 设置自动续期
sudo crontab -e
# 添加以下行
0 12 * * * /usr/bin/certbot renew --quiet
```

### 3. Nginx配置更新
编辑 `nginx/conf.d/default.conf`，添加SSL配置：

```nginx
server {
    listen 443 ssl http2;
    server_name your-domain.com;
    
    ssl_certificate /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;
    
    # 其他配置...
}

server {
    listen 80;
    server_name your-domain.com;
    return 301 https://$server_name$request_uri;
}
```

## 维护操作

### 更新应用
```bash
# 拉取最新代码
git pull origin main

# 重新部署
./deploy.sh --backup
```

### 数据备份
```bash
# 备份数据库
docker exec doc-center-mysql mysqldump -u root -p document_center_db > backup_$(date +%Y%m%d).sql

# 备份上传文件
tar -czf uploads_backup_$(date +%Y%m%d).tar.gz uploads/
```

### 数据恢复
```bash
# 恢复数据库
docker exec -i doc-center-mysql mysql -u root -p document_center_db < backup_20250118.sql

# 恢复上传文件
tar -xzf uploads_backup_20250118.tar.gz
```

### 监控和日志
```bash
# 查看所有服务状态
docker-compose -f docker-compose.prod.yml ps

# 查看应用日志
docker logs -f doc-center-app

# 查看数据库日志
docker logs doc-center-mysql

# 查看系统资源使用
docker stats
```

### 性能调优
```bash
# 修改应用内存限制
# 在 .env.prod 中添加：
JAVA_OPTS=-Xms1g -Xmx2g

# 重启应用
docker-compose -f docker-compose.prod.yml restart app
```

## 常见问题排查

### 1. 服务启动失败
```bash
# 检查端口占用
sudo netstat -tlnp | grep :80
sudo netstat -tlnp | grep :3306

# 查看详细错误日志
docker logs doc-center-app --tail 100
```

### 2. 数据库连接失败
```bash
# 检查MySQL服务
docker logs doc-center-mysql

# 测试数据库连接
docker exec -it doc-center-mysql mysql -u root -p
```

### 3. 文件上传失败
```bash
# 检查上传目录权限
ls -la uploads/

# 检查磁盘空间
df -h
```

### 4. 内存不足
```bash
# 检查内存使用
free -h
docker stats

# 调整Java堆大小
# 修改 .env.prod 中的 JAVA_OPTS
```

## 默认账户

部署完成后，可使用以下默认账户登录：

- **管理员**: admin / admin123
- **普通用户**: user / user123
- **演示账户**: demo / demo123

**⚠️ 重要：生产环境请立即修改默认密码！**

## 支持与维护

- 应用访问地址: `http://your-domain.com`
- API文档地址: `http://your-domain.com/doc.html`
- 文件预览地址: `http://your-domain.com/preview`

如遇问题，请检查：
1. 服务器防火墙设置
2. Docker服务状态
3. 环境变量配置
4. 应用日志信息

---

## 💻 开发环境部署

如果需要在本地开发环境运行项目：

### 环境要求
- JDK 17+
- Maven 3.6+
- Node.js 16+
- MySQL 8.0+
- Redis 6.0+

### 启动步骤

#### 1. 数据库初始化
```bash
# 启动 MySQL 和 Redis
docker-compose up -d mysql redis

# 初始化数据库
mysql -u root -p < sql/init.sql
```

#### 2. 启动后端
```bash
cd backend
mvn clean install
mvn spring-boot:run
```

#### 3. 启动前端
```bash
cd frontend
npm install
npm run dev
```


### 开发环境访问地址
- **前端应用**: http://localhost:3000
- **后端API**: http://localhost:8080
- **API文档**: http://localhost:8080/doc.html
- **文件预览**: http://localhost:8080/api/preview （集成在主应用中）

## 🔄 快速命令参考

### Docker 管理
```bash
# 查看服务状态
docker-compose -f docker-compose.prod.yml ps

# 查看日志
docker-compose -f docker-compose.prod.yml logs -f app

# 重启服务
docker-compose -f docker-compose.prod.yml restart app

# 停止所有服务
docker-compose -f docker-compose.prod.yml down

# 清理系统
docker system prune -f
docker volume prune -f
```

### 数据管理
```bash
# 备份数据库
docker exec doc-center-mysql mysqladump -u root -p document_center_db > backup_$(date +%Y%m%d).sql

# 恢复数据库
docker exec -i doc-center-mysql mysql -u root -p document_center_db < backup.sql

# 备份文件
tar -czf uploads_backup.tar.gz uploads/
```

---

**部署完成！🎉**

如有问题，请参考故障排查部分或查看项目文档。