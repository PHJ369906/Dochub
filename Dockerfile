# 运行时 Dockerfile（使用宿主机JDK）
FROM ubuntu:22.04

# 安装必要的工具
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# 从宿主机复制构建产物（在部署脚本中预先构建）
COPY backend/target/*.jar app.jar
COPY frontend/dist ./static

# 创建必要的目录
RUN mkdir -p uploads logs config && \
    chown -R appuser:appuser /app

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/actuator/health || exit 1

# 切换到非root用户
USER appuser

# 启动应用（动态检测Java路径）
CMD ["sh", "-c", "JAVA_CMD=/usr/bin/java; if [ -f /usr/bin/java ]; then JAVA_CMD=/usr/bin/java; elif command -v java >/dev/null 2>&1; then JAVA_CMD=java; else echo 'Java not found'; exit 1; fi; $JAVA_CMD -Xms512m -Xmx1g -Dspring.profiles.active=prod -Djava.security.egd=file:/dev/./urandom -jar app.jar"]