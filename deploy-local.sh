#!/bin/bash

# 本地构建部署脚本
# 用途：在服务器上本地构建前后端，避免网络问题

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查命令是否存在
check_command() {
    if ! command -v $1 &> /dev/null; then
        log_error "$1 未安装"
        return 1
    fi
    return 0
}

# 安装Node.js
install_nodejs() {
    log_info "检查Node.js安装状态..."
    if ! check_command node; then
        log_info "安装Node.js 18..."
        curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -
        sudo yum install -y nodejs
    fi
    
    node_version=$(node -v)
    npm_version=$(npm -v)
    log_success "Node.js版本: $node_version, npm版本: $npm_version"
    
    # 配置npm镜像源
    npm config set registry https://registry.npmmirror.com/
}

# 检查Java和Maven
check_java_maven() {
    log_info "检查Java和Maven..."
    
    if ! check_command java; then
        log_error "Java未安装，请先安装Java 17"
        exit 1
    fi
    
    if ! check_command mvn; then
        log_error "Maven未安装，请先安装Maven"
        exit 1
    fi
    
    # 检查Java版本
    java_version=$(java -version 2>&1 | head -n 1)
    log_info "Java版本: $java_version"
    
    # 如果不是Java 17，尝试切换
    if [[ ! $java_version == *"17"* ]]; then
        log_warning "当前不是Java 17，尝试切换..."
        if [ -d "/usr/lib/jvm/java-17-openjdk"* ]; then
            export JAVA_HOME=$(ls -d /usr/lib/jvm/java-17-openjdk* | head -1)
            export PATH=$JAVA_HOME/bin:$PATH
            log_info "已切换到Java 17: $JAVA_HOME"
        else
            log_error "未找到Java 17，请先安装"
            exit 1
        fi
    fi
}

# 检查Docker
check_docker() {
    log_info "检查Docker..."
    if ! check_command docker; then
        log_error "Docker未安装，请先安装Docker"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        log_error "Docker未运行，请启动Docker服务"
        exit 1
    fi
    
    if ! check_command docker-compose; then
        log_error "Docker Compose未安装"
        exit 1
    fi
}

# 检查并启动数据库服务
check_database_services() {
    log_info "检查数据库服务状态..."
    
    # 检查MySQL容器
    if docker ps | grep -q "doc-center-mysql.*Up"; then
        log_success "MySQL容器已运行"
    elif docker ps -a | grep -q "doc-center-mysql"; then
        log_info "启动已存在的MySQL容器..."
        docker start doc-center-mysql
    else
        log_info "MySQL容器不存在，将通过docker-compose创建"
    fi
    
    # 检查Redis容器
    if docker ps | grep -q "doc-center-redis.*Up"; then
        log_success "Redis容器已运行"
    elif docker ps -a | grep -q "doc-center-redis"; then
        log_info "启动已存在的Redis容器..."
        docker start doc-center-redis
    else
        log_info "Redis容器不存在，将通过docker-compose创建"
    fi
    
    # 仅在需要清理数据时删除卷
    if [ "$CLEAN_DATA" == "true" ]; then
        log_warning "清理数据卷..."
        docker volume rm $(docker volume ls -q | grep -E "(mysql_data|redis_data)") 2>/dev/null || true
    fi
}

# 构建前端
build_frontend() {
    log_info "构建前端项目..."
    
    if [ ! -d "frontend" ]; then
        log_error "前端目录不存在"
        exit 1
    fi
    
    cd frontend
    
    # 安装依赖
    log_info "安装前端依赖..."
    npm install
    
    # 构建
    log_info "构建前端..."
    # 跳过TypeScript检查，直接构建
    npx vite build
    
    # 检查构建结果
    if [ ! -d "dist" ] || [ ! -f "dist/index.html" ]; then
        log_error "前端构建失败"
        exit 1
    fi
    
    log_success "前端构建完成"
    ls -la dist/
    
    cd ..
}

