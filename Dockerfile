# 多阶段构建 Dockerfile
# 第一阶段：构建前端
FROM node:18-alpine AS frontend-builder

# 设置镜像源以加快下载速度
RUN npm config set registry https://registry.npmmirror.com/

WORKDIR /app/frontend

# 复制前端依赖文件
COPY frontend/package*.json ./

# 安装依赖（包含开发依赖，因为构建需要）
RUN npm ci

# 复制前端源码
COPY frontend/ ./

# 设置构建环境变量
ARG API_BASE_URL=http://localhost:8080
ENV VITE_API_BASE_URL=${API_BASE_URL}

# 构建前端
RUN npm run build

# 第二阶段：构建后端
FROM maven:3.9-openjdk-17 AS backend-builder

# 设置Maven镜像源
RUN mkdir -p /root/.m2 && \
    echo '<settings><mirrors><mirror><id>aliyunmaven</id><mirrorOf>*</mirrorOf><name>阿里云公共仓库</name><url>https://maven.aliyun.com/repository/public</url></mirror></mirrors></settings>' > /root/.m2/settings.xml

WORKDIR /app/backend

# 复制Maven配置文件
COPY backend/pom.xml ./

# 下载依赖
RUN mvn dependency:go-offline -B

# 复制后端源码
COPY backend/src ./src

# 构建后端
RUN mvn clean package -DskipTests -B

# 第三阶段：运行时镜像
FROM openjdk:17-jre-slim

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

# 从构建阶段复制构建产物
COPY --from=backend-builder /app/backend/target/*.jar app.jar
COPY --from=frontend-builder /app/frontend/dist ./static

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