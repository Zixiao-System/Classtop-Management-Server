#!/bin/bash
# ClassTop Management Server - 启动脚本
# 适用于 macOS/Linux 系统

set -e

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

echo -e "${BLUE}===========================================

${NC}"
echo -e "${BLUE}  ClassTop Management Server Launcher${NC}"
echo -e "${BLUE}===========================================${NC}\n"

# 检查 .env 文件
if [ ! -f .env ]; then
    echo -e "${RED}错误: .env 文件不存在${NC}"
    echo -e "${YELLOW}正在从 .env.example 创建 .env...${NC}"
    cp .env.example .env
    echo -e "${YELLOW}请编辑 .env 文件并配置以下内容：${NC}"
    echo "  - DATABASE_URL"
    echo "  - JWT_SECRET (使用: openssl rand -base64 32)"
    echo "  - CORS_ALLOWED_ORIGINS"
    echo -e "${RED}然后重新运行此脚本${NC}"
    exit 1
fi

# 检查 JWT_SECRET
if grep -q "your-secret-key-change-this-in-production" .env; then
    echo -e "${YELLOW}警告: 检测到默认的 JWT_SECRET${NC}"
    echo -e "${YELLOW}生成新的 JWT 密钥...${NC}"
    JWT_SECRET=$(openssl rand -base64 32)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s|JWT_SECRET=.*|JWT_SECRET=$JWT_SECRET|" .env
    else
        sed -i "s|JWT_SECRET=.*|JWT_SECRET=$JWT_SECRET|" .env
    fi
    echo -e "${GREEN}✓ JWT 密钥已更新${NC}\n"
fi

# 检查数据库连接
echo -e "${BLUE}[1/5] 检查数据库连接...${NC}"
if ! cargo run --bin classtop-management-server -- --help >/dev/null 2>&1; then
    echo -e "${RED}错误: 无法编译项目${NC}"
    exit 1
fi
echo -e "${GREEN}✓ 项目编译成功${NC}\n"

# 构建前端
echo -e "${BLUE}[2/5] 构建前端...${NC}"
cd frontend
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}安装前端依赖...${NC}"
    npm install
fi
npm run build
cd ..
echo -e "${GREEN}✓ 前端构建完成${NC}\n"

# 检查 Nginx
echo -e "${BLUE}[3/5] 检查 Nginx...${NC}"
if ! command -v nginx &> /dev/null; then
    echo -e "${YELLOW}警告: Nginx 未安装${NC}"
    echo -e "${YELLOW}在 macOS 上安装: brew install nginx${NC}"
    echo -e "${YELLOW}在 Ubuntu 上安装: sudo apt install nginx${NC}"
    echo ""
    read -p "是否继续不使用 Nginx？(y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    USE_NGINX=false
else
    echo -e "${GREEN}✓ Nginx 已安装${NC}"
    USE_NGINX=true
fi

# 配置 Nginx
if [ "$USE_NGINX" = true ]; then
    echo -e "\n${BLUE}[4/5] 配置 Nginx...${NC}"

    # 检测 Nginx 配置目录
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS (Homebrew)
        NGINX_CONF_DIR="/opt/homebrew/etc/nginx"
        if [ ! -d "$NGINX_CONF_DIR" ]; then
            NGINX_CONF_DIR="/usr/local/etc/nginx"
        fi
    else
        # Linux
        NGINX_CONF_DIR="/etc/nginx"
    fi

    if [ -d "$NGINX_CONF_DIR/sites-available" ]; then
        # Linux: 使用 sites-available/enabled 模式
        SITES_CONF="$NGINX_CONF_DIR/sites-available/classtop.conf"
        SITES_ENABLED="$NGINX_CONF_DIR/sites-enabled/classtop.conf"

        # 复制 server 块配置
        sudo cp nginx-site.conf "$SITES_CONF"
        sudo ln -sf "$SITES_CONF" "$SITES_ENABLED"

        echo -e "${GREEN}✓ Nginx 配置已安装到: $SITES_CONF${NC}"
    else
        # macOS: 直接替换主配置文件
        echo -e "${YELLOW}正在配置 Nginx...${NC}"
        sudo cp nginx.conf "$NGINX_CONF_DIR/nginx.conf"
        echo -e "${GREEN}✓ Nginx 配置已更新: $NGINX_CONF_DIR/nginx.conf${NC}"
    fi

    # 测试 Nginx 配置
    if sudo nginx -t 2>/dev/null; then
        echo -e "${GREEN}✓ Nginx 配置有效${NC}"

        # 重启 Nginx
        if [[ "$OSTYPE" == "darwin"* ]]; then
            brew services restart nginx
        else
            sudo systemctl restart nginx
        fi
        echo -e "${GREEN}✓ Nginx 已重启${NC}\n"
    else
        echo -e "${RED}Nginx 配置测试失败${NC}"
        echo -e "${YELLOW}请检查配置文件${NC}\n"
    fi
else
    echo -e "\n${BLUE}[4/5] 跳过 Nginx 配置${NC}\n"
fi

# 启动后端服务
echo -e "${BLUE}[5/5] 启动后端服务...${NC}"
echo -e "${GREEN}✓ 准备就绪${NC}\n"

echo -e "${BLUE}===========================================${NC}"
echo -e "${GREEN}服务启动信息：${NC}"
echo -e "  后端 API: http://localhost:8765"
if [ "$USE_NGINX" = true ]; then
    echo -e "  前端界面: http://localhost (Nginx)"
else
    echo -e "  前端界面: http://localhost:8765 (内置)"
fi
echo -e "  API 文档: http://localhost:8765/api/docs"
echo -e "${BLUE}===========================================${NC}\n"

echo -e "${YELLOW}按 Ctrl+C 停止服务${NC}\n"

# 启动后端
RUST_LOG=info cargo run --release
