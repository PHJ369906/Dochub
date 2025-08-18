#!/bin/bash

# Docker 部署脚本
set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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
        log_error "$1 未安装，请先安装 $1"
        exit 1
    fi
}

# 检查环境
check_environment() {
    log_info "检查部署环境..."
    
    check_command "docker"
    check_command "docker-compose"
    
    # 检查Docker是否运行
    if ! docker info &> /dev/null; then
        log_error "Docker 未运行，请启动 Docker 服务"
        exit 1
    fi
    
    # 检查环境变量文件
    if [ ! -f ".env.prod" ]; then
        log_warning ".env.prod 文件不存在"
        if [ -f ".env.example" ]; then
            log_info "从 .env.example 创建 .env.prod"
            cp .env.example .env.prod
            log_warning "请编辑 .env.prod 文件并设置正确的环境变量"
            exit 1
        else
            log_error "未找到 .env.example 文件"
            exit 1
        fi
    fi
    
    log_success "环境检查通过"
}

# 备份数据
backup_data() {
    if [ "$BACKUP" == "true" ]; then
        log_info "备份现有数据..."
        
        BACKUP_DIR="backup/$(date +%Y%m%d_%H%M%S)"
        mkdir -p "$BACKUP_DIR"
        
        # 备份数据库
        if docker ps | grep -q "doc-center-mysql"; then
            log_info "备份 MySQL 数据..."
            docker exec doc-center-mysql mysqldump -u root -p"$(grep MYSQL_ROOT_PASSWORD .env.prod | cut -d '=' -f2)" document_center_db > "$BACKUP_DIR/database.sql"
        fi
        
        # 备份上传文件
        if [ -d "uploads" ]; then
            log_info "备份上传文件..."
            cp -r uploads "$BACKUP_DIR/"
        fi
        
        log_success "数据备份完成: $BACKUP_DIR"
    fi
}

# 部署服务
deploy_services() {
    log_info "开始部署服务..."
    
    # 加载环境变量
    export $(grep -v '^#' .env.prod | xargs)
    
    # 停止现有服务
    if [ "$STOP_EXISTING" == "true" ]; then
        log_info "停止现有服务..."
        docker-compose -f docker-compose.prod.yml down
    fi
    
    # 清理旧的镜像（可选）
    if [ "$CLEAN_IMAGES" == "true" ]; then
        log_info "清理旧镜像..."
        docker image prune -f
        docker volume prune -f
    fi
    
    # 构建并启动服务
    log_info "构建和启动服务..."
    docker-compose -f docker-compose.prod.yml up -d --build
    
    # 等待服务启动
    log_info "等待服务启动..."
    sleep 30
    
    # 检查服务状态
    check_services_health
}

# 检查服务健康状态
check_services_health() {
    log_info "检查服务健康状态..."
    
    services=("doc-center-mysql" "doc-center-redis" "doc-center-kkfileview" "doc-center-app" "doc-center-nginx")
    
    for service in "${services[@]}"; do
        if docker ps | grep -q "$service"; then
            log_success "✓ $service 运行正常"
        else
            log_error "✗ $service 未运行"
            docker logs "$service" --tail 20
        fi
    done
    
    # 检查应用接口
    log_info "检查应用接口..."
    if curl -f http://localhost/health &> /dev/null; then
        log_success "✓ 应用健康检查通过"
    else
        log_warning "⚠ 应用健康检查失败"
    fi
}

# 显示部署信息
show_deployment_info() {
    log_success "部署完成！"
    echo
    echo "==================================="
    echo "       部署信息"
    echo "==================================="
    echo "应用访问地址: http://localhost"
    echo "API接口地址:  http://localhost/api"
    echo "文件预览地址: http://localhost/preview"
    echo
    echo "默认管理员账户:"
    echo "用户名: admin"
    echo "密码:   admin123"
    echo
    echo "查看服务状态: docker-compose -f docker-compose.prod.yml ps"
    echo "查看应用日志: docker logs doc-center-app"
    echo "停止所有服务: docker-compose -f docker-compose.prod.yml down"
    echo "==================================="
}

# 主函数
main() {
    # 默认参数
    BACKUP=false
    STOP_EXISTING=true
    CLEAN_IMAGES=false
    
    # 解析参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --backup)
                BACKUP=true
                shift
                ;;
            --no-stop)
                STOP_EXISTING=false
                shift
                ;;
            --clean)
                CLEAN_IMAGES=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --backup      备份现有数据"
                echo "  --no-stop     不停止现有服务"
                echo "  --clean       清理旧的镜像和卷"
                echo "  --help, -h    显示帮助信息"
                exit 0
                ;;
            *)
                log_error "未知参数: $1"
                exit 1
                ;;
        esac
    done
    
    log_info "开始 Docker 部署流程..."
    
    check_environment
    backup_data
    deploy_services
    show_deployment_info
}

# 脚本入口
main "$@"