use std::fs;
use std::fs::File;
use std::io::Read; // 'self' supprimé pour éviter le warning
use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

pub struct AudioFileProcessor;

impl AudioFileProcessor {
    /// Scan directory recursively for audio files
    pub fn find_audio_files(source_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut audio_files = Vec::new();
        let audio_extensions = ["flac", "wav", "mp3", "aac", "ogg"];

        for entry in WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if audio_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            audio_files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }

        Ok(audio_files)
    }

    /// Calcule une empreinte numérique (Hash) unique pour le fichier.
    /// Fix E0277 : Utilise hex::encode pour transformer le hash en String.
    pub fn calculate_hash(path: &Path) -> Result<String> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024 * 1024]; // Buffer de 1MB

        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 { break; }
            hasher.update(&buffer[..count]);
        }

        let hash_result = hasher.finalize();
        // Utilisation de la crate hex pour convertir les octets en texte
        Ok(hex::encode(hash_result))
    }

    /// Copy and rename audio file to destination
    pub fn copy_and_rename(
        source: &Path,
        destination_dir: &Path,
        genre_folder: &str,
        new_name: &str,
    ) -> Result<PathBuf> {
        let genre_path = destination_dir.join(genre_folder);
        fs::create_dir_all(&genre_path)?;

        let file_extension = source
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("audio");

        let destination_file = genre_path.join(format!("{}.{}", new_name, file_extension));

        if destination_file.exists() {
            return Err(anyhow::anyhow!("File already exists at destination"));
        }

        fs::copy(source, &destination_file)?;

        Ok(destination_file)
    }

    /// Generate sanitized filename
    pub fn sanitize_filename(title: &str, artist: &str, _genre: &str) -> String {
        let sanitize = |s: &str| {
            s.chars()
                .map(|c| match c {
                    '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                    _ => c,
                })
                .collect::<String>()
        };

        format!(
            "{} - {}",
            sanitize(artist),
            sanitize(title)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        let result = AudioFileProcessor::sanitize_filename(
            "Song: Title",
            "Artist/Name",
            "Electronic*Music",
        );
        assert_eq!(result, "Artist_Name - Song_ Title");
    }
}