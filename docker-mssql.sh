#!/bin/bash

# ================================================================
# SQL Server Docker 环境管理脚本
# ================================================================

set -e

COMPOSE_FILE="docker-compose.mssql.yml"
SA_PASSWORD="ClassTop@2024Dev!"
SERVER="localhost,1433"
DATABASE="classtop_dev"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# 启动 SQL Server
start_server() {
    print_info "Starting SQL Server container..."
    docker-compose -f "$COMPOSE_FILE" up -d

    print_info "Waiting for SQL Server to be ready..."
    for i in {1..30}; do
        if docker exec classtop-mssql-dev /opt/mssql-tools/bin/sqlcmd \
            -S localhost -U sa -P "$SA_PASSWORD" -Q "SELECT 1" &> /dev/null; then
            print_success "SQL Server is ready!"
            break
        fi
        echo -n "."
        sleep 2
    done

    print_info "\n📊 Connection Information:"
    echo "  Host: localhost"
    echo "  Port: 1433"
    echo "  User: sa"
    echo "  Password: $SA_PASSWORD"
    echo "  Database: $DATABASE"
    echo ""
    echo "  Connection String:"
    echo "  mssql://sa:$SA_PASSWORD@localhost:1433/$DATABASE"
}

# 停止 SQL Server
stop_server() {
    print_info "Stopping SQL Server container..."
    docker-compose -f "$COMPOSE_FILE" down
    print_success "SQL Server stopped."
}

# 重启 SQL Server
restart_server() {
    stop_server
    start_server
}

# 查看日志
logs() {
    docker-compose -f "$COMPOSE_FILE" logs -f sqlserver
}

# 执行 SQL 查询
query() {
    if [ -z "$1" ]; then
        print_error "Usage: ./docker-mssql.sh query \"SELECT 1\""
        exit 1
    fi

    docker exec -it classtop-mssql-dev /opt/mssql-tools/bin/sqlcmd \
        -S localhost -U sa -P "$SA_PASSWORD" -d "$DATABASE" -Q "$1"
}

# 进入交互式 SQL Shell
shell() {
    print_info "Entering SQL Server shell (type 'exit' to quit)..."
    docker exec -it classtop-mssql-dev /opt/mssql-tools/bin/sqlcmd \
        -S localhost -U sa -P "$SA_PASSWORD" -d "$DATABASE"
}

# 重新初始化数据库
reinit() {
    print_warning "This will drop and recreate the database. Continue? (y/N)"
    read -r confirm
    if [ "$confirm" != "y" ]; then
        print_info "Aborted."
        exit 0
    fi

    docker exec -it classtop-mssql-dev /opt/mssql-tools/bin/sqlcmd \
        -S localhost -U sa -P "$SA_PASSWORD" \
        -i /docker-entrypoint-initdb.d/init.sql

    print_success "Database reinitialized."
}

# 显示状态
status() {
    docker-compose -f "$COMPOSE_FILE" ps
}

# 清理所有数据（包括 volume）
clean() {
    print_warning "This will remove all data including volumes. Continue? (y/N)"
    read -r confirm
    if [ "$confirm" != "y" ]; then
        print_info "Aborted."
        exit 0
    fi

    docker-compose -f "$COMPOSE_FILE" down -v
    print_success "All data cleaned."
}

# 帮助信息
show_help() {
    cat << EOF
${GREEN}SQL Server Docker Management Script${NC}

Usage: ./docker-mssql.sh [command]

Commands:
  start       Start SQL Server container
  stop        Stop SQL Server container
  restart     Restart SQL Server container
  logs        Show SQL Server logs (follow mode)
  status      Show container status
  query       Execute a SQL query
              Example: ./docker-mssql.sh query "SELECT * FROM test_types"
  shell       Enter interactive SQL shell
  reinit      Reinitialize database (run init.sql again)
  clean       Remove all data including volumes
  help        Show this help message

Connection Info:
  Server: localhost,1433
  User: sa
  Password: $SA_PASSWORD
  Database: $DATABASE

EOF
}

# 主逻辑
case "${1:-help}" in
    start)
        start_server
        ;;
    stop)
        stop_server
        ;;
    restart)
        restart_server
        ;;
    logs)
        logs
        ;;
    status)
        status
        ;;
    query)
        query "$2"
        ;;
    shell)
        shell
        ;;
    reinit)
        reinit
        ;;
    clean)
        clean
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
