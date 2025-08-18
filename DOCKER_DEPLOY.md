# Docker 部署指南

本指南将帮助你使用 Docker 和 Docker Compose 部署文档管理系统。

## 系统要求

- Docker >= 20.10
- Docker Compose >= 2.0
- 系统内存 >= 4GB
- 磁盘空间 >= 10GB

## 快速部署

### 1. 准备环境变量

```bash
# 复制环境变量模板
cp .env.example .env.prod

# 编辑环境变量文件
vim .env.prod
```

**重要**: 请确保修改以下关键配置：
- `MYSQL_ROOT_PASSWORD`: MySQL root 密码
- `MYSQL_PASSWORD`: 应用数据库密码
- `REDIS_PASSWORD`: Redis 密码
- `JWT_SECRET_KEY`: JWT 签名密钥（至少32位）
- `API_BASE_URL`: 应用的访问地址

### 2. 一键部署

```bash
# 执行部署脚本
./deploy.sh

# 带备份的部署
./deploy.sh --backup

# 清理旧镜像的部署
./deploy.sh --clean
```

### 3. 手动部署

如果需要更细粒度的控制：

```bash
# 1. 启动基础服务（MySQL + Redis）
docker-compose -f docker-compose.prod.yml up -d mysql redis

# 2. 等待数据库初始化完成
sleep 30

# 3. 启动应用服务
docker-compose -f docker-compose.prod.yml up -d kkfileview app

# 4. 启动反向代理
docker-compose -f docker-compose.prod.yml up -d nginx
```

## 服务架构

部署包含以下服务：

| 服务 | 端口 | 说明 |
|------|------|------|
| nginx | 80/443 | 反向代理和负载均衡 |
| app | 8080 | Spring Boot 主应用 |
| kkfileview | 8012 | 文件预览服务 |
| mysql | 3306 | MySQL 数据库 |
| redis | 6379 | Redis 缓存 |

## 访问地址

- **主应用**: http://localhost
- **API 接口**: http://localhost/api
- **文件预览**: http://localhost/preview
- **健康检查**: http://localhost/health

## 默认账户

| 用户名 | 密码 | 角色 |
|--------|------|------|
| admin | admin123 | 管理员 |
| user | user123 | 普通用户 |
| demo | demo123 | 演示用户 |

## 常用操作

### 查看服务状态

```bash
# 查看所有服务状态
docker-compose -f docker-compose.prod.yml ps

# 查看服务日志
docker-compose -f docker-compose.prod.yml logs -f app
docker-compose -f docker-compose.prod.yml logs -f nginx
```

### 服务管理

```bash
# 停止所有服务
docker-compose -f docker-compose.prod.yml down

# 重启特定服务
docker-compose -f docker-compose.prod.yml restart app

# 重新构建并启动
docker-compose -f docker-compose.prod.yml up -d --build

# 扩展应用服务（负载均衡）
docker-compose -f docker-compose.prod.yml up -d --scale app=3
```

### 数据备份

```bash
# 备份 MySQL 数据
docker exec doc-center-mysql mysqldump -u root -p document_center_db > backup_$(date +%Y%m%d).sql

# 备份上传文件
docker cp doc-center-app:/app/uploads ./backup_uploads_$(date +%Y%m%d)

# 备份 Redis 数据
docker exec doc-center-redis redis-cli BGSAVE
docker cp doc-center-redis:/data/dump.rdb ./backup_redis_$(date +%Y%m%d).rdb
```

### 数据恢复

```bash
# 恢复 MySQL 数据
cat backup.sql | docker exec -i doc-center-mysql mysql -u root -p document_center_db

# 恢复上传文件
docker cp ./backup_uploads doc-center-app:/app/uploads
```

## 监控和维护

### 健康检查

所有服务都配置了健康检查：

```bash
# 检查服务健康状态
docker-compose -f docker-compose.prod.yml ps

# 查看详细健康信息
docker inspect --format='{{json .State.Health}}' doc-center-app
```

### 日志管理

```bash
# 查看应用日志
docker logs -f doc-center-app

# 查看 Nginx 访问日志
docker exec doc-center-nginx tail -f /var/log/nginx/access.log

# 查看错误日志
docker exec doc-center-nginx tail -f /var/log/nginx/error.log
```

### 性能监控

```bash
# 查看容器资源使用情况
docker stats

# 查看特定容器的资源使用
docker stats doc-center-app
```

## 配置调优

### 应用配置

修改 `config/application-prod.yml` 文件：

- 数据库连接池大小
- JVM 内存参数
- Redis 连接配置
- 文件上传限制

### Nginx 配置

修改 `nginx/conf.d/default.conf` 文件：

- 负载均衡策略
- 缓存设置
- 压缩配置
- 安全头设置

### MySQL 配置

创建 `mysql/conf.d/my.cnf` 文件进行 MySQL 优化：

```ini
[mysqld]
max_connections = 200
innodb_buffer_pool_size = 1G
innodb_log_file_size = 256M
```

## 常见问题

### 1. 服务启动失败

```bash
# 查看详细错误信息
docker-compose -f docker-compose.prod.yml logs service_name

# 检查端口占用
netstat -tlnp | grep :8080
```

### 2. 数据库连接失败

- 检查 MySQL 是否完全启动
- 验证环境变量配置
- 确认网络连接

### 3. 文件上传失败

- 检查文件大小限制
- 验证存储目录权限
- 查看应用日志

### 4. 内存不足

```bash
# 调整 JVM 内存参数
docker-compose -f docker-compose.prod.yml exec app \
  bash -c 'export JAVA_OPTS="-Xms1g -Xmx2g" && java $JAVA_OPTS -jar app.jar'
```

## 安全建议

1. **修改默认密码**: 更改所有默认密码
2. **使用 HTTPS**: 配置 SSL 证书
3. **防火墙设置**: 只开放必要端口
4. **定期备份**: 建立自动备份机制
5. **监控日志**: 监控异常访问和错误

## 更新升级

```bash
# 1. 备份数据
./deploy.sh --backup

# 2. 拉取最新代码
git pull origin main

# 3. 重新部署
./deploy.sh --clean

# 4. 验证服务状态
docker-compose -f docker-compose.prod.yml ps
```

## 开发环境

如需在开发环境使用 Docker：

```bash
# 使用开发环境配置
docker-compose up -d

# 前端开发模式
cd frontend && npm run dev

# 后端开发模式
cd backend && mvn spring-boot:run
```