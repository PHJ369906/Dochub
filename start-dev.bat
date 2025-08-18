@echo off
REM Windows 开发环境启动脚本
chcp 65001 >nul

echo ========================================
echo        开发环境启动脚本
echo ========================================
echo.

REM 检查 Docker 环境
docker --version >nul 2>&1
if errorlevel 1 (
    echo [WARNING] Docker 未安装，将跳过容器服务启动
    set SKIP_DOCKER=true
) else (
    docker info >nul 2>&1
    if errorlevel 1 (
        echo [WARNING] Docker 未运行，将跳过容器服务启动
        set SKIP_DOCKER=true
    ) else (
        echo [INFO] Docker 环境正常
        set SKIP_DOCKER=false
    )
)

REM 启动依赖服务
if "%SKIP_DOCKER%"=="false" (
    echo [INFO] 启动 MySQL 和 Redis 服务...
    docker-compose up -d mysql redis
    
    echo [INFO] 等待服务启动...
    timeout /t 20 /nobreak >nul
    
    REM 检查服务状态
    docker ps | findstr "mysql\|redis" >nul
    if not errorlevel 1 (
        echo [SUCCESS] 数据库和缓存服务启动成功
    ) else (
        echo [WARNING] 部分服务启动异常，请检查 Docker 日志
    )
) else (
    echo [INFO] 跳过 Docker 服务启动
    echo [INFO] 请确保 MySQL 和 Redis 已手动启动
)

echo.
echo 选择启动方式:
echo 1. 启动后端服务 (Spring Boot)
echo 2. 启动前端服务 (Vue3 + Vite)  
echo 3. 同时启动前后端 (推荐)
echo 4. 仅显示启动命令
echo 5. 退出
set /p choice="请输入选择 (1-5): "

if "%choice%"=="1" goto start_backend
if "%choice%"=="2" goto start_frontend  
if "%choice%"=="3" goto start_both
if "%choice%"=="4" goto show_commands
if "%choice%"=="5" goto :eof
echo [ERROR] 无效选择
pause
exit /b 1

:start_backend
echo.
echo [INFO] 启动后端服务...
echo [INFO] 进入 backend 目录
cd backend
echo [INFO] 执行: mvn spring-boot:run
echo.
echo 后端启动成功标志:
echo - 看到 "Started SpringbootBackendApplication"  
echo - 访问 http://localhost:8080/doc.html 查看API文档
echo.
call mvn spring-boot:run
goto :eof

:start_frontend
echo.
echo [INFO] 启动前端服务...
echo [INFO] 进入 frontend 目录
cd frontend

REM 检查是否已安装依赖
if not exist "node_modules" (
    echo [INFO] 首次运行，安装前端依赖...
    echo [INFO] 使用国内镜像加速...
    call npm config set registry https://registry.npmmirror.com/
    call npm install
    if errorlevel 1 (
        echo [ERROR] 依赖安装失败
        pause
        exit /b 1
    )
)

echo [INFO] 执行: npm run dev
echo.
echo 前端启动成功标志:
echo - 看到 "Local: http://localhost:3000"
echo - 浏览器自动打开登录页面
echo.
call npm run dev
goto :eof

:start_both
echo.
echo [INFO] 同时启动前后端服务...
echo [INFO] 将打开两个新的命令窗口

REM 启动后端
start "后端服务 - Spring Boot" cmd /k "cd /d "%~dp0backend" && echo [INFO] 启动后端服务... && mvn spring-boot:run"

REM 等待一下让后端先启动
timeout /t 5 /nobreak >nul

REM 启动前端
cd frontend
if not exist "node_modules" (
    echo [INFO] 安装前端依赖...
    call npm config set registry https://registry.npmmirror.com/
    call npm install
)

start "前端服务 - Vue3" cmd /k "cd /d "%~dp0frontend" && echo [INFO] 启动前端服务... && npm run dev"

echo [SUCCESS] 服务启动中...
echo.
echo 预计启动时间: 30-60秒
echo 后端服务: http://localhost:8080
echo 前端应用: http://localhost:3000  
echo API文档: http://localhost:8080/doc.html
echo.
echo 按任意键退出...
pause >nul
goto :eof

:show_commands
echo.
echo 常用启动命令:
echo.
echo === Docker 服务 ===
echo 启动依赖服务: docker-compose up -d
echo 查看服务状态: docker-compose ps
echo 停止服务:     docker-compose down
echo.
echo === 后端服务 ===
echo 进入目录: cd backend
echo 编译测试: mvn clean compile
echo 启动服务: mvn spring-boot:run
echo 调试模式: mvn spring-boot:run -Dspring-boot.run.jvmArguments="-Xdebug -Xrunjdwp:transport=dt_socket,server=y,suspend=n,address=5005"
echo.
echo === 前端服务 ===
echo 进入目录: cd frontend
echo 安装依赖: npm install
echo 启动服务: npm run dev
echo 构建打包: npm run build
echo 代码检查: npm run lint
echo.
echo === 访问地址 ===
echo 前端应用: http://localhost:3000
echo 后端API:  http://localhost:8080
echo API文档:  http://localhost:8080/doc.html
echo.
pause
goto :eof