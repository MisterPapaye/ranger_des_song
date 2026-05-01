# Installation & Development Guide

Welcome to **Ranger de Song** - Your DJ Music Library Organizer!

This guide will walk you through installation and first steps.

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **OS** | Windows 10 / macOS 10.13 / Ubuntu 18.04 | Latest version |
| **RAM** | 4 GB | 8 GB or more |
| **Disk** | 500 MB | 2 GB |
| **Rust** | 1.70 | Latest stable |
| **Node.js** | 16 LTS | 18+ |

## Step 1: Install Prerequisites

### Windows

1. Install Rust: https://rustup.rs/
   - Run the installer
   - Defaults are fine
   
2. Install Node.js: https://nodejs.org/
   - Download LTS version
   - Run installer with default settings
   
3. Verify installation:
   ```bash
   rustc --version
   cargo --version
   node --version
   npm --version
   ```

### macOS

```bash
# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install rust node

# Verify
rustc --version
node --version
```

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install additional dependencies
sudo apt-get install libssl-dev libgtk-3-dev libayatana-appindicator3-dev

# Verify
rustc --version
node --version
```

### Linux (Fedora)

```bash
sudo dnf install openssl-devel gtk3-devel libappindicator-gtk3-devel
# Then follow Rust/Node installation steps above
```

## Step 2: Clone/Setup Project

```bash
# Navigate to project directory
cd /path/to/ranger_de_song

# Run verification
bash verify.sh

# If verification passes, proceed to Step 3
```

## Step 3: Install Dependencies

### Option A: Automatic Setup (macOS/Linux)

```bash
chmod +x setup.sh
./setup.sh
```

### Option B: Manual Setup

```bash
# Install Node dependencies
npm install

# Navigate to Rust backend
cd src-tauri

# Build Rust backend
cargo build

# Return to project root
cd ..
```

## Step 4: Start Development

```bash
# From project root
npm run dev
```

This command will:
1. Start Vite dev server on port 5173
2. Build Rust backend
3. Launch Tauri app with dev tools
4. Hot reload on file changes

### Dev Tools

Once app is running:
- **Inspect elements**: Right-click → Inspect
- **Console**: Ctrl+Shift+I (or Cmd+Option+I on macOS)
- **Network**: Check API calls and performance

## Step 5: First Run

1. **Source folder**:
   - Click "Browse" 
   - Select folder with music files
   - Examples: `~/Music`, `~/Downloads/Music`

2. **Destination folder**:
   - Click "Browse"
   - Select where to organize music
   - Can be on same or different drive

3. **Start organization**:
   - Click "Start Organization"
   - Watch progress bar
   - Results show when complete

### Expected Output

Files will be organized like:
```
destination/
├── Electronic/
│   ├── Song One - Artist A - Electronic.flac
│   └── Song Two - Artist B - Electronic.flac
├── House/
│   └── Song Three - Artist C - House.wav
└── Techno/
    └── Song Four - Artist D - Techno.mp3
```

## Building for Distribution

### Create Release Build

```bash
npm run build
```

### Output Locations

- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

### Installer Files

| Platform | File | Type |
|----------|------|------|
| Windows | `ranger_de_song_0.1.0_x64_en-US.msi` | Installer |
| macOS | `Ranger de Song.dmg` | Disk image |
| Linux | `ranger-de-song_0.1.0_amd64.AppImage` | Executable |

## Troubleshooting

### Problem: "npm: command not found"

**Solution**: Node.js not installed
```bash
# Verify Node.js
node --version

# If not installed, install from https://nodejs.org/
```

### Problem: "cargo: command not found"

**Solution**: Rust not installed
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Problem: Build fails with permission errors

**Solution**: Grant execute permission
```bash
chmod +x setup.sh
chmod +x verify.sh
```

### Problem: "Cannot compile tauri"

**Solution**: Update Rust
```bash
rustup update
cd src-tauri && cargo clean && cargo build && cd ..
```

### Problem: MusicBrainz API not responding

**Solution**: Check internet and API status
- Verify internet connection
- Check: https://status.musicbrainz.org/
- Try again in a few minutes

### Problem: File copy fails

**Solution**: Check permissions and space
```bash
# Check destination folder permissions
ls -la /path/to/destination

# Check available disk space
df -h /path/to/destination
```

### Problem: "Module not found" error

**Solution**: Reinstall dependencies
```bash
rm -rf node_modules
npm install
cd src-tauri && cargo clean && cd ..
npm run dev
```

## Performance Tips

1. **Organize smaller batches first** (< 1000 files)
2. **Use FLAC files** - Fastest metadata extraction
3. **Fast SSD destination** - Speeds up file copying
4. **Complete ID3 tags** - Skips MusicBrainz queries
5. **Close other apps** - More resources available

## File Format Support

| Format | Read Tags | Copy | Status |
|--------|-----------|------|--------|
| FLAC | ✅ | ✅ | Full support |
| WAV | ✅ | ✅ | Full support |
| MP3 | ✅ | ✅ | Full support |
| AAC | ✅ | ✅ | Full support |
| OGG | ✅ | ✅ | Full support |
| ALAC | ⚠️ | ✅ | Partial |
| Others | ❌ | ✅ | Copied as-is |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+I` | Dev Tools (Windows/Linux) |
| `Cmd+Option+I` | Dev Tools (macOS) |
| `F12` | Toggle Dev Console |
| `Ctrl+R` | Reload App |

## Memory & Performance

### Typical Usage

| Scenario | Memory | Time |
|----------|--------|------|
| Scan 100 files | ~150 MB | 2-3s |
| Scan 1000 files | ~300 MB | 10-15s |
| Scan 5000 files | ~800 MB | 1-2 min |
| Process 1000 with API | ~500 MB | 5-10 min |

## Environment Variables

### For Development

```bash
# Enable debug logging
RUST_LOG=debug npm run dev

# Disable optimizations
RUST_OPT_LEVEL=0 npm run dev
```

## Getting Help

1. **Check documentation**: See README.md, ARCHITECTURE.md
2. **Review logs**: Check console output
3. **Verify setup**: Run `bash verify.sh`
4. **Check issues**: https://github.com/yourname/ranger_de_song/issues

## Next Steps

- 📖 Read [ARCHITECTURE.md](./ARCHITECTURE.md) for technical deep dive
- 🎨 Customize styling in `src/App.css`
- 🔧 Explore Rust code in `src-tauri/src/`
- 🚀 Build and distribute your creation

## Common Tasks

### Clear Project Cache

```bash
# Full reset
rm -rf node_modules src-tauri/target dist
cargo clean
npm install
npm run dev
```

### Update Dependencies

```bash
# Node
npm update

# Rust
cd src-tauri && cargo update && cd ..
```

### Type Check

```bash
npm run type-check
```

### Format Code

```bash
# Rust
cd src-tauri && cargo fmt && cd ..

# TypeScript would need Prettier (optional setup)
```

---

**Questions?** Check out the documentation files or open an issue.

**Happy organizing!** 🎵🎧
