#!/bin/bash

# Build script for Fedora 43 with webkit2gtk4.1 and libsoup 3

set -e

echo "🎵 Ranger de Song - Building for Fedora 43 (webkit2gtk4.1 + libsoup3)"
echo "========================================================================="

# Clean Cargo cache
echo "🧹 Cleaning Cargo cache..."
cd src-tauri
rm -rf target Cargo.lock
cd ..

# Update package lock
echo "🧹 Cleaning npm cache..."
rm -rf node_modules package-lock.json

# Install dependencies
echo "📦 Installing Node dependencies..."
npm install

# Build Rust backend
echo "🦀 Building Rust backend with webkit2gtk4.1..."
cd src-tauri
cargo build
cd ..

echo "✅ Build complete!"
echo ""
echo "To start development, run:"
echo "  npm run dev"
