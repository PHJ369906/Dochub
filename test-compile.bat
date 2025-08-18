@echo off
REM Windows 编译测试脚本
chcp 65001 >nul

echo ========================================
echo          编译测试脚本
echo ========================================
echo.

REM 检查 Java 环境
java -version 2>&1 | findstr "version" >nul
if errorlevel 1 (
    echo [ERROR] Java 未安装或未配置PATH
    echo 请安装 JDK 17+
    pause
    exit /b 1
)

echo [INFO] Java 环境检查通过

REM 检查 Maven 环境
mvn -version 2>&1 | findstr "Apache Maven" >nul
if errorlevel 1 (
    echo [ERROR] Maven 未安装或未配置PATH
    echo 请安装 Maven 3.6+
    pause
    exit /b 1
)

echo [INFO] Maven 环境检查通过

REM 检查项目结构
if not exist "backend\pom.xml" (
    echo [ERROR] 未找到后端项目文件 backend\pom.xml
    echo 请确保在项目根目录下运行此脚本
    pause
    exit /b 1
)

echo [INFO] 项目结构检查通过
echo.

REM 进入后端目录
cd backend

echo [INFO] 开始编译测试...
echo [INFO] 清理并编译项目...

REM 执行 Maven 编译
call mvn clean compile -q
if errorlevel 1 (
    echo.
    echo [ERROR] 编译失败！
    echo.
    echo 常见解决方案:
    echo 1. 检查网络连接，Maven 需要下载依赖
    echo 2. 配置国内 Maven 镜像源
    echo 3. 检查 JDK 版本是否为 17+
    echo.
    echo 配置 Maven 镜像源:
    echo 1. 编辑 %%USERPROFILE%%\.m2\settings.xml
    echo 2. 添加阿里云镜像配置
    echo.
    pause
    exit /b 1
)

echo [SUCCESS] 编译成功！
echo.

REM 检查关键类文件
echo [INFO] 检查编译产物...

set CLASSES_DIR=target\classes\com\example
if exist "%CLASSES_DIR%\SpringbootBackendApplication.class" (
    echo [SUCCESS] ✓ 主程序类编译成功
) else (
    echo [WARNING] ⚠ 主程序类编译异常
)

if exist "%CLASSES_DIR%\controller" (
    echo [SUCCESS] ✓ 控制器类编译成功
) else (
    echo [WARNING] ⚠ 控制器类编译异常
)

if exist "%CLASSES_DIR%\service" (
    echo [SUCCESS] ✓ 服务类编译成功
) else (
    echo [WARNING] ⚠ 服务类编译异常
)

echo.
echo [INFO] 编译测试完成！
echo.
echo 下一步:
echo 1. 启动数据库和Redis
echo 2. 运行: mvn spring-boot:run
echo 3. 访问: http://localhost:8080

cd ..
echo.
pause