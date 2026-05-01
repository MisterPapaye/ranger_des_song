# Quick Start Guide - Ranger de Song

## Prerequisites

Before you start, ensure you have:

- **Rust 1.70+**: https://rustup.rs/
- **Node.js 16+**: https://nodejs.org/
- **Git**: For version control
- **npm**: Comes with Node.js

### Linux Only

```bash
# For Ubuntu/Debian
sudo apt-get install libssl-dev libgtk-3-dev libayatana-appindicator3-dev

# For Fedora
sudo dnf install openssl-devel gtk3-devel libappindicator-gtk3-devel
```

## Installation (5 minutes)

```bash
# 1. Clone or navigate to the project
cd ranger_de_song

# 2. Install Node dependencies
npm install

# 3. Build Rust backend
cd src-tauri
cargo build --release
cd ..

# Done! Ready to run
```

## Running in Development

```bash
npm run dev
```

This will:
- Start Vite dev server (frontend hot reload)
- Build Rust in debug mode
- Launch Tauri app with fast updates
- Open dev console with Ctrl+Shift+I

## Building for Production

```bash
npm run build
```

Output:
- **Windows**: `dist/ranger_de_song_0.1.0_x64_en-US.msi`
- **macOS**: `dist/Ranger de Song.app`
- **Linux**: `dist/ranger-de-song_0.1.0_amd64.AppImage`

## First Run

1. Launch the app
2. Click "Browse" next to "Source Music Folder"
3. Select a folder with your music files
4. Click "Browse" next to "Destination Folder"
5. Select where you want organized music
6. Click "Start Organization"
7. Watch the progress bar ✨

## Project Structure TL;DR

```
├── src/                    # React frontend (TypeScript)
├── src-tauri/             # Rust backend
│   └── src/
│       ├── audio/         # Audio file handling
│       ├── api/           # MusicBrainz integration
│       └── commands.rs    # Tauri bridge
├── package.json           # Node config
└── Cargo.toml             # Rust config
```

## Useful Commands

```bash
# Development
npm run dev          # Start dev server with hot reload
npm run type-check   # Check TypeScript types

# Build
npm run build        # Production build

# Cargo (Rust only)
cd src-tauri && cargo test       # Run tests
cd src-tauri && cargo build --release  # Optimized build
cd src-tauri && cargo clean      # Clean build artifacts
```

## Troubleshooting

### "npm: command not found"
→ Install Node.js from https://nodejs.org/

### "cargo: command not found"
→ Install Rust from https://rustup.rs/

### "Failed to build Rust"
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cd src-tauri
cargo clean
cargo build
cd ..
```

### "Cannot find module @tauri-apps/api"
```bash
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### "EACCES: permission denied"
```bash
# Fix npm permissions (Linux/macOS)
sudo chown -R $(whoami) ~/.npm
npm install
```

## Performance Tips

- Organize smaller library first (<1000 songs) to test
- Ensure good internet for MusicBrainz lookups
- Use FLAC files with complete tags for fastest processing
- Copy destination to fast SSD if possible

## Next Steps

- Read [ARCHITECTURE.md](./ARCHITECTURE.md) for development details
- See [README.md](./README.md) for full feature list
- Check `.gitignore` for what's tracked

## Support

Having issues?

1. Check if MusicBrainz API is online: https://musicbrainz.org/
2. Verify file permissions for source and destination
3. Check disk space on destination drive
4. See troubleshooting section above

---

Happy organizing! 🎵🎧