# 构建后端
build_backend() {
    log_info "构建后端项目..."
    
    if [ ! -d "backend" ]; then
        log_error "后端目录不存在"
        exit 1
    fi
    
    cd backend
    
    # 配置Maven使用阿里云镜像
    mkdir -p ~/.m2
    cat > ~/.m2/settings.xml << 'EOF'
<settings>
  <mirrors>
    <mirror>
      <id>aliyunmaven</id>
      <mirrorOf>*</mirrorOf>
      <name>阿里云公共仓库</name>
      <url>https://maven.aliyun.com/repository/public</url>
    </mirror>
  </mirrors>
</settings>
EOF
    
    # 构建
    log_info "构建后端..."
    mvn clean package -DskipTests
    
    # 检查构建结果
    if [ ! -f target/*.jar ]; then
        log_error "后端构建失败"
        exit 1
    fi
    
    log_success "后端构建完成"
    ls -la target/*.jar
    
    cd ..
}

# 创建Dockerfile
create_dockerfile() {
    log_info "创建Dockerfile..."
    
    cat > Dockerfile << 'EOF'
FROM openjdk:17-jdk-slim

WORKDIR /app

# 安装curl用于健康检查
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# 复制后端jar包
COPY backend/target/*.jar app.jar

# 复制前端构建产物
COPY frontend/dist ./static

# 创建必要目录
RUN mkdir -p uploads logs config

# 设置文件权限
RUN chmod 755 /app && \
    chown -R 1000:1000 /app

EXPOSE 8080

# 设置JVM参数
ENV JAVA_OPTS="-Xms512m -Xmx1g -Dspring.profiles.active=prod -Djava.security.egd=file:/dev/./urandom"

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/actuator/health || exit 1

# 启动应用
CMD ["sh", "-c", "java $JAVA_OPTS -jar app.jar"]
EOF
    
    log_success "Dockerfile创建完成"
}

# 创建Docker Compose配置
create_docker_compose() {
    log_info "创建Docker Compose配置..."
    
    cat > docker-compose.local.yml << 'EOF'
services:
  # MySQL数据库
  mysql:
    image: mysql:8.0
    container_name: doc-center-mysql
    restart: unless-stopped
    pull_policy: if_not_present
    environment:
      MYSQL_ROOT_PASSWORD: 123456
      MYSQL_DATABASE: document_center_db
      MYSQL_USER: docuser
      MYSQL_PASSWORD: 123456
      MYSQL_CHARSET: utf8mb4
      MYSQL_COLLATION: utf8mb4_unicode_ci
    ports:
      - "3306:3306"
    volumes:
      - mysql_data:/var/lib/mysql
      - ./sql:/docker-entrypoint-initdb.d
    command: >
      --default-authentication-plugin=mysql_native_password
      --character-set-server=utf8mb4
      --collation-server=utf8mb4_unicode_ci
    networks:
      - doc-center-network
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost", "-u", "root", "-p123456"]
      timeout: 20s
      retries: 10
      interval: 30s

  # Redis缓存
  redis:
    image: redis:7-alpine
    container_name: doc-center-redis
    restart: unless-stopped
    pull_policy: if_not_present
    command: redis-server --appendonly yes --requirepass "123456"
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    networks:
      - doc-center-network
    healthcheck:
      test: ["CMD", "redis-cli", "-a", "123456", "ping"]
      interval: 30s
      timeout: 10s
      retries: 3

  # 主应用服务
  app:
    image: doc-center-app:latest
    container_name: doc-center-app
    restart: unless-stopped
    pull_policy: never
    ports:
      - "8080:8080"
    environment:
      - SPRING_PROFILES_ACTIVE=prod
      - SPRING_DATASOURCE_URL=jdbc:mysql://mysql:3306/document_center_db?useUnicode=true&characterEncoding=utf8&useSSL=false&serverTimezone=Asia/Shanghai&allowPublicKeyRetrieval=true
      - SPRING_DATASOURCE_USERNAME=docuser
      - SPRING_DATASOURCE_PASSWORD=123456
      - SPRING_DATA_REDIS_HOST=redis
      - SPRING_DATA_REDIS_PORT=6379
      - SPRING_DATA_REDIS_PASSWORD=123456
      - SA_TOKEN_JWT_SECRET_KEY=DocCenter2025JwtSecretKey32Chars
    volumes:
      - app_uploads:/app/uploads
      - app_logs:/app/logs
    depends_on:
      mysql:
        condition: service_healthy
      redis:
        condition: service_healthy
    networks:
      - doc-center-network

  # Nginx 反向代理
  nginx:
    image: nginx:alpine
    container_name: doc-center-nginx
    restart: unless-stopped
    pull_policy: if_not_present
    ports:
      - "80:80"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/conf.d:/etc/nginx/conf.d:ro
      - nginx_logs:/var/log/nginx
    depends_on:
      - app
    networks:
      - doc-center-network
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  mysql_data:
    driver: local
  redis_data:
    driver: local
  app_uploads:
    driver: local
  app_logs:
    driver: local
  nginx_logs:
    driver: local

networks:
  doc-center-network:
    driver: bridge
EOF
    
    log_success "Docker Compose配置创建完成"
}

# 构建Docker镜像
build_docker_image() {
    log_info "构建Docker镜像..."
    
    # 删除旧镜像
    if docker images | grep -q "doc-center-app"; then
        log_info "删除旧镜像..."
        docker rmi doc-center-app:latest 2>/dev/null || true
    fi
    
    # 构建新镜像
    docker build -t doc-center-app:latest .
    
    log_success "Docker镜像构建完成"
    docker images | grep doc-center-app
}

# 启动服务
start_services() {
    log_info "启动所有服务..."
    
    # 只停止应用服务，保留数据库服务
    if docker ps | grep -q "doc-center-app"; then
        log_info "停止现有应用服务..."
        docker stop doc-center-app 2>/dev/null || true
        docker rm doc-center-app 2>/dev/null || true
    fi
    
    if docker ps | grep -q "doc-center-nginx"; then
        log_info "停止现有Nginx服务..."
        docker stop doc-center-nginx 2>/dev/null || true
        docker rm doc-center-nginx 2>/dev/null || true
    fi
    
    # 启动服务
    docker-compose -f docker-compose.local.yml up -d
    
    log_info "等待服务启动..."
    sleep 30
    
    # 检查服务状态
    check_services_health
}

# 检查服务健康状态
check_services_health() {
    log_info "检查服务健康状态..."
    
    # 显示服务状态
    docker-compose -f docker-compose.local.yml ps
    
    # 检查每个服务
    services=("doc-center-mysql" "doc-center-redis" "doc-center-app" "doc-center-nginx")
    
    for service in "${services[@]}"; do
        if docker ps | grep -q "$service.*Up"; then
            log_success "✓ $service 运行正常"
        else
            log_error "✗ $service 运行异常"
            log_info "查看 $service 日志:"
            docker logs "$service" --tail 10
        fi
    done
    
    # 测试应用访问
    log_info "测试应用访问..."
    sleep 10
    
    if curl -f http://localhost/health &> /dev/null; then
        log_success "✓ 应用健康检查通过"
    else
        log_warning "⚠ 应用健康检查失败，查看应用日志:"
        docker logs doc-center-app --tail 20
    fi
}

# 显示部署信息
show_deployment_info() {
    local server_ip=$(curl -s ifconfig.me 2>/dev/null || echo "localhost")
    
    log_success "==================================="
    log_success "       部署完成！"
    log_success "==================================="
    echo
    echo "📱 访问地址:"
    echo "   主应用:     http://$server_ip"
    echo "   API接口:    http://$server_ip/api"
    echo "   健康检查:   http://$server_ip/health"
    echo
    echo "🔐 默认账户:"
    echo "   管理员:     admin / admin123"
    echo "   普通用户:   user / user123"
    echo "   演示账户:   demo / demo123"
    echo
    echo "🗄️ 数据库信息:"
    echo "   MySQL:      mysql:3306"
    echo "   数据库:     document_center_db"
    echo "   用户名:     docuser"
    echo "   密码:       123456"
    echo
    echo "📊 管理命令:"
    echo "   查看状态:   docker-compose -f docker-compose.local.yml ps"
    echo "   查看日志:   docker logs doc-center-app"
    echo "   重启服务:   docker-compose -f docker-compose.local.yml restart app"
    echo "   停止服务:   docker-compose -f docker-compose.local.yml down"
    echo
    log_success "==================================="
}

# 主函数
main() {
    # 解析参数
    CLEAN_DATA=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --clean-data)
                CLEAN_DATA=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --clean-data  清理数据库和Redis数据"
                echo "  --help, -h    显示帮助信息"
                exit 0
                ;;
            *)
                log_error "未知参数: $1"
                exit 1
                ;;
        esac
    done
    
    log_info "开始本地构建部署流程..."
    
    # 执行部署步骤
    check_docker
    check_java_maven
    install_nodejs
    check_database_services
    build_frontend
    build_backend
    create_dockerfile
    create_docker_compose
    build_docker_image
    start_services
    show_deployment_info
}

# 脚本入口
main "$@"