# 运行时 Dockerfile（宿主机预编译版本）
FROM registry.cn-hangzhou.aliyuncs.com/library/openjdk:17-jre-slim

# 安装必要的工具和字体
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    fontconfig \
    ttf-dejavu \
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

# 设置JVM参数
ENV JAVA_OPTS="-Xms512m -Xmx1g -Dspring.profiles.active=prod -Djava.security.egd=file:/dev/./urandom"

# 启动应用
CMD ["sh", "-c", "java $JAVA_OPTS -jar app.jar"]