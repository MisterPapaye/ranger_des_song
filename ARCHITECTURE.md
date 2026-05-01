# Ranger de Song - Architecture & Development Guide

## Project Overview

Ranger de Song is a desktop application for DJs that automatically organizes their music library by genre, with intelligent metadata extraction and enrichment via MusicBrainz API.

## Architecture

### Backend (Rust)

The Rust backend handles all heavy lifting:

#### Core Modules

1. **audio/metadata.rs**
   - `AudioMetadataExtractor`: Parses FLAC, WAV, MP3 tags
   - Falls back to filename parsing if tags are missing
   - Returns structured metadata with completeness flag

2. **audio/processor.rs**
   - `AudioFileProcessor`: File operations
   - `find_audio_files()`: Recursive directory scan
   - `copy_and_rename()`: Safe file operations with genre organization
   - `sanitize_filename()`: Prevent filesystem issues

3. **api/musicbrainz.rs**
   - `MusicBrainzClient`: HTTP client for MusicBrainz API
   - `search_recording()`: Query by title + artist
   - Handles rate limiting and errors gracefully
   - Returns enriched metadata

4. **models/mod.rs**
   - `Track`: Represents a single audio file
   - `OrganizationResult`: Final statistics
   - `ProgressUpdate`: Real-time feedback to UI

5. **commands.rs**
   - `start_organization()`: Main orchestrator
   - Handles async task coordination
   - Emits progress events to frontend

### Frontend (React + TypeScript)

Single-page application built with Tauri WebView.

#### Components

- **App.tsx**: Main component
  - Folder selection UI
  - Progress display
  - Results summary
  - Tauri command invocation

#### Styling

- **App.css**: Dark mode theme
  - Gradient backgrounds
  - Responsive grid layout
  - Smooth animations

#### Tauri Integration

- `@tauri-apps/api` for system integration
- `dialog` API for folder selection
- `window` API for event listeners
- `invoke` for backend commands

## Development Workflow

### Setting Up Development Environment

```bash
# Install Node dependencies
npm install

# Build Rust dependencies
cd src-tauri
cargo build
cd ..

# Start dev server with hot reload
npm run dev
```

### File Structure for New Features

When adding features:

1. **Backend logic** → `src-tauri/src/modules/`
2. **Models** → `src-tauri/src/models/mod.rs`
3. **Tauri commands** → `src-tauri/src/commands.rs`
4. **Frontend UI** → `src/App.tsx` or new components
5. **Styles** → `src/App.css` or component.css files

### Building for Production

```bash
npm run build
```

This:
1. Compiles React + TypeScript
2. Builds Rust in release mode with optimizations
3. Bundles everything with Tauri
4. Creates platform-specific installers

## Key Design Decisions

### 1. Non-destructive Organization

- Files are **copied**, never moved from source
- Source directory remains completely untouched
- Gives users confidence in the process

### 2. Incremental Metadata Enrichment

- Uses local tags first (fastest)
- Falls back to filename parsing
- Only queries MusicBrainz if needed
- Reduces API calls and improves performance

### 3. Reactive Progress Feedback

- Tauri window events emit progress updates
- Frontend listens and updates UI in real-time
- Smooth animations and clear status messaging

### 4. Async Processing

- Tokio runtime for concurrent operations
- Non-blocking UI during heavy workloads
- Graceful error handling at each step

## API Integration - MusicBrainz

### Query Format

The app searches MusicBrainz using:

```
GET https://musicbrainz.org/ws/2/recording?query=recording:"Song Title" artist:"Artist Name"&fmt=json
```

### Response Parsing

```json
{
  "recordings": [
    {
      "title": "Song Title",
      "artist-credit": [
        {
          "artist": {
            "name": "Artist Name"
          }
        }
      ]
    }
  ]
}
```

### Rate Limiting

- 1 request per second maximum
- User-Agent header is required
- Set in `musicbrainz.rs` as: `RangerDeSong/0.1.0`

## Performance Considerations

### Bottlenecks

1. **File I/O**: Copying large files
   - Mitigate: Async file operations, progress updates

2. **API latency**: MusicBrainz queries
   - Mitigate: Cache results, batch requests where possible

3. **Metadata parsing**: Large tag structures
   - Mitigate: Use native libraries (metaflac), avoid regex

### Optimization Tips

- Use `--release` build for production
- Enable LTO (Link Time Optimization) in Cargo
- Consider caching MusicBrainz results
- Profile with `cargo flamegraph`

## Testing

### Unit Tests

Run Rust tests:

```bash
cd src-tauri
cargo test
```

Existing tests cover:
- Filename parsing logic
- Filename sanitization

### Integration Testing

Manual testing required for:
- File operations (permissions, disk space)
- MusicBrainz API integration
- UI responsiveness during processing

### Test Data

Create sample FLAC files with metaflac:

```bash
metaflac --set-tag="TITLE=Test Song" --set-tag="ARTIST=Test Artist" test.flac
```

## Common Tasks

### Add a New Audio Format

1. Update `audio/processor.rs`:
   ```rust
   let audio_extensions = ["flac", "wav", "mp3", "aac", "ogg", "opus"];
   ```

2. Add parsing to `audio/metadata.rs` if needed

3. Test with sample files

### Customize Rename Pattern

Edit `AudioFileProcessor::sanitize_filename()` in `audio/processor.rs`:

```rust
// Current format: "Title - Artist - Genre"
// To change: Modify the format! macro
```

### Adjust UI Colors

Edit `src/App.css`:

```css
/* Primary gradient */
background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

/* Dark background */
background-color: #0a0e27;
```

### Add New Tauri Command

1. Create function in `src-tauri/src/commands.rs`:

```rust
#[tauri::command]
pub fn my_command() -> String {
    "result".to_string()
}
```

2. Register in `src-tauri/src/lib.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
    // ... other commands
])
```

3. Call from React:

```typescript
const result = await invoke("my_command");
```

## Debugging

### Rust Debug Output

Enable logging:

```rust
env_logger::init();
log::info!("Message: {}", value);
```

### Tauri Console

Frontend errors appear in Tauri dev console (Ctrl+Shift+I).

### Common Issues

1. **"Source folder does not exist"**
   - Verify path exists and is accessible
   - Check file permissions

2. **MusicBrainz API timeouts**
   - Check internet connection
   - Verify API is online: https://musicbrainz.org/

3. **File copy failures**
   - Verify destination has write permissions
   - Check disk space
   - Ensure no file locks

## Dependencies Management

### Critical Dependencies

- **tauri**: UI framework and system integration
- **tokio**: Async runtime
- **metaflac**: FLAC metadata parsing
- **reqwest**: HTTP client for MusicBrainz

### Updating Dependencies

```bash
# Check for updates
cargo update

# Update specific package
cargo update -p tauri
```

## Deployment

### Release Build

```bash
npm run build
```

Creates:
- Windows: `.msi` installer in `src-tauri/target/release/bundle/msi/`
- macOS: `.app` bundle and `.dmg` in `src-tauri/target/release/bundle/macos/`
- Linux: `.AppImage` and `.deb` in `src-tauri/target/release/bundle/appimage/`

### Code Signing (Production)

Requires platform-specific certificates:
- Windows: Code signing certificate
- macOS: Apple Developer ID certificate
- Linux: GPG key (optional)

See Tauri docs for detailed signing instructions.

## Future Enhancements

Priority list for improvements:

1. **Caching Layer**: Redis/RocksDB for MusicBrainz results
2. **Batch Operations**: Process multiple folders
3. **Undo/Rollback**: Track operations, allow reversal
4. **Album Art**: Extract and organize cover images
5. **Custom Patterns**: User-defined rename formats
6. **Playlists**: Auto-generate playlists by genre

---

For more information, see README.md
