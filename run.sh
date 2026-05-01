#!/bin/bash

# Quick build and run script
# This script builds and starts the development server

set -e

echo "🎵 Ranger de Song - Build & Run"
echo "==============================="
echo ""

# Check if dependencies are installed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node dependencies..."
    npm install
    echo ""
fi

if [ ! -d "src-tauri/target" ]; then
    echo "🦀 Building Rust backend..."
    cd src-tauri
    cargo build
    cd ..
    echo ""
fi

echo "🚀 Starting development server..."
echo ""
echo "Frontend: http://localhost:5173"
echo "Dev Tools: Ctrl+Shift+I (or Cmd+Option+I on macOS)"
echo ""
echo "Press Ctrl+C to stop"
echo ""

npm run dev
