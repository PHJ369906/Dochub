# Windows Docker 部署指南

本指南专门针对 Windows 系统的 Docker 部署。

## 🖥️ Windows 环境准备

### 1. 安装 Docker Desktop

**下载地址**: https://www.docker.com/products/docker-desktop

**系统要求**:
- Windows 10/11 (64-bit)
- WSL 2 (推荐) 或 Hyper-V
- 至少 4GB 内存

**安装步骤**:
1. 下载并运行 Docker Desktop 安装程序
2. 重启计算机
3. 启动 Docker Desktop
4. 验证安装: 打开 PowerShell 或 CMD，运行 `docker --version`

### 2. 配置 WSL 2 (推荐)

如果使用 WSL 2 后端：
```powershell
# 以管理员身份运行 PowerShell
wsl --install
wsl --set-default-version 2
```

### 3. Docker Desktop 设置

打开 Docker Desktop 设置：
- **Resources**: 分配至少 4GB 内存，2-4 CPU
- **Docker Engine**: 可选配置国内镜像加速
- **File Sharing**: 确保项目所在驱动器已共享

## 🚀 快速部署

### 方式一：使用批处理脚本（推荐）

```batch
REM 1. 配置环境变量
copy .env.example .env.prod
notepad .env.prod

REM 2. 一键部署
deploy.bat

REM 3. 带选项部署
deploy.bat --backup  
deploy.bat --clean
```

### 方式二：使用 PowerShell

```powershell
# 1. 配置环境变量
Copy-Item .env.example .env.prod
notepad .env.prod

# 2. 部署服务
docker-compose -f docker-compose.prod.yml up -d --build

# 3. 查看状态
docker-compose -f docker-compose.prod.yml ps
```

### 方式三：手动部署

```batch
REM 1. 启动基础服务
docker-compose -f docker-compose.prod.yml up -d mysql redis

REM 2. 等待数据库启动（重要！）
timeout /t 30

REM 3. 启动应用服务
docker-compose -f docker-compose.prod.yml up -d kkfileview app nginx
```

## 🔧 Windows 特定配置

### 1. 文件路径问题

Windows 路径分隔符处理：
```yaml
# docker-compose.prod.yml 中的卷映射
volumes:
  - ./uploads:/app/uploads          # 相对路径（推荐）
  - C:\项目\uploads:/app/uploads    # 绝对路径（不推荐含中文）
```

### 2. 字符编码

确保文件使用 UTF-8 编码：
```batch
chcp 65001  # 设置 CMD 为 UTF-8 编码
```

### 3. 防火墙设置

Windows Defender 防火墙可能阻止端口访问：
- 允许 Docker Desktop 通过防火墙
- 如需要，手动开放端口 80、3000、8080

## 📝 环境变量配置

编辑 `.env.prod` 文件（使用记事本或其他编辑器）：

```env
# 数据库配置
MYSQL_ROOT_PASSWORD=your_secure_password
MYSQL_DATABASE=document_center_db
MYSQL_USER=app_user
MYSQL_PASSWORD=your_db_password

# Redis配置  
REDIS_PASSWORD=your_redis_password

# 应用配置
API_BASE_URL=http://localhost

# JWT密钥（必须修改）
JWT_SECRET_KEY=your_very_secure_jwt_secret_key_32chars
```

## 🛠️ 常用 Windows 命令

### PowerShell 命令

```powershell
# 查看所有容器
docker ps -a

# 查看服务日志
docker-compose -f docker-compose.prod.yml logs -f app

# 进入容器
docker exec -it doc-center-app bash

# 清理系统
docker system prune -f
docker volume prune -f

# 备份数据
$date = Get-Date -Format "yyyyMMdd_HHmmss"
docker exec doc-center-mysql mysqladump -u root -p document_center_db > "backup_$date.sql"
```

### CMD 命令

```batch
REM 查看容器状态
docker ps

REM 重启服务
docker-compose -f docker-compose.prod.yml restart app

REM 查看实时日志
docker logs -f doc-center-app

REM 停止所有服务
docker-compose -f docker-compose.prod.yml down
```

## 🔍 故障排查

### 1. Docker Desktop 无法启动

**症状**: "Docker Desktop starting..." 一直显示

**解决方案**:
```batch
REM 重启 Docker 服务
net stop com.docker.service
net start com.docker.service

REM 或重置 Docker Desktop
"C:\Program Files\Docker\Docker\Docker Desktop.exe" --uninstall
```

### 2. 容器无法访问网络

**症状**: 容器内无法连接外网或其他容器

**解决方案**:
```batch
REM 重置 Docker 网络
docker network prune -f
docker-compose -f docker-compose.prod.yml down
docker-compose -f docker-compose.prod.yml up -d
```

### 3. 端口被占用

**症状**: "Port 8080 is already in use"

**解决方案**:
```batch
REM 查看端口占用
netstat -ano | findstr :8080

REM 结束进程（替换PID）
taskkill /PID 1234 /F

REM 或修改端口配置
```

### 4. 内存不足

**症状**: 容器频繁重启或性能差

**解决方案**:
- Docker Desktop -> Settings -> Resources -> 增加内存分配
- 关闭不必要的应用程序
- 重启计算机

### 5. WSL 2 相关问题

**症状**: Docker 启动慢或不稳定

**解决方案**:
```batch
REM 重启 WSL
wsl --shutdown
wsl --unregister docker-desktop
wsl --unregister docker-desktop-data

REM 重新启动 Docker Desktop
```

## 📊 性能优化

### 1. Docker Desktop 优化

Settings -> Resources 配置：
- **Memory**: 4-8GB（根据系统配置）
- **CPU**: 2-4 cores
- **Disk**: 至少20GB空间

### 2. WSL 2 内存限制

创建 `%USERPROFILE%\.wslconfig` 文件：
```ini
[wsl2]
memory=4GB
processors=2
swap=2GB
```

### 3. 镜像加速

Docker Desktop -> Settings -> Docker Engine，添加：
```json
{
  "registry-mirrors": [
    "https://docker.mirrors.ustc.edu.cn/",
    "https://hub-mirror.c.163.com/",
    "https://reg-mirror.qiniu.com"
  ]
}
```

## 🎯 部署验证

部署完成后验证：

```batch
REM 1. 检查所有服务状态
docker-compose -f docker-compose.prod.yml ps

REM 2. 测试应用访问
curl http://localhost/health
REM 或在浏览器打开 http://localhost

REM 3. 检查数据库连接
docker exec -it doc-center-mysql mysql -u root -p -e "SHOW DATABASES;"

REM 4. 检查日志无错误
docker-compose -f docker-compose.prod.yml logs --tail 50
```

## 📱 访问地址

部署成功后的访问地址：
- **主应用**: http://localhost
- **API 接口**: http://localhost/api
- **文件预览**: http://localhost/preview
- **健康检查**: http://localhost/health

## 🔐 默认账户

| 用户名 | 密码 | 角色 |
|--------|------|------|
| admin | admin123 | 系统管理员 |
| user | user123 | 普通用户 |
| demo | demo123 | 演示账户 |

## 📞 技术支持

如遇问题，可以：
1. 查看 Docker Desktop 的 Troubleshoot 功能
2. 检查 Windows 事件查看器中的应用程序日志
3. 确保 Windows 系统和 Docker Desktop 都是最新版本

---

**Windows 部署愉快！** 🎉