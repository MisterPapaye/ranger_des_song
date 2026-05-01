use std::path::Path;
use anyhow::Result;
use metaflac::Tag as FlacTag;

#[derive(Debug, Clone)]
pub struct AudioMetadata {
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub has_complete_tags: bool,
}

pub struct AudioMetadataExtractor;

impl AudioMetadataExtractor {
    /// Extract metadata from audio file
    /// Returns (title, artist, genre, is_complete)
    pub fn extract(path: &Path) -> Result<AudioMetadata> {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Try to extract from FLAC tags
        if let Ok(flac_meta) = Self::extract_flac(path) {
            if !flac_meta.title.is_empty() && !flac_meta.artist.is_empty() {
                return Ok(AudioMetadata {
                    has_complete_tags: true,
                    ..flac_meta
                });
            }
        }

        // Fallback: use filename as source
        let (title, artist) = Self::parse_filename(&file_name);
        Ok(AudioMetadata {
            title,
            artist,
            genre: String::from("Unknown"),
            has_complete_tags: false,
        })
    }

    fn extract_flac(path: &Path) -> Result<AudioMetadata> {
        let tag = FlacTag::read_from_path(path)?;
        let vorbis = tag.vorbis_comments().ok_or_else(|| {
            anyhow::anyhow!("No vorbis comments found")
        })?;

        let title = vorbis
            .title()
            .and_then(|iter| iter.iter().next().map(|s| s.to_string()))
            .unwrap_or_default();

        let artist = vorbis
            .artist()
            .and_then(|iter| iter.iter().next().map(|s| s.to_string()))
            .unwrap_or_default();

        let genre = vorbis
            .genre()
            .and_then(|iter| iter.iter().next().map(|s| s.to_string()))
            .unwrap_or_else(|| String::from("Unknown"));

        Ok(AudioMetadata {
            title,
            artist,
            genre,
            has_complete_tags: false,
        })
    }

    /// Parse filename to extract title and artist
    /// Handles formats like: "Title - Artist.ext" or "Title.ext"
    fn parse_filename(filename: &str) -> (String, String) {
        let name_without_ext = filename
            .split('.')
            .next()
            .unwrap_or(filename)
            .to_string();

        if let Some(idx) = name_without_ext.find(" - ") {
            let title = name_without_ext[..idx].trim().to_string();
            let artist = name_without_ext[idx + 3..].trim().to_string();
            (title, artist)
        } else {
            (name_without_ext, String::from("Unknown Artist"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename() {
        let (title, artist) = AudioMetadataExtractor::parse_filename("Song Title - Artist Name.flac");
        assert_eq!(title, "Song Title");
        assert_eq!(artist, "Artist Name");
    }

    #[test]
    fn test_parse_filename_single() {
        let (title, artist) = AudioMetadataExtractor::parse_filename("Song Title.flac");
        assert_eq!(title, "Song Title");
        assert_eq!(artist, "Unknown Artist");
    }
}
