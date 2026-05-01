#!/bin/bash

# Verification script for Tauri 2.x compatible build on Fedora 43

echo "🔍 Ranger de Song - Tauri 2.x Compatibility Check"
echo "===================================================="
echo ""

check_file() {
    if grep -q "$2" "$1"; then
        echo "✅ $1: Found '$2'"
        return 0
    else
        echo "❌ $1: NOT found '$2'"
        return 1
    fi
}

check_not_contain() {
    if ! grep -q "$2" "$1"; then
        echo "✅ $1: No old '$2' found"
        return 0
    else
        echo "❌ $1: Still contains old '$2'"
        return 1
    fi
}

echo "📋 Checking Cargo.toml..."
check_file "src-tauri/Cargo.toml" 'tauri = { version = "2.6"'
check_file "src-tauri/Cargo.toml" 'tauri-build = { version = "2.6"'
check_not_contain "src-tauri/Cargo.toml" 'tauri = { version = "1.5"'
echo ""

echo "📋 Checking package.json..."
check_file "package.json" '"@tauri-apps/api": "^2.6.0"'
check_file "package.json" '"@tauri-apps/cli": "^2.6.0"'
check_not_contain "package.json" '"@tauri-apps/api": "^1.5.0"'
echo ""

echo "📋 Checking Rust source files..."
check_file "src-tauri/src/lib.rs" "#\[cfg_attr(mobile, tauri::mobile_entry_point)\]"
check_file "src-tauri/src/commands.rs" "pub async fn start_organization"
check_file "src-tauri/src/api/musicbrainz.rs" "MusicBrainzClient"
echo ""

echo "📋 Checking configuration..."
check_file "src-tauri/tauri.conf.json" '"productName": "Ranger de Song"'
echo ""

echo "📚 Checking documentation..."
test -f "FEDORA43_BUILD.md" && echo "✅ FEDORA43_BUILD.md exists" || echo "❌ FEDORA43_BUILD.md missing"
test -f "CHANGELOG_TAURI2_UPDATE.md" && echo "✅ CHANGELOG_TAURI2_UPDATE.md exists" || echo "❌ CHANGELOG_TAURI2_UPDATE.md missing"
test -f "build-fedora43.sh" && echo "✅ build-fedora43.sh exists" || echo "❌ build-fedora43.sh missing"
echo ""

echo "✅ Verification complete!"
echo ""
echo "Next steps:"
echo "1. sudo dnf install webkit2gtk4.1-devel libsoup3-devel glib2-devel openssl-devel"
echo "2. ./build-fedora43.sh"
echo "3. npm run dev"
