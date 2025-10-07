#!/bin/bash

# Build script for ClassTop Management Server
# This script builds both frontend and backend

set -e

echo "🏗️  Building ClassTop Management Server..."
echo ""

# Build frontend
echo "📦 Building frontend..."
cd frontend
npm install
npm run build
cd ..
echo "✅ Frontend built successfully!"
echo ""

# Build backend
echo "🦀 Building backend..."
cargo build --release
echo "✅ Backend built successfully!"
echo ""

echo "🎉 Build complete!"
echo ""
echo "To run the server:"
echo "  ./target/release/classtop-management-server"
echo ""
echo "Or in development mode:"
echo "  cargo run"
