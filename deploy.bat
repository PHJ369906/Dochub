@echo off
REM Windows Docker 部署脚本
chcp 65001 >nul
setlocal EnableDelayedExpansion

REM 颜色定义 (Windows 10+ 支持 ANSI 颜色)
set "RED=[31m"
set "GREEN=[32m"
set "YELLOW=[33m"
set "BLUE=[34m"
set "NC=[0m"

REM 默认参数
set BACKUP=false
set STOP_EXISTING=true
set CLEAN_IMAGES=false

REM 解析参数
:parse_args
if "%~1"=="" goto main
if "%~1"=="--backup" (
    set BACKUP=true
    shift
    goto parse_args
)
if "%~1"=="--no-stop" (
    set STOP_EXISTING=false
    shift
    goto parse_args
)
if "%~1"=="--clean" (
    set CLEAN_IMAGES=true
    shift
    goto parse_args
)
if "%~1"=="--help" goto show_help
if "%~1"=="-h" goto show_help
echo %RED%[ERROR]%NC% 未知参数: %~1
exit /b 1

:show_help
echo Usage: %0 [options]
echo Options:
echo   --backup      备份现有数据
echo   --no-stop     不停止现有服务
echo   --clean       清理旧的镜像和卷
echo   --help, -h    显示帮助信息
exit /b 0

:main
echo %BLUE%[INFO]%NC% 开始 Docker 部署流程...

REM 检查环境
call :check_environment
if errorlevel 1 exit /b 1

REM 备份数据
if "%BACKUP%"=="true" (
    call :backup_data
)

REM 部署服务
call :deploy_services
if errorlevel 1 exit /b 1

REM 显示部署信息
call :show_deployment_info

goto :eof

:check_environment
echo %BLUE%[INFO]%NC% 检查部署环境...

REM 检查 Docker
docker --version >nul 2>&1
if errorlevel 1 (
    echo %RED%[ERROR]%NC% Docker 未安装，请先安装 Docker Desktop
    exit /b 1
)

REM 检查 Docker Compose
docker-compose --version >nul 2>&1
if errorlevel 1 (
    echo %RED%[ERROR]%NC% Docker Compose 未安装或未在PATH中
    exit /b 1
)

REM 检查 Docker 是否运行
docker info >nul 2>&1
if errorlevel 1 (
    echo %RED%[ERROR]%NC% Docker 未运行，请启动 Docker Desktop
    exit /b 1
)

REM 检查环境变量文件
if not exist ".env.prod" (
    echo %YELLOW%[WARNING]%NC% .env.prod 文件不存在
    if exist ".env.example" (
        echo %BLUE%[INFO]%NC% 从 .env.example 创建 .env.prod
        copy ".env.example" ".env.prod" >nul
        echo %YELLOW%[WARNING]%NC% 请编辑 .env.prod 文件并设置正确的环境变量
        echo 使用记事本编辑: notepad .env.prod
        pause
        exit /b 1
    ) else (
        echo %RED%[ERROR]%NC% 未找到 .env.example 文件
        exit /b 1
    )
)

echo %GREEN%[SUCCESS]%NC% 环境检查通过
goto :eof

:backup_data
echo %BLUE%[INFO]%NC% 备份现有数据...

set BACKUP_DIR=backup\%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%
set BACKUP_DIR=%BACKUP_DIR: =0%
if not exist "backup" mkdir backup
mkdir "%BACKUP_DIR%" 2>nul

REM 备份数据库
docker ps | findstr "doc-center-mysql" >nul
if not errorlevel 1 (
    echo %BLUE%[INFO]%NC% 备份 MySQL 数据...
    for /f "tokens=2 delims==" %%i in ('findstr "MYSQL_ROOT_PASSWORD" .env.prod') do set mysql_pass=%%i
    docker exec doc-center-mysql mysqladump -u root -p!mysql_pass! document_center_db > "%BACKUP_DIR%\database.sql"
)

REM 备份上传文件
if exist "uploads" (
    echo %BLUE%[INFO]%NC% 备份上传文件...
    xcopy /E /I "uploads" "%BACKUP_DIR%\uploads" >nul 2>&1
)

echo %GREEN%[SUCCESS]%NC% 数据备份完成: %BACKUP_DIR%
goto :eof

:deploy_services
echo %BLUE%[INFO]%NC% 开始部署服务...

REM 停止现有服务
if "%STOP_EXISTING%"=="true" (
    echo %BLUE%[INFO]%NC% 停止现有服务...
    docker-compose -f docker-compose.prod.yml down
)

REM 清理旧的镜像
if "%CLEAN_IMAGES%"=="true" (
    echo %BLUE%[INFO]%NC% 清理旧镜像...
    docker image prune -f
    docker volume prune -f
)

REM 构建并启动服务
echo %BLUE%[INFO]%NC% 构建和启动服务...
docker-compose -f docker-compose.prod.yml up -d --build
if errorlevel 1 (
    echo %RED%[ERROR]%NC% 服务启动失败
    exit /b 1
)

REM 等待服务启动
echo %BLUE%[INFO]%NC% 等待服务启动...
timeout /t 30 /nobreak >nul

REM 检查服务状态
call :check_services_health

goto :eof

:check_services_health
echo %BLUE%[INFO]%NC% 检查服务健康状态...

set services=doc-center-mysql doc-center-redis doc-center-kkfileview doc-center-app doc-center-nginx

for %%s in (%services%) do (
    docker ps | findstr "%%s" >nul
    if not errorlevel 1 (
        echo %GREEN%[SUCCESS]%NC% ✓ %%s 运行正常
    ) else (
        echo %RED%[ERROR]%NC% ✗ %%s 未运行
        echo 最近日志:
        docker logs "%%s" --tail 20
    )
)

REM 检查应用接口
echo %BLUE%[INFO]%NC% 检查应用接口...
curl -f http://localhost/health >nul 2>&1
if not errorlevel 1 (
    echo %GREEN%[SUCCESS]%NC% ✓ 应用健康检查通过
) else (
    echo %YELLOW%[WARNING]%NC% ⚠ 应用健康检查失败，可能还在启动中
)

goto :eof

:show_deployment_info
echo.
echo %GREEN%[SUCCESS]%NC% 部署完成！
echo.
echo ===================================
echo        部署信息
echo ===================================
echo 应用访问地址: http://localhost
echo API接口地址:  http://localhost/api
echo 文件预览地址: http://localhost/preview
echo.
echo 默认管理员账户:
echo 用户名: admin
echo 密码:   admin123
echo.
echo 常用命令:
echo 查看服务状态: docker-compose -f docker-compose.prod.yml ps
echo 查看应用日志: docker logs doc-center-app
echo 停止所有服务: docker-compose -f docker-compose.prod.yml down
echo ===================================
goto :eof