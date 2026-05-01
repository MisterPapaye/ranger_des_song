#!/bin/bash

# Setup script for Ranger de Song development environment

set -e

echo "🎵 Ranger de Song - Development Setup"
echo "=========================================="

# Check prerequisites
echo "✓ Checking prerequisites..."

if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install from https://nodejs.org/"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Please install from https://rustup.rs/"
    exit 1
fi

echo "✓ Node.js $(node --version)"
echo "✓ Rust $(rustc --version)"

# Install Node dependencies
echo ""
echo "📦 Installing Node dependencies..."
npm install

# Build Rust
echo ""
echo "🦀 Building Rust backend..."
cd src-tauri
cargo build
cd ..

echo ""
echo "✅ Setup complete!"
echo ""
echo "To start development, run:"
echo "  npm run dev"
echo ""
echo "To build for production, run:"
echo "  npm run build"
echo ""
