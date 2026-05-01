# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-XX

### Added

- Initial release
- Audio file scanning and metadata extraction
- Support for FLAC, WAV, MP3, AAC, OGG formats
- MusicBrainz API integration for metadata enrichment
- Dark mode UI optimized for DJs
- Real-time progress tracking
- Non-destructive file organization
- Automatic genre-based folder structure
- Standardized file renaming
- Cross-platform support (Windows, macOS, Linux)

### Features

- **Metadata Extraction**
  - FLAC tag reading via metaflac
  - WAV metadata support
  - MP3 ID3 tag parsing
  - Filename-based fallback parsing

- **API Integration**
  - MusicBrainz recording search
  - Artist credit extraction
  - Rate-limited requests
  - Graceful error handling

- **UI Components**
  - Folder selection with native dialogs
  - Progress bar with percentage
  - Live file processing status
  - Results summary with statistics
  - Responsive design

- **File Operations**
  - Recursive directory scanning
  - Safe file copying (non-destructive)
  - Automatic genre folder creation
  - Filename sanitization
  - Duplicate handling

### Performance

- Async processing with Tokio
- Fast metadata parsing with native libraries
- Efficient file I/O operations
- Responsive UI with real-time updates

## [Unreleased]

### Planned Features

- [ ] Database caching for MusicBrainz results
- [ ] Batch API requests optimization
- [ ] Album art extraction and organization
- [ ] Custom renaming patterns
- [ ] Playlist generation
- [ ] Undo/rollback functionality
- [ ] Duplicate detection
- [ ] Multi-language support
- [ ] Tag editing interface
- [ ] Performance metrics dashboard

### Improvements

- [ ] Enhanced error messages
- [ ] Configuration file support
- [ ] Advanced filtering options
- [ ] Keyboard shortcuts
- [ ] Dark/light theme toggle
