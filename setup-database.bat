@echo off
REM Windows 数据库初始化脚本
chcp 65001 >nul
setlocal EnableDelayedExpansion

echo ========================================
echo          数据库初始化脚本
echo ========================================
echo.

REM 检查 MySQL 是否安装
mysql --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] MySQL 未安装或未在PATH中，请安装 MySQL 8.0+
    echo.
    echo 可选方案:
    echo 1. 安装 MySQL Server: https://dev.mysql.com/downloads/mysql/
    echo 2. 使用 Docker: docker-compose up -d mysql
    pause
    exit /b 1
)

echo [INFO] 检测到 MySQL 已安装

REM 获取数据库连接信息
set /p DB_HOST="请输入 MySQL 主机地址 (默认: localhost): "
if "%DB_HOST%"=="" set DB_HOST=localhost

set /p DB_PORT="请输入 MySQL 端口 (默认: 3306): "
if "%DB_PORT%"=="" set DB_PORT=3306

set /p DB_ROOT_USER="请输入 MySQL root 用户名 (默认: root): "
if "%DB_ROOT_USER%"=="" set DB_ROOT_USER=root

echo.
echo [INFO] 连接信息:
echo   主机: %DB_HOST%
echo   端口: %DB_PORT%
echo   用户: %DB_ROOT_USER%
echo.

REM 测试数据库连接
echo [INFO] 测试数据库连接...
mysql -h %DB_HOST% -P %DB_PORT% -u %DB_ROOT_USER% -p -e "SELECT 1;" >nul 2>&1
if errorlevel 1 (
    echo [ERROR] 数据库连接失败，请检查:
    echo 1. MySQL 服务是否启动
    echo 2. 连接信息是否正确
    echo 3. 密码是否正确
    pause
    exit /b 1
)

echo [SUCCESS] 数据库连接成功

REM 选择初始化方式
echo.
echo 请选择初始化方式:
echo 1. 完整初始化 (包含示例数据) - 推荐
echo 2. 仅初始化表结构
echo 3. 取消
set /p choice="请输入选择 (1-3): "

if "%choice%"=="1" (
    set SQL_FILE=sql\init.sql
    echo [INFO] 执行完整数据库初始化...
) else if "%choice%"=="2" (
    set SQL_FILE=sql\schema.sql
    echo [INFO] 执行数据库结构初始化...
) else if "%choice%"=="3" (
    echo [INFO] 取消初始化
    exit /b 0
) else (
    echo [ERROR] 无效选择
    pause
    exit /b 1
)

REM 检查 SQL 文件是否存在
if not exist "%SQL_FILE%" (
    echo [ERROR] SQL 文件不存在: %SQL_FILE%
    echo 请确保在项目根目录下运行此脚本
    pause
    exit /b 1
)

REM 执行 SQL 脚本
echo [INFO] 执行 SQL 脚本: %SQL_FILE%
mysql -h %DB_HOST% -P %DB_PORT% -u %DB_ROOT_USER% -p < "%SQL_FILE%"
if errorlevel 1 (
    echo [ERROR] SQL 脚本执行失败
    pause
    exit /b 1
)

echo.
echo [SUCCESS] 数据库初始化完成！
echo.
echo 数据库信息:
echo   数据库名: document_center_db
echo   字符集: utf8mb4
echo   排序规则: utf8mb4_unicode_ci

if "%choice%"=="1" (
    echo.
    echo 默认测试账户:
    echo   管理员: admin / admin123
    echo   普通用户: user / user123
    echo   演示用户: demo / demo123
)

echo.
echo 下一步:
echo 1. 启动 Redis: docker run -d --name redis -p 6379:6379 redis:7-alpine
echo 2. 启动后端: cd backend ^&^& mvn spring-boot:run
echo 3. 启动前端: cd frontend ^&^& npm run dev

echo.
pause