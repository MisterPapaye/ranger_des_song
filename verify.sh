#!/usr/bin/env bash

# Verification script for Ranger de Song project setup
# This script checks that all required files and directories are in place

echo "🔍 Ranger de Song - Project Structure Verification"
echo "=================================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1"
        return 0
    else
        echo -e "${RED}✗${NC} Missing: $1"
        return 1
    fi
}

check_dir() {
    if [ -d "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1/"
        return 0
    else
        echo -e "${RED}✗${NC} Missing: $1/"
        return 1
    fi
}

files_ok=0
dirs_ok=0
files_total=0
dirs_total=0

echo "📁 Checking directories..."
for dir in "src" "src-tauri" "src/node_modules" "src-tauri/src/audio" "src-tauri/src/api" "src-tauri/src/models" ".cargo"; do
    dirs_total=$((dirs_total + 1))
    if check_dir "$dir"; then
        dirs_ok=$((dirs_ok + 1))
    fi
done

echo ""
echo "📄 Checking configuration files..."
for file in "package.json" "tsconfig.json" "vite.config.ts" "index.html" "src-tauri/Cargo.toml" "src-tauri/tauri.conf.json" "src-tauri/build.rs"; do
    files_total=$((files_total + 1))
    if check_file "$file"; then
        files_ok=$((files_ok + 1))
    fi
done

echo ""
echo "🦀 Checking Rust source files..."
rust_files=("src-tauri/src/main.rs" "src-tauri/src/lib.rs" "src-tauri/src/commands.rs" "src-tauri/src/audio/mod.rs" "src-tauri/src/audio/metadata.rs" "src-tauri/src/audio/processor.rs" "src-tauri/src/api/mod.rs" "src-tauri/src/api/musicbrainz.rs" "src-tauri/src/models/mod.rs")
for file in "${rust_files[@]}"; do
    files_total=$((files_total + 1))
    if check_file "$file"; then
        files_ok=$((files_ok + 1))
    fi
done

echo ""
echo "⚛️  Checking React/TypeScript files..."
for file in "src/main.tsx" "src/App.tsx" "src/App.css" "src/index.css" "src/vite-env.d.ts"; do
    files_total=$((files_total + 1))
    if check_file "$file"; then
        files_ok=$((files_ok + 1))
    fi
done

echo ""
echo "📚 Checking documentation..."
for file in "README.md" "ARCHITECTURE.md" "QUICKSTART.md" "CONTRIBUTING.md" "CHANGELOG.md" "LICENSE" "PROJECT_SETUP.md"; do
    files_total=$((files_total + 1))
    if check_file "$file"; then
        files_ok=$((files_ok + 1))
    fi
done

echo ""
echo "📊 Verification Summary"
echo "======================"
echo -e "Files: ${GREEN}${files_ok}/${files_total}${NC}"
echo -e "Dirs:  ${GREEN}${dirs_ok}/${dirs_total}${NC}"

if [ $files_ok -eq $files_total ] && [ $dirs_ok -eq $dirs_total ]; then
    echo -e "\n${GREEN}✓ All files and directories are in place!${NC}\n"
    echo "Next steps:"
    echo "1. npm install"
    echo "2. cd src-tauri && cargo build && cd .."
    echo "3. npm run dev"
    exit 0
else
    echo -e "\n${YELLOW}⚠ Some files or directories are missing.${NC}\n"
    echo "Please check the output above."
    exit 1
fi
