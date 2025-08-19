# PowerShell Docker 部署脚本
param(
    [switch]$Backup,
    [switch]$NoStop,
    [switch]$Clean,
    [switch]$Help
)

# 颜色函数
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    } else {
        $input | Write-Output
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-Info($message) {
    Write-ColorOutput Blue "[INFO] $message"
}

function Write-Success($message) {
    Write-ColorOutput Green "[SUCCESS] $message"
}

function Write-Warning($message) {
    Write-ColorOutput Yellow "[WARNING] $message"
}

function Write-Error($message) {
    Write-ColorOutput Red "[ERROR] $message"
}

# 显示帮助信息
function Show-Help {
    Write-Host "Usage: .\deploy.ps1 [options]"
    Write-Host "Options:"
    Write-Host "  -Backup      备份现有数据"
    Write-Host "  -NoStop      不停止现有服务"
    Write-Host "  -Clean       清理旧的镜像和卷"
    Write-Host "  -Help        显示帮助信息"
}

# 检查命令是否存在
function Test-Command($command) {
    try {
        if (Get-Command $command -ErrorAction Stop) {
            return $true
        }
    }
    catch {
        return $false
    }
}

# 检查环境
function Test-Environment {
    Write-Info "检查部署环境..."
    
    # 检查 Docker
    if (-not (Test-Command "docker")) {
        Write-Error "Docker 未安装，请先安装 Docker Desktop"
        return $false
    }
    
    # 检查 Docker Compose
    if (-not (Test-Command "docker-compose")) {
        Write-Error "Docker Compose 未安装或未在PATH中"
        return $false
    }
    
    # 检查 Docker 是否运行
    try {
        docker info | Out-Null
    }
    catch {
        Write-Error "Docker 未运行，请启动 Docker Desktop"
        return $false
    }
    
    # 检查环境变量文件
    if (-not (Test-Path ".env.prod")) {
        Write-Warning ".env.prod 文件不存在"
        if (Test-Path ".env.example") {
            Write-Info "从 .env.example 创建 .env.prod"
            Copy-Item ".env.example" ".env.prod"
            Write-Warning "请编辑 .env.prod 文件并设置正确的环境变量"
            Write-Host "使用记事本编辑: notepad .env.prod"
            return $false
        } else {
            Write-Error "未找到 .env.example 文件"
            return $false
        }
    }
    
    Write-Success "环境检查通过"
    return $true
}

# 备份数据
function Backup-Data {
    if ($Backup) {
        Write-Info "备份现有数据..."
        
        $backupDir = "backup\$(Get-Date -Format 'yyyyMMdd_HHmmss')"
        New-Item -ItemType Directory -Path $backupDir -Force | Out-Null
        
        # 备份数据库
        $containers = docker ps --format "table {{.Names}}" | Select-String "doc-center-mysql"
        if ($containers) {
            Write-Info "备份 MySQL 数据..."
            $envContent = Get-Content ".env.prod" -Raw
            $mysqlPassword = ($envContent | Select-String "MYSQL_ROOT_PASSWORD=(.+)" | ForEach-Object { $_.Matches.Groups[1].Value }).Trim()
            
            $cmd = "docker exec doc-center-mysql mysqladump -u root -p$mysqlPassword document_center_db"
            Invoke-Expression $cmd | Out-File "$backupDir\database.sql" -Encoding UTF8
        }
        
        # 备份上传文件
        if (Test-Path "uploads") {
            Write-Info "备份上传文件..."
            Copy-Item -Path "uploads" -Destination "$backupDir\uploads" -Recurse
        }
        
        Write-Success "数据备份完成: $backupDir"
    }
}

# 部署服务
function Deploy-Services {
    Write-Info "开始部署服务..."
    
    # 停止现有服务
    if (-not $NoStop) {
        Write-Info "停止现有服务..."
        docker-compose -f docker-compose.prod.yml down
    }
    
    # 清理旧的镜像
    if ($Clean) {
        Write-Info "清理旧镜像..."
        docker image prune -f
        docker volume prune -f
    }
    
    # 构建并启动服务
    Write-Info "构建和启动服务..."
    $result = docker-compose -f docker-compose.prod.yml up -d --build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "服务启动失败"
        return $false
    }
    
    # 等待服务启动
    Write-Info "等待服务启动..."
    Start-Sleep -Seconds 30
    
    # 检查服务状态
    Test-ServicesHealth
    return $true
}

# 检查服务健康状态
function Test-ServicesHealth {
    Write-Info "检查服务健康状态..."
    
    $services = @("doc-center-mysql", "doc-center-redis", "doc-center-app", "doc-center-nginx")
    
    foreach ($service in $services) {
        $containers = docker ps --format "table {{.Names}}" | Select-String $service
        if ($containers) {
            Write-Success "✓ $service 运行正常"
        } else {
            Write-Error "✗ $service 未运行"
            Write-Host "最近日志:"
            docker logs $service --tail 20
        }
    }
    
    # 检查应用接口
    Write-Info "检查应用接口..."
    try {
        $response = Invoke-WebRequest -Uri "http://localhost/health" -TimeoutSec 10
        if ($response.StatusCode -eq 200) {
            Write-Success "✓ 应用健康检查通过"
        }
    }
    catch {
        Write-Warning "⚠ 应用健康检查失败，可能还在启动中"
    }
}

# 显示部署信息
function Show-DeploymentInfo {
    Write-Success "部署完成！"
    Write-Host ""
    Write-Host "===================================" -ForegroundColor Cyan
    Write-Host "       部署信息" -ForegroundColor Cyan
    Write-Host "===================================" -ForegroundColor Cyan
    Write-Host "应用访问地址: http://localhost"
    Write-Host "API接口地址:  http://localhost/api"
    Write-Host "文件预览地址: http://localhost/api/preview"
    Write-Host ""
    Write-Host "默认管理员账户:"
    Write-Host "用户名: admin"
    Write-Host "密码:   admin123"
    Write-Host ""
    Write-Host "常用命令:"
    Write-Host "查看服务状态: docker-compose -f docker-compose.prod.yml ps"
    Write-Host "查看应用日志: docker logs doc-center-app"
    Write-Host "停止所有服务: docker-compose -f docker-compose.prod.yml down"
    Write-Host "===================================" -ForegroundColor Cyan
}

# 主函数
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Info "开始 Docker 部署流程..."
    
    if (-not (Test-Environment)) {
        exit 1
    }
    
    Backup-Data
    
    if (-not (Deploy-Services)) {
        exit 1
    }
    
    Show-DeploymentInfo
}

# 脚本入口
Main